use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Index, Meta, Path, Type, punctuated::Punctuated};

use super::models::{FieldAttribute, FieldAttributeBuilder, TypeAttributeBuilder};
use crate::{
    TraitHandler,
    common::{
        bound::BOUND_EXCEPTIONS_CLONE, r#type::type_uses_type_params,
        where_predicates_bool::WherePredicates,
    },
    supported_traits::Trait,
    trait_handlers::TraitHandlerContext,
};

/// Generates the `Clone` implementation for a struct.
pub(crate) struct CloneStructHandler;

impl TraitHandler for CloneStructHandler {
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

        if let Data::Struct(data) = &ast.data {
            let mut fields: Vec<(&Field, FieldAttribute)> = Vec::new();

            #[cfg(feature = "Copy")]
            let contains_copy = traits.contains(&Trait::Copy);

            #[cfg(not(feature = "Copy"))]
            let contains_copy = false;

            for field in data.fields.iter() {
                let field_attribute = FieldAttributeBuilder {
                    enable_method: true
                }
                .build_from_attributes(&field.attrs, traits)?;

                fields.push((field, field_attribute));
            }

            let has_custom_method =
                fields.iter().any(|(_, field_attribute)| field_attribute.method.is_some());

            let uses_generics = data
                .fields
                .iter()
                .any(|field| type_uses_type_params(&field.ty, &ast.generics.params));

            // Like the built-in derives, `clone` can be a plain bitwise copy only when `Copy` is derived together, no generic type parameter is involved, and no field uses a custom clone method.
            // When generic type parameters are involved, a field-wise clone keeps the `Clone` impl usable for type arguments that are `Clone` but not `Copy`.
            let use_bitwise_copy = contains_copy && !uses_generics && !has_custom_method;

            let mut clone_types: Vec<&Type> = Vec::new();

            match &data.fields {
                Fields::Unit => {
                    if !use_bitwise_copy {
                        clone_token_stream.extend(quote!(Self));
                        clone_from_token_stream.extend(quote!(let _ = source;));
                    }
                },
                Fields::Named(_) => {
                    let mut fields_token_stream = proc_macro2::TokenStream::new();
                    let mut clone_from_body_token_stream = proc_macro2::TokenStream::new();

                    if fields.is_empty() {
                        clone_from_body_token_stream.extend(quote!(let _ = source;));
                    } else {
                        for (field, field_attribute) in fields {
                            let field_name = field.ident.as_ref().unwrap();

                            if let Some(clone) = field_attribute.method.as_ref() {
                                mark_fields.push((&field.ty, clone.clone()));

                                fields_token_stream.extend(quote! {
                                    #field_name: #clone(&self.#field_name),
                                });

                                clone_from_body_token_stream.extend(
                                    quote!(self.#field_name = #clone(&source.#field_name);),
                                );
                            } else {
                                clone_types.push(&field.ty);

                                fields_token_stream.extend(quote! {
                                    #field_name: ::core::clone::Clone::clone(&self.#field_name),
                                });

                                clone_from_body_token_stream.extend(
                                        quote!( ::core::clone::Clone::clone_from(&mut self.#field_name, &source.#field_name); ),
                                    );
                            }
                        }
                    }

                    if !use_bitwise_copy {
                        clone_token_stream.extend(quote! {
                            Self {
                                #fields_token_stream
                            }
                        });

                        clone_from_token_stream.extend(clone_from_body_token_stream);
                    }
                },
                Fields::Unnamed(_) => {
                    let mut fields_token_stream = proc_macro2::TokenStream::new();
                    let mut clone_from_body_token_stream = proc_macro2::TokenStream::new();

                    if fields.is_empty() {
                        clone_from_body_token_stream.extend(quote!(let _ = source;));
                    } else {
                        for (index, (field, field_attribute)) in fields.into_iter().enumerate() {
                            let field_name = Index::from(index);

                            if let Some(clone) = field_attribute.method.as_ref() {
                                mark_fields.push((&field.ty, clone.clone()));

                                fields_token_stream.extend(quote!(#clone(&self.#field_name),));

                                clone_from_body_token_stream.extend(
                                    quote!(self.#field_name = #clone(&source.#field_name);),
                                );
                            } else {
                                clone_types.push(&field.ty);

                                fields_token_stream.extend(
                                    quote! ( ::core::clone::Clone::clone(&self.#field_name), ),
                                );

                                clone_from_body_token_stream.extend(
                                        quote!( ::core::clone::Clone::clone_from(&mut self.#field_name, &source.#field_name); ),
                                    );
                            }
                        }
                    }

                    if !use_bitwise_copy {
                        clone_token_stream.extend(quote!(Self ( #fields_token_stream )));
                        clone_from_token_stream.extend(clone_from_body_token_stream);
                    }
                },
            }

            if use_bitwise_copy {
                clone_token_stream.extend(quote!(*self));
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
