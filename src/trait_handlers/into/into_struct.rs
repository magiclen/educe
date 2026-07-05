use std::collections::BTreeMap;

use quote::quote;
use syn::{Data, DeriveInput, Field, Meta, Path, Type};

use super::{
    TraitHandlerMultiple,
    models::{FieldAttribute, FieldAttributeBuilder, TypeAttributeBuilder},
};
use crate::{Trait, common::ident_index::IdentOrIndex, trait_handlers::TraitHandlerContext};

/// Generates the `Into` implementation for a struct.
pub(crate) struct IntoStructHandler;

impl TraitHandlerMultiple for IntoStructHandler {
    #[inline]
    fn trait_meta_handler(
        ast: &DeriveInput,
        _ctx: &mut TraitHandlerContext,
        token_stream: &mut proc_macro2::TokenStream,
        traits: &[Trait],
        meta: &[Meta],
    ) -> syn::Result<()> {
        let generated_impl_attributes =
            crate::common::attributes::generated_impl_attributes(&ast.attrs);

        let type_attribute = TypeAttributeBuilder {
            enable_types: true
        }
        .build_from_into_meta(meta)?;

        if let Data::Struct(data) = &ast.data {
            let fields = &data.fields;

            let field_attributes: BTreeMap<usize, FieldAttribute> = {
                let mut map = BTreeMap::new();

                for (index, field) in fields.iter().enumerate() {
                    let field_attribute = FieldAttributeBuilder {
                        enable_types: true
                    }
                    .build_from_attributes(&field.attrs, traits)?;

                    for ty in field_attribute.types.keys() {
                        if !type_attribute.types.contains_key(ty) {
                            return Err(super::panic::no_into_impl(ty));
                        }
                    }

                    map.insert(index, field_attribute);
                }

                map
            };

            for (target_ty, target) in type_attribute.types {
                // By default a `From` impl is generated because it provides `Into` for free; the `into` flag asks for a direct `Into` impl instead.
                let generate_from = !target.force_into;

                // The conversion body takes the source value from `self` in an `Into` impl and from the `value` parameter in a `From` impl.
                let source = if generate_from { quote!(value) } else { quote!(self) };

                let bound = target.bound;

                let mut into_types: Vec<&Type> = Vec::new();

                let mut into_token_stream = proc_macro2::TokenStream::new();

                let (index, field, method) = {
                    let fields = &data.fields;

                    if fields.len() == 1 {
                        let field = fields.into_iter().next().unwrap();

                        let method = if let Some(field_attribute) = field_attributes.get(&0) {
                            if let Some(method) = field_attribute.types.get(&target_ty) {
                                method.as_ref()
                            } else {
                                None
                            }
                        } else {
                            None
                        };

                        (0usize, field, method)
                    } else {
                        let mut into_field: Option<(usize, &Field, Option<&Path>)> = None;

                        for (index, field) in fields.iter().enumerate() {
                            if let Some(field_attribute) = field_attributes.get(&index)
                                && let Some((key, method)) =
                                    field_attribute.types.get_key_value(&target_ty)
                            {
                                if into_field.is_some() {
                                    return Err(super::panic::multiple_into_fields(key));
                                }

                                into_field = Some((index, field, method.as_ref()));
                            }
                        }

                        if into_field.is_none() {
                            // search the same type
                            for (index, field) in fields.iter().enumerate() {
                                let field_ty = super::common::to_hash_type(&field.ty);

                                if target_ty.eq(&field_ty) {
                                    if into_field.is_some() {
                                        // multiple candidates
                                        into_field = None;

                                        break;
                                    }

                                    into_field = Some((index, field, None));
                                }
                            }
                        }

                        if let Some(into_field) = into_field {
                            into_field
                        } else {
                            return Err(super::panic::no_into_field(&target_ty));
                        }
                    }
                };

                let field_name = IdentOrIndex::from_ident_with_index(field.ident.as_ref(), index);

                if let Some(method) = method {
                    into_token_stream.extend(quote!( #method(#source.#field_name) ));
                } else {
                    let ty = &field.ty;

                    let field_ty = super::common::to_hash_type(ty);

                    if target_ty.eq(&field_ty) {
                        into_token_stream.extend(quote!( #source.#field_name ));
                    } else {
                        into_types.push(ty);

                        into_token_stream
                            .extend(quote!( ::core::convert::Into::into(#source.#field_name) ));
                    }
                }

                let ident = &ast.ident;

                let bound = bound.into_where_predicates_by_generic_parameters_check_types_shallow(
                    &ast.generics.params,
                    &syn::parse2(quote!(::core::convert::Into<#target_ty>)).unwrap(),
                    &into_types,
                );

                // clone generics in order to not to affect other Into<T> implementations
                let mut generics = ast.generics.clone();

                let where_clause = generics.make_where_clause();

                for where_predicate in bound {
                    where_clause.predicates.push(where_predicate);
                }

                let (impl_generics, ty_generics, _) = ast.generics.split_for_impl();

                token_stream.extend(if generate_from {
                    quote! {
                        #generated_impl_attributes
                        impl #impl_generics ::core::convert::From<#ident #ty_generics> for #target_ty #where_clause {
                            #[inline]
                            fn from(value: #ident #ty_generics) -> Self {
                                #into_token_stream
                            }
                        }
                    }
                } else {
                    quote! {
                        #generated_impl_attributes
                        impl #impl_generics ::core::convert::Into<#target_ty> for #ident #ty_generics #where_clause {
                            #[inline]
                            fn into(self) -> #target_ty {
                                #into_token_stream
                            }
                        }
                    }
                });
            }
        }

        Ok(())
    }
}
