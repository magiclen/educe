use syn::{Data, DeriveInput, Fields, Meta, Type};

use super::{
    TraitHandler,
    models::{FieldAttributeBuilder, TypeAttributeBuilder},
};
use crate::{
    Trait,
    common::{bound::BOUND_EXCEPTIONS_EQUALITY, ident_index::IdentOrIndex, quote_mixed},
    trait_handlers::TraitHandlerContext,
};

/// Generates the `PartialEq` implementation for an enum.
pub(crate) struct PartialEqEnumHandler;

impl TraitHandler for PartialEqEnumHandler {
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

        let type_attribute =
            TypeAttributeBuilder {
                enable_flag: true, enable_unsafe: false, enable_bound: true
            }
            .build_from_partial_eq_meta(meta)?;

        let mut partial_eq_types: Vec<&Type> = Vec::new();

        let mut eq_token_stream = proc_macro2::TokenStream::new();

        let mut arms_token_stream = proc_macro2::TokenStream::new();

        if let Data::Enum(data) = &ast.data {
            for variant in data.variants.iter() {
                let _ = TypeAttributeBuilder {
                    enable_flag:   false,
                    enable_unsafe: false,
                    enable_bound:  false,
                }
                .build_from_attributes(&variant.attrs, traits)?;

                let variant_ident = &variant.ident;

                if let Fields::Unit = &variant.fields {
                    arms_token_stream.extend(quote_mixed! {
                        Self::#variant_ident => {
                            if let Self::#variant_ident = other {
                                // The same unit variant on both sides, so there is nothing to compare.
                            } else {
                                return false;
                            }
                        }
                    });

                    continue;
                }

                let mut pattern_self_token_stream = proc_macro2::TokenStream::new();
                let mut pattern_other_token_stream = proc_macro2::TokenStream::new();
                let mut block_token_stream = proc_macro2::TokenStream::new();

                for (index, field) in variant.fields.iter().enumerate() {
                    let field_attribute = FieldAttributeBuilder {
                        enable_ignore: true,
                        enable_method: true,
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

                    if let Some(method) = field_attribute.method {
                        block_token_stream.extend(quote_mixed! {
                            if !#method(#field_name_var_self, #field_name_var_other) {
                                return false;
                            }
                        });
                    } else {
                        partial_eq_types.push(&field.ty);

                        block_token_stream.extend(quote_mixed! {
                            if ::core::cmp::PartialEq::ne(#field_name_var_self, #field_name_var_other) {
                                return false;
                            }
                        });
                    }
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
                        } else {
                            return false;
                        }
                    }
                });
            }
        }

        if !arms_token_stream.is_empty() {
            eq_token_stream.extend(quote_mixed! {
                match self {
                    #arms_token_stream
                }
            });
        }

        // `Eq` compares exactly the same fields, so it reuses this list instead of parsing the field attributes again.
        #[cfg(feature = "Eq")]
        ctx.record_partial_eq_types(&partial_eq_types);

        let ident = &ast.ident;

        let bound = type_attribute.bound.into_where_predicates_by_generic_parameters_check_types(
            &ast.generics.params,
            &syn::parse2(quote_mixed!(::core::cmp::PartialEq)).unwrap(),
            &partial_eq_types,
            &ast.ident,
            &BOUND_EXCEPTIONS_EQUALITY,
        );

        ctx.record(Trait::PartialEq, &bound);

        let generics = crate::common::generics::with_predicates(ast.generics.clone(), bound);

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        token_stream.extend(quote_mixed! {
            #generated_impl_attributes
            impl #impl_generics ::core::cmp::PartialEq for #ident #ty_generics #where_clause {
                #[inline]
                fn eq(&self, other: &Self) -> ::core::primitive::bool {
                    #eq_token_stream

                    true
                }
            }
        });

        Ok(())
    }
}
