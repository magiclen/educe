use syn::{Data, DeriveInput, Fields, Meta, Type};

use super::{
    TraitHandler,
    models::{FieldAttributeBuilder, FieldName, TypeAttributeBuilder, TypeName},
};
use crate::{
    Trait,
    common::{
        attributes::{borrow_field, is_packed},
        bound::BOUND_EXCEPTIONS_DEBUG,
        ident_index::IdentOrIndex,
        quote_mixed,
        where_predicates_bool::{WherePredicates, extend_where_predicates},
    },
    trait_handlers::TraitHandlerContext,
};

pub struct DebugStructHandler;

impl TraitHandler for DebugStructHandler {
    fn trait_meta_handler<'a>(
        ast: &'a DeriveInput,
        _ctx: &mut TraitHandlerContext<'a>,
        token_stream: &mut proc_macro2::TokenStream,
        traits: &[Trait],
        meta: &Meta,
    ) -> syn::Result<()> {
        let generated_impl_attributes =
            crate::common::attributes::generated_impl_attributes(&ast.attrs);

        let is_tuple = {
            if let Data::Struct(data) = &ast.data {
                matches!(data.fields, Fields::Unnamed(_))
            } else {
                true
            }
        };

        let type_attribute = TypeAttributeBuilder {
            enable_flag:        true,
            enable_unsafe:      false,
            enable_name:        true,
            enable_named_field: true,
            enable_bound:       true,
            name:               TypeName::Default,
            named_field:        !is_tuple,
        }
        .build_from_debug_meta(meta)?;

        let name = type_attribute.name.to_ident_by_ident(&ast.ident);
        let helper_types = super::common::HelperTypes::new(ast);
        let raw_string_ident = &helper_types.raw_string;

        let mut debug_types: Vec<&Type> = Vec::new();

        // A `#[repr(packed)]` type reads every formatted field through a copy, so those field types additionally have to be `Copy`.
        let is_packed = is_packed(&ast.attrs);
        let mut copy_types: Vec<&Type> = Vec::new();
        let this = quote_mixed!(self);

        let mut builder_token_stream = proc_macro2::TokenStream::new();
        let mut mark_fields = Vec::new();
        let mut has_fields = false;

        if type_attribute.named_field {
            builder_token_stream.extend(if let Some(name) = name {
                quote_mixed!(let mut builder = f.debug_struct(stringify!(#name));)
            } else {
                let raw_string_type = super::common::create_raw_string_type(raw_string_ident);
                let map_builder = super::common::create_debug_map_builder();

                quote_mixed! {
                    #raw_string_type

                    #map_builder
                }
            });

            if let Data::Struct(data) = &ast.data {
                for (index, field) in data.fields.iter().enumerate() {
                    let field_attribute = FieldAttributeBuilder {
                        enable_name:   true,
                        enable_ignore: true,
                        enable_method: true,
                        name:          FieldName::Default,
                    }
                    .build_from_attributes(&field.attrs, traits)?;

                    if field_attribute.ignore {
                        continue;
                    }

                    // The displayed field name is a plain string, so a tuple field can be shown with its real name `0`, which is not a valid ident.
                    let (key, field_name) = match field_attribute.name {
                        FieldName::Custom(name) => (
                            name.to_string(),
                            IdentOrIndex::from_ident_with_index(field.ident.as_ref(), index),
                        ),
                        FieldName::Default => {
                            if let Some(ident) = field.ident.as_ref() {
                                (ident.to_string(), IdentOrIndex::from(ident))
                            } else {
                                (index.to_string(), IdentOrIndex::from(index))
                            }
                        },
                    };

                    let key = syn::LitStr::new(&key, proc_macro2::Span::call_site());

                    let ty = &field.ty;

                    if is_packed {
                        copy_types.push(ty);
                    }

                    let field_ref = borrow_field(is_packed, &this, &field_name);

                    if let Some(method) = field_attribute.method {
                        let arg = super::common::create_format_arg(
                            &helper_types.field,
                            ty,
                            &method,
                            field_ref,
                        );

                        builder_token_stream.extend(arg);
                        mark_fields.push((ty, method));

                        builder_token_stream.extend(if name.is_some() {
                            quote_mixed! (builder.field(#key, &arg);)
                        } else {
                            quote_mixed! (builder.entry(&#raw_string_ident(#key), &arg);)
                        });
                    } else {
                        debug_types.push(ty);

                        builder_token_stream.extend(if name.is_some() {
                            quote_mixed! (builder.field(#key, &#field_ref);)
                        } else {
                            quote_mixed! (builder.entry(&#raw_string_ident(#key), &#field_ref);)
                        });
                    }

                    has_fields = true;
                }
            }
        } else {
            // A struct without a name is formatted like a plain tuple, which the standard builder spells with an empty name.
            let name_string = syn::LitStr::new(
                &name.map(|name| name.to_string()).unwrap_or_default(),
                proc_macro2::Span::call_site(),
            );

            builder_token_stream
                .extend(quote_mixed!(let mut builder = f.debug_tuple(#name_string);));

            if let Data::Struct(data) = &ast.data {
                for (index, field) in data.fields.iter().enumerate() {
                    let field_attribute = FieldAttributeBuilder {
                        enable_name:   false,
                        enable_ignore: true,
                        enable_method: true,
                        name:          FieldName::Default,
                    }
                    .build_from_attributes(&field.attrs, traits)?;

                    if field_attribute.ignore {
                        continue;
                    }

                    let field_name =
                        IdentOrIndex::from_ident_with_index(field.ident.as_ref(), index);

                    let ty = &field.ty;

                    if is_packed {
                        copy_types.push(ty);
                    }

                    let field_ref = borrow_field(is_packed, &this, &field_name);

                    if let Some(method) = field_attribute.method {
                        let arg = super::common::create_format_arg(
                            &helper_types.field,
                            ty,
                            &method,
                            field_ref,
                        );

                        builder_token_stream.extend(arg);
                        mark_fields.push((ty, method));

                        builder_token_stream.extend(quote_mixed! (builder.field(&arg);));
                    } else {
                        debug_types.push(ty);

                        builder_token_stream.extend(quote_mixed! (builder.field(&#field_ref);));
                    }

                    has_fields = true;
                }
            }
        }

        let ident = &ast.ident;

        if !has_fields && name.is_none() {
            return Err(super::panic::unit_struct_need_name(ident));
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

        let mut bound =
            type_attribute.bound.into_where_predicates_by_generic_parameters_check_types(
                &ast.generics.params,
                &syn::parse2(quote_mixed!(::core::fmt::Debug)).unwrap(),
                &debug_types,
                &ast.ident,
                &BOUND_EXCEPTIONS_DEBUG,
            );

        extend_where_predicates(&mut bound, packed_copy_predicates);

        let mut generics = ast.generics.clone();

        let where_clause = generics.make_where_clause();

        for where_predicate in bound {
            where_clause.predicates.push(where_predicate);
        }

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        token_stream.extend(quote_mixed! {
            #generated_impl_attributes
            impl #impl_generics ::core::fmt::Debug for #ident #ty_generics #where_clause {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    #builder_token_stream

                    builder.finish()
                }
            }
        });

        for (field_ty, method) in &mark_fields {
            token_stream
                .extend(super::common::create_mark_method_used(ast, &generics, field_ty, method));
        }

        Ok(())
    }
}
