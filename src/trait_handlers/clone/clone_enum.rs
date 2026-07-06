use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Field, Fields, Meta, Path, Type, Variant, punctuated::Punctuated};

use super::models::{FieldAttribute, FieldAttributeBuilder, TypeAttributeBuilder};
// Only the bitwise-copy fast path, gated on the `Copy` trait, needs to inspect whether a field uses a type parameter.
#[cfg(feature = "Copy")]
use crate::common::r#type::type_uses_type_params;
use crate::{
    TraitHandler,
    common::{bound::BOUND_EXCEPTIONS_CLONE, where_predicates_bool::WherePredicates},
    supported_traits::Trait,
    trait_handlers::TraitHandlerContext,
};

/// Generates the `Clone` implementation for an enum.
pub(crate) struct CloneEnumHandler;

impl TraitHandler for CloneEnumHandler {
    #[inline]
    fn trait_meta_handler(
        ast: &DeriveInput,
        ctx: &mut TraitHandlerContext,
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
        let mut mark_fields: Vec<(&Type, Path)> = Vec::new();

        let mut clone_token_stream = proc_macro2::TokenStream::new();
        let mut clone_from_token_stream = proc_macro2::TokenStream::new();

        if let Data::Enum(data) = &ast.data {
            type Variants<'a> = Vec<(&'a Variant, Vec<(&'a Field, FieldAttribute)>)>;

            let mut variants: Variants = Vec::new();

            #[cfg(feature = "Copy")]
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

                    #[cfg(feature = "Copy")]
                    if field_attribute.method.is_some() {
                        has_custom_clone_method = true;
                    }

                    variant_fields.push((field, field_attribute));
                }

                variants.push((variant, variant_fields));
            }

            // Like the built-in derives, `clone` can be a plain bitwise copy only when `Copy` is derived together, no generic type parameter is involved, and no field uses a custom clone method.
            // When generic type parameters are involved, a field-wise clone keeps the `Clone` impl usable for type arguments that are `Clone` but not `Copy`.
            #[cfg(feature = "Copy")]
            let use_bitwise_copy = !has_custom_clone_method
                && traits.contains(&Trait::Copy)
                && !data.variants.iter().any(|variant| {
                    variant
                        .fields
                        .iter()
                        .any(|field| type_uses_type_params(&field.ty, &ast.generics.params))
                });

            #[cfg(not(feature = "Copy"))]
            let use_bitwise_copy = false;

            if use_bitwise_copy {
                clone_token_stream.extend(quote!(*self));
            }

            let mut clone_types: Vec<&Type> = Vec::new();

            if variants.is_empty() {
                if !use_bitwise_copy {
                    clone_token_stream.extend(quote!(unreachable!()));
                    clone_from_token_stream.extend(quote!(let _ = source;));
                }
            } else {
                let mut clone_variants_token_stream = proc_macro2::TokenStream::new();
                let mut clone_from_variants_token_stream = proc_macro2::TokenStream::new();

                for (variant, variant_fields) in variants {
                    let variant_ident = &variant.ident;

                    match &variant.fields {
                        Fields::Unit => {
                            clone_variants_token_stream.extend(quote! {
                                Self::#variant_ident => Self::#variant_ident,
                            });
                            clone_from_variants_token_stream.extend(quote! {
                                Self::#variant_ident => {
                                    if let Self::#variant_ident = source {
                                        // same
                                    } else {
                                        *self = ::core::clone::Clone::clone(source);
                                    }
                                },
                            });
                        },
                        Fields::Named(_) => {
                            let mut pattern_src_token_stream = proc_macro2::TokenStream::new();
                            let mut pattern_dst_token_stream = proc_macro2::TokenStream::new();
                            let mut cl_fields_token_stream = proc_macro2::TokenStream::new();
                            let mut cf_body_token_stream = proc_macro2::TokenStream::new();

                            for (field, field_attribute) in variant_fields {
                                let field_name_real = field.ident.as_ref().unwrap();
                                let field_name_src = format_ident!("_s_{}", field_name_real);
                                let field_name_dst = format_ident!("_d_{}", field_name_real);

                                pattern_src_token_stream
                                    .extend(quote!(#field_name_real: #field_name_src,));
                                pattern_dst_token_stream
                                    .extend(quote!(#field_name_real: #field_name_dst,));

                                if let Some(clone) = field_attribute.method.as_ref() {
                                    mark_fields.push((&field.ty, clone.clone()));

                                    cl_fields_token_stream.extend(quote! {
                                        #field_name_real: #clone(#field_name_src),
                                    });
                                    cf_body_token_stream.extend(
                                        quote!(*#field_name_dst = #clone(#field_name_src);),
                                    );
                                } else {
                                    clone_types.push(&field.ty);

                                    cl_fields_token_stream.extend(quote! {
                                        #field_name_real: ::core::clone::Clone::clone(#field_name_src),
                                    });
                                    cf_body_token_stream.extend(
                                        quote!( ::core::clone::Clone::clone_from(#field_name_dst, #field_name_src); ),
                                    );
                                }
                            }

                            clone_variants_token_stream.extend(quote! {
                                    Self::#variant_ident { #pattern_src_token_stream } => Self::#variant_ident { #cl_fields_token_stream },
                                });

                            clone_from_variants_token_stream.extend(quote! {
                                    Self::#variant_ident { #pattern_dst_token_stream } => {
                                        if let Self::#variant_ident { #pattern_src_token_stream } = source {
                                            #cf_body_token_stream
                                        } else {
                                            *self = ::core::clone::Clone::clone(source);
                                        }
                                    },
                                });
                        },
                        Fields::Unnamed(_) => {
                            let mut pattern_token_stream = proc_macro2::TokenStream::new();
                            let mut pattern2_token_stream = proc_macro2::TokenStream::new();
                            let mut fields_token_stream = proc_macro2::TokenStream::new();
                            let mut body_token_stream = proc_macro2::TokenStream::new();

                            for (index, (field, field_attribute)) in
                                variant_fields.into_iter().enumerate()
                            {
                                let field_name_src = format_ident!("_{}", index);

                                pattern_token_stream.extend(quote!(#field_name_src,));

                                let field_name_dst = format_ident!("_{}", field_name_src);

                                pattern2_token_stream.extend(quote!(#field_name_dst,));

                                if let Some(clone) = field_attribute.method.as_ref() {
                                    mark_fields.push((&field.ty, clone.clone()));

                                    fields_token_stream.extend(quote! (#clone(#field_name_src),));
                                    body_token_stream.extend(
                                        quote!(*#field_name_src = #clone(#field_name_dst);),
                                    );
                                } else {
                                    clone_types.push(&field.ty);

                                    fields_token_stream.extend(
                                        quote! ( ::core::clone::Clone::clone(#field_name_src), ),
                                    );
                                    body_token_stream.extend(
                                        quote!( ::core::clone::Clone::clone_from(#field_name_src, #field_name_dst); ),
                                    );
                                }
                            }

                            clone_variants_token_stream.extend(quote! {
                                    Self::#variant_ident ( #pattern_token_stream ) => Self::#variant_ident ( #fields_token_stream ),
                                });

                            clone_from_variants_token_stream.extend(quote! {
                                    Self::#variant_ident ( #pattern_token_stream ) => {
                                        if let Self::#variant_ident ( #pattern2_token_stream ) = source {
                                            #body_token_stream
                                        } else {
                                            *self = ::core::clone::Clone::clone(source);
                                        }
                                    },
                                });
                        },
                    }
                }

                if !use_bitwise_copy {
                    clone_token_stream.extend(quote! {
                        match self {
                            #clone_variants_token_stream
                        }
                    });

                    clone_from_token_stream.extend(quote! {
                        match self {
                            #clone_from_variants_token_stream
                        }
                    });
                }
            }

            // The bound trait is always `Clone`; the `Copy` impl is emitted by the `Copy` handler with its own bounds.
            bound = type_attribute.bound.into_where_predicates_by_generic_parameters_check_types(
                &ast.generics.params,
                &syn::parse2(quote!(::core::clone::Clone)).unwrap(),
                &clone_types,
                &ast.ident,
                &BOUND_EXCEPTIONS_CLONE,
            );

            ctx.record(Trait::Clone, &bound);
        }

        let clone_from_fn_token_stream = if clone_from_token_stream.is_empty() {
            None
        } else {
            Some(quote! {
                #[inline]
                fn clone_from(&mut self, source: &Self) {
                    #clone_from_token_stream
                }
            })
        };

        let ident = &ast.ident;

        let mut generics = ast.generics.clone();

        let where_clause = generics.make_where_clause();

        for where_predicate in bound {
            where_clause.predicates.push(where_predicate);
        }

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        token_stream.extend(quote! {
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
            token_stream.extend(super::create_mark_method_used(&generics, field_ty, method));
        }

        Ok(())
    }
}
