use syn::{Data, DeriveInput, ExprPath, Field, Fields, Index, Meta, Type, punctuated::Punctuated};

use super::models::{FieldAttribute, FieldAttributeBuilder, TypeAttributeBuilder};
use crate::{
    TraitHandler,
    common::{
        attributes::{borrow_field, is_packed},
        bound::BOUND_EXCEPTIONS_CLONE,
        quote_mixed,
        where_predicates_bool::{WherePredicates, extend_where_predicates},
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
        let mut mark_fields: Vec<(&Type, ExprPath)> = Vec::new();

        // A `#[repr(packed)]` type reads every cloned field through a copy, so those field types additionally have to be `Copy`.
        let is_packed = is_packed(&ast.attrs);
        let mut copy_types: Vec<&Type> = Vec::new();
        let this = quote_mixed!(self);
        let source = quote_mixed!(source);

        let mut clone_token_stream = proc_macro2::TokenStream::new();
        let mut clone_from_token_stream = proc_macro2::TokenStream::new();

        if let Data::Struct(data) = &ast.data {
            let mut fields: Vec<(&Field, FieldAttribute)> = Vec::new();

            for field in data.fields.iter() {
                let field_attribute = FieldAttributeBuilder {
                    enable_method: true
                }
                .build_from_attributes(&field.attrs, traits)?;

                fields.push((field, field_attribute));
            }

            let has_custom_method =
                fields.iter().any(|(_, field_attribute)| field_attribute.method.is_some());

            let use_bitwise_copy =
                super::can_use_bitwise_copy(ast, ctx, traits, has_custom_method)?;

            let mut clone_types: Vec<&Type> = Vec::new();

            if use_bitwise_copy {
                // A whole-value copy needs no field-wise body or field bounds.
                clone_token_stream.extend(quote_mixed!(*self));
            } else {
                match &data.fields {
                    Fields::Unit => {
                        clone_token_stream.extend(quote_mixed!(Self));
                        clone_from_token_stream.extend(quote_mixed!(let _ = source;));
                    },
                    Fields::Named(_) => {
                        let mut fields_token_stream = proc_macro2::TokenStream::new();
                        let mut clone_from_body_token_stream = proc_macro2::TokenStream::new();

                        if fields.is_empty() {
                            clone_from_body_token_stream.extend(quote_mixed!(let _ = source;));
                        } else {
                            for (field, field_attribute) in fields {
                                let field_name = field.ident.as_ref().unwrap();

                                if is_packed {
                                    copy_types.push(&field.ty);
                                }

                                let self_ref = borrow_field(is_packed, &this, field_name);
                                let source_ref = borrow_field(is_packed, &source, field_name);

                                if let Some(clone) = field_attribute.method.as_ref() {
                                    mark_fields.push((&field.ty, clone.clone()));

                                    fields_token_stream.extend(quote_mixed! {
                                        #field_name: #clone(#self_ref),
                                    });

                                    clone_from_body_token_stream.extend(
                                        quote_mixed!(self.#field_name = #clone(#source_ref);),
                                    );
                                } else {
                                    clone_types.push(&field.ty);

                                    fields_token_stream.extend(quote_mixed! {
                                        #field_name: ::core::clone::Clone::clone(#self_ref),
                                    });

                                    // A packed field cannot be borrowed mutably either, so the cloned value is assigned back instead of being cloned in place.
                                    clone_from_body_token_stream.extend(if is_packed {
                                        quote_mixed!(self.#field_name = ::core::clone::Clone::clone(#source_ref);)
                                    } else {
                                        quote_mixed!( ::core::clone::Clone::clone_from(&mut self.#field_name, #source_ref); )
                                    });
                                }
                            }
                        }

                        clone_token_stream.extend(quote_mixed! {
                            Self {
                                #fields_token_stream
                            }
                        });

                        clone_from_token_stream.extend(clone_from_body_token_stream);
                    },
                    Fields::Unnamed(_) => {
                        let mut fields_token_stream = proc_macro2::TokenStream::new();
                        let mut clone_from_body_token_stream = proc_macro2::TokenStream::new();

                        if fields.is_empty() {
                            clone_from_body_token_stream.extend(quote_mixed!(let _ = source;));
                        } else {
                            for (index, (field, field_attribute)) in fields.into_iter().enumerate()
                            {
                                let field_name = Index::from(index);

                                if is_packed {
                                    copy_types.push(&field.ty);
                                }

                                let self_ref = borrow_field(is_packed, &this, &field_name);
                                let source_ref = borrow_field(is_packed, &source, &field_name);

                                if let Some(clone) = field_attribute.method.as_ref() {
                                    mark_fields.push((&field.ty, clone.clone()));

                                    fields_token_stream.extend(quote_mixed!(#clone(#self_ref),));

                                    clone_from_body_token_stream.extend(
                                        quote_mixed!(self.#field_name = #clone(#source_ref);),
                                    );
                                } else {
                                    clone_types.push(&field.ty);

                                    fields_token_stream.extend(
                                        quote_mixed! ( ::core::clone::Clone::clone(#self_ref), ),
                                    );

                                    // A packed field cannot be borrowed mutably either, so the cloned value is assigned back instead of being cloned in place.
                                    clone_from_body_token_stream.extend(if is_packed {
                                        quote_mixed!(self.#field_name = ::core::clone::Clone::clone(#source_ref);)
                                    } else {
                                        quote_mixed!( ::core::clone::Clone::clone_from(&mut self.#field_name, #source_ref); )
                                    });
                                }
                            }
                        }

                        clone_token_stream.extend(quote_mixed!(Self ( #fields_token_stream )));
                        clone_from_token_stream.extend(clone_from_body_token_stream);
                    },
                }
            }

            let packed_copy_predicates = if is_packed {
                type_attribute.bound.packed_copy_predicates(
                    &ast.generics.params,
                    &copy_types,
                    &ast.ident,
                )
            } else {
                WherePredicates::new()
            };

            // The bound trait is always `Clone`; the `Copy` impl is emitted by the `Copy` handler with its own bounds.
            bound = type_attribute.bound.into_where_predicates_by_generic_parameters_check_types(
                &ast.generics.params,
                &syn::parse2(quote_mixed!(::core::clone::Clone)).unwrap(),
                &clone_types,
                &ast.ident,
                &BOUND_EXCEPTIONS_CLONE,
            );

            extend_where_predicates(&mut bound, packed_copy_predicates);

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

        let mut generics = ast.generics.clone();

        let where_clause = generics.make_where_clause();

        for where_predicate in bound {
            where_clause.predicates.push(where_predicate);
        }

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
