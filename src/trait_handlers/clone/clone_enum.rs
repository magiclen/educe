use syn::{
    Data, DeriveInput, ExprPath, Field, Fields, Meta, Type, Variant, punctuated::Punctuated,
};

use super::models::{FieldAttribute, FieldAttributeBuilder, TypeAttributeBuilder};
use crate::{
    TraitHandler,
    common::{
        bound::BOUND_EXCEPTIONS_CLONE, ident_index::IdentOrIndex, quote_mixed,
        where_predicates_bool::WherePredicates,
    },
    supported_traits::Trait,
    trait_handlers::TraitHandlerContext,
};

/// Generates the `Clone` implementation for an enum.
pub(crate) struct CloneEnumHandler;

impl TraitHandler for CloneEnumHandler {
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
        .build_from_clone_meta(meta)?;

        let mut bound: WherePredicates = Punctuated::new();

        // Custom clone methods are referenced only inside the derived impl body, which dead-code analysis skips, so each one is collected here and later re-referenced by a marker item.
        let mut mark_fields: Vec<(&Type, ExprPath)> = Vec::new();

        let mut clone_token_stream = proc_macro2::TokenStream::new();
        let mut clone_from_token_stream = proc_macro2::TokenStream::new();

        if let Data::Enum(data) = &ast.data {
            type Variants<'a> = Vec<(&'a Variant, Vec<(&'a Field, FieldAttribute)>)>;

            let mut variants: Variants = Vec::new();

            let mut has_custom_clone_method = false;

            for variant in data.variants.iter() {
                let _ = TypeAttributeBuilder {
                    enable_flag: false, enable_bound: false
                }
                .build_from_attributes(&variant.attrs, traits)?;

                let mut variant_fields: Vec<(&Field, FieldAttribute)> = Vec::new();

                for field in variant.fields.iter() {
                    let field_attribute = FieldAttributeBuilder {
                        enable_method: true
                    }
                    .build_from_attributes(&field.attrs, traits)?;

                    if field_attribute.method.is_some() {
                        has_custom_clone_method = true;
                    }

                    variant_fields.push((field, field_attribute));
                }

                variants.push((variant, variant_fields));
            }

            let use_bitwise_copy =
                super::can_use_bitwise_copy(ast, ctx, traits, has_custom_clone_method)?;

            let mut clone_types: Vec<&Type> = Vec::new();

            if use_bitwise_copy {
                // A whole-value copy needs no field-wise body or field bounds.
                clone_token_stream.extend(quote_mixed!(*self));
            } else if variants.is_empty() {
                clone_token_stream.extend(quote_mixed!(::core::unreachable!()));
                clone_from_token_stream.extend(quote_mixed!(let _ = source;));
            } else {
                let mut clone_variants_token_stream = proc_macro2::TokenStream::new();
                let mut clone_from_variants_token_stream = proc_macro2::TokenStream::new();

                for (variant, variant_fields) in variants {
                    let variant_ident = &variant.ident;

                    if let Fields::Unit = &variant.fields {
                        clone_variants_token_stream.extend(quote_mixed! {
                            Self::#variant_ident => Self::#variant_ident,
                        });
                        clone_from_variants_token_stream.extend(quote_mixed! {
                            Self::#variant_ident => {
                                if let Self::#variant_ident = source {
                                    // same
                                } else {
                                    *self = ::core::clone::Clone::clone(source);
                                }
                            },
                        });

                        continue;
                    }

                    // The bindings are named after their role: the source pattern matches `self` in `clone` and `source` in `clone_from`, while the destination pattern only matches `self` in `clone_from`.
                    let mut pattern_src_token_stream = proc_macro2::TokenStream::new();
                    let mut pattern_dst_token_stream = proc_macro2::TokenStream::new();
                    let mut fields_token_stream = proc_macro2::TokenStream::new();
                    let mut body_token_stream = proc_macro2::TokenStream::new();

                    for (index, (field, field_attribute)) in variant_fields.into_iter().enumerate()
                    {
                        let field_name =
                            IdentOrIndex::from_ident_with_index(field.ident.as_ref(), index);
                        let field_name_src = field_name.to_binding("_s_");
                        let field_name_dst = field_name.to_binding("_d_");

                        pattern_src_token_stream
                            .extend(field_name.to_field(&quote_mixed!(#field_name_src)));
                        pattern_dst_token_stream
                            .extend(field_name.to_field(&quote_mixed!(#field_name_dst)));

                        let value = if let Some(clone) = field_attribute.method.as_ref() {
                            mark_fields.push((&field.ty, clone.clone()));

                            body_token_stream
                                .extend(quote_mixed!(*#field_name_dst = #clone(#field_name_src);));

                            quote_mixed!(#clone(#field_name_src))
                        } else {
                            clone_types.push(&field.ty);

                            body_token_stream.extend(
                                quote_mixed!( ::core::clone::Clone::clone_from(#field_name_dst, #field_name_src); ),
                            );

                            quote_mixed!(::core::clone::Clone::clone(#field_name_src))
                        };

                        fields_token_stream.extend(field_name.to_field(&value));
                    }

                    let (pattern_src, pattern_dst, construction) =
                        if let Fields::Named(_) = &variant.fields {
                            (
                                quote_mixed!(Self::#variant_ident { #pattern_src_token_stream }),
                                quote_mixed!(Self::#variant_ident { #pattern_dst_token_stream }),
                                quote_mixed!(Self::#variant_ident { #fields_token_stream }),
                            )
                        } else {
                            (
                                quote_mixed!(Self::#variant_ident ( #pattern_src_token_stream )),
                                quote_mixed!(Self::#variant_ident ( #pattern_dst_token_stream )),
                                quote_mixed!(Self::#variant_ident ( #fields_token_stream )),
                            )
                        };

                    clone_variants_token_stream.extend(quote_mixed! {
                        #pattern_src => #construction,
                    });

                    clone_from_variants_token_stream.extend(quote_mixed! {
                        #pattern_dst => {
                            if let #pattern_src = source {
                                #body_token_stream
                            } else {
                                *self = ::core::clone::Clone::clone(source);
                            }
                        },
                    });
                }

                clone_token_stream.extend(quote_mixed! {
                    match self {
                        #clone_variants_token_stream
                    }
                });

                clone_from_token_stream.extend(quote_mixed! {
                    match self {
                        #clone_from_variants_token_stream
                    }
                });
            }

            // The bound trait is always `Clone`; the `Copy` impl is emitted by the `Copy` handler with its own bounds.
            bound = type_attribute.bound.into_where_predicates_by_generic_parameters_check_types(
                &ast.generics.params,
                &syn::parse2(quote_mixed!(::core::clone::Clone)).unwrap(),
                &clone_types,
                &ast.ident,
                &BOUND_EXCEPTIONS_CLONE,
            );

            ctx.record(Trait::Clone, &bound);
        }

        let clone_from_fn_token_stream = if clone_from_token_stream.is_empty() {
            None
        } else {
            Some(quote_mixed! {
                #[inline]
                fn clone_from(&mut self, source: &Self) {
                    #clone_from_token_stream
                }
            })
        };

        let ident = &ast.ident;

        let generics = crate::common::generics::with_predicates(ast.generics.clone(), bound);

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        token_stream.extend(quote_mixed! {
            #generated_impl_attributes
            impl #impl_generics ::core::clone::Clone for #ident #ty_generics #where_clause {
                #[inline]
                fn clone(&self) -> Self {
                    #clone_token_stream
                }

                #clone_from_fn_token_stream
            }
        });

        for (field_ty, method) in &mark_fields {
            token_stream.extend(super::create_mark_method_used(ast, &generics, field_ty, method));
        }

        Ok(())
    }
}
