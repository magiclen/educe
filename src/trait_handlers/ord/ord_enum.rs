use std::collections::BTreeMap;

use syn::{Data, DeriveInput, ExprPath, Field, Fields, Ident, Meta, Type, spanned::Spanned};

use super::{
    TraitHandler,
    models::{FieldAttribute, FieldAttributeBuilder, TypeAttributeBuilder},
};
use crate::{
    Trait,
    common::{
        bound::{BOUND_EXCEPTIONS_ORDER, Bound},
        ident_index::IdentOrIndex,
        quote_mixed,
        tools::DiscriminantType,
    },
    trait_handlers::TraitHandlerContext,
};

/// Generates the `Ord` implementation for an enum.
pub(crate) struct OrdEnumHandler;

impl TraitHandler for OrdEnumHandler {
    #[inline]
    fn trait_meta_handler<'a>(
        ast: &'a DeriveInput,
        ctx: &mut TraitHandlerContext<'a>,
        token_stream: &mut proc_macro2::TokenStream,
        traits: &[Trait],
        meta: &Meta,
    ) -> syn::Result<()> {
        let generated_impl_attributes =
            crate::common::attributes::generated_impl_attributes(&ast.attrs);

        let type_attribute = TypeAttributeBuilder {
            enable_flag: true, enable_bound: true
        }
        .build_from_ord_meta(meta)?;

        let mut ord_types: Vec<&Type> = Vec::new();

        let mut cmp_token_stream = proc_macro2::TokenStream::new();

        let (discriminant_type, discriminant_values) = DiscriminantType::from_ast(ast)?;

        let mut arms_token_stream = proc_macro2::TokenStream::new();

        // Maps every variant to its discriminant, used to order variants before their fields are compared.
        let mut key_arms_token_stream = proc_macro2::TokenStream::new();

        let mut all_unit = true;

        if let Data::Enum(data) = &ast.data {
            let built_in_cmp: ExprPath = syn::parse2(quote_mixed!(::core::cmp::Ord::cmp)).unwrap();

            for (variant_index, variant) in data.variants.iter().enumerate() {
                let _ = TypeAttributeBuilder {
                    enable_flag: false, enable_bound: false
                }
                .build_from_attributes(&variant.attrs, traits)?;

                let variant_ident = &variant.ident;

                let discriminant = &discriminant_values[variant_index];

                let key_pattern = match &variant.fields {
                    Fields::Unit => quote_mixed!(Self::#variant_ident),
                    Fields::Named(_) => quote_mixed!(Self::#variant_ident { .. }),
                    Fields::Unnamed(_) => quote_mixed!(Self::#variant_ident ( .. )),
                };

                key_arms_token_stream.extend(quote_mixed! {
                    #key_pattern => #discriminant,
                });

                if let Fields::Unit = &variant.fields {
                    arms_token_stream.extend(quote_mixed! {
                        Self::#variant_ident => {
                            return ::core::cmp::Ordering::Equal;
                        }
                    });

                    continue;
                }

                all_unit = false;

                let mut pattern_self_token_stream = proc_macro2::TokenStream::new();
                let mut pattern_other_token_stream = proc_macro2::TokenStream::new();
                let mut block_token_stream = proc_macro2::TokenStream::new();

                let mut fields: BTreeMap<isize, (&Field, Ident, Ident, FieldAttribute)> =
                    BTreeMap::new();

                for (index, field) in variant.fields.iter().enumerate() {
                    let field_attribute = FieldAttributeBuilder {
                        enable_ignore: true,
                        enable_method: true,
                        enable_rank:   true,
                        rank:          isize::MIN + index as isize,
                    }
                    .build_from_attributes(&field.attrs, traits)?;

                    let field_name =
                        IdentOrIndex::from_ident_with_index(field.ident.as_ref(), index);

                    if field_attribute.ignore {
                        let ignored = field_name.to_field(&quote_mixed!(_));

                        pattern_self_token_stream.extend(ignored.clone());
                        pattern_other_token_stream.extend(ignored);

                        continue;
                    }

                    let field_name_var_self = field_name.to_binding("_s_");
                    let field_name_var_other = field_name.to_binding("_o_");

                    pattern_self_token_stream
                        .extend(field_name.to_field(&quote_mixed!(#field_name_var_self)));
                    pattern_other_token_stream
                        .extend(field_name.to_field(&quote_mixed!(#field_name_var_other)));

                    let rank = field_attribute.rank;

                    if fields.contains_key(&rank) {
                        return Err(super::panic::reuse_a_rank(
                            field_attribute.rank_span.unwrap_or_else(|| field.span()),
                            rank,
                        ));
                    }

                    fields.insert(
                        rank,
                        (field, field_name_var_self, field_name_var_other, field_attribute),
                    );
                }

                for (field, field_name_var_self, field_name_var_other, field_attribute) in
                    fields.values()
                {
                    let cmp = field_attribute.method.as_ref().unwrap_or_else(|| {
                        ord_types.push(&field.ty);

                        &built_in_cmp
                    });

                    block_token_stream.extend(quote_mixed! {
                        match #cmp(#field_name_var_self, #field_name_var_other) {
                            ::core::cmp::Ordering::Equal => (),
                            ::core::cmp::Ordering::Greater => return ::core::cmp::Ordering::Greater,
                            ::core::cmp::Ordering::Less => return ::core::cmp::Ordering::Less,
                        }
                    });
                }

                let (pattern_self, pattern_other) = if let Fields::Named(_) = &variant.fields {
                    (
                        quote_mixed!(Self::#variant_ident { #pattern_self_token_stream }),
                        quote_mixed!(Self::#variant_ident { #pattern_other_token_stream }),
                    )
                } else {
                    (
                        quote_mixed!(Self::#variant_ident ( #pattern_self_token_stream )),
                        quote_mixed!(Self::#variant_ident ( #pattern_other_token_stream )),
                    )
                };

                arms_token_stream.extend(quote_mixed! {
                    #pattern_self => {
                        if let #pattern_other = other {
                            #block_token_stream
                        }
                    }
                });
            }
        }

        if arms_token_stream.is_empty() {
            cmp_token_stream.extend(quote_mixed!(::core::cmp::Ordering::Equal));
        } else {
            // Order variants by their discriminant, which is evaluated by the compiler so that no unsafe assumption about the in-memory layout of the enum is needed; this reproduces the ordering of the standard `Ord` derive.
            let discriminant = quote_mixed! {
                let discriminant = |this: &Self| -> #discriminant_type {
                    match this {
                        #key_arms_token_stream
                    }
                };
            };

            cmp_token_stream.extend(if all_unit {
                quote_mixed! {
                    #discriminant

                    ::core::cmp::Ord::cmp(&discriminant(self), &discriminant(other))
                }
            } else {
                quote_mixed! {
                    #discriminant

                    match ::core::cmp::Ord::cmp(&discriminant(self), &discriminant(other)) {
                        ::core::cmp::Ordering::Equal => {
                            match self {
                                #arms_token_stream
                            }

                            ::core::cmp::Ordering::Equal
                        },
                        ::core::cmp::Ordering::Greater => ::core::cmp::Ordering::Greater,
                        ::core::cmp::Ordering::Less => ::core::cmp::Ordering::Less,
                    }
                }
            });
        }

        let ident = &ast.ident;

        let bound_is_auto = matches!(type_attribute.bound, Bound::Auto);

        let mut bound =
            type_attribute.bound.into_where_predicates_by_generic_parameters_check_types(
                &ast.generics.params,
                &syn::parse2(quote_mixed!(::core::cmp::Ord)).unwrap(),
                &ord_types,
                &ast.ident,
                &BOUND_EXCEPTIONS_ORDER,
            );

        if bound_is_auto {
            ctx.inherit_from(super::prerequisites(), &mut bound);
        }

        ctx.record(Trait::Ord, &bound);

        let generics = crate::common::generics::with_predicates(ast.generics.clone(), bound);

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        token_stream.extend(quote_mixed! {
            #generated_impl_attributes
            impl #impl_generics ::core::cmp::Ord for #ident #ty_generics #where_clause {
                #[inline]
                fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                    #cmp_token_stream
                }
            }
        });

        Ok(())
    }
}
