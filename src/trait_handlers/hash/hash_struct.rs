use syn::{Data, DeriveInput, ExprPath, Meta, Type};

use super::{
    TraitHandler,
    models::{FieldAttributeBuilder, TypeAttributeBuilder},
};
use crate::{
    Trait,
    common::{
        attributes::{borrow_field, is_packed},
        bound::BOUND_EXCEPTIONS_HASH,
        ident_index::IdentOrIndex,
        quote_mixed,
        where_predicates_bool::{WherePredicates, extend_where_predicates},
    },
    trait_handlers::TraitHandlerContext,
};

/// Generates the `Hash` implementation for a struct.
pub(crate) struct HashStructHandler;

impl TraitHandler for HashStructHandler {
    #[inline]
    fn trait_meta_handler(
        ast: &DeriveInput,
        _ctx: &mut TraitHandlerContext,
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
            .build_from_hash_meta(meta)?;

        let mut hash_types: Vec<&Type> = Vec::new();

        // A `#[repr(packed)]` type reads every hashed field through a copy, so those field types additionally have to be `Copy`.
        let is_packed = is_packed(&ast.attrs);
        let mut copy_types: Vec<&Type> = Vec::new();

        let mut hash_token_stream = proc_macro2::TokenStream::new();

        if let Data::Struct(data) = &ast.data {
            let this = quote_mixed!(self);

            let built_in_hash: ExprPath =
                syn::parse2(quote_mixed!(::core::hash::Hash::hash)).unwrap();

            for (index, field) in data.fields.iter().enumerate() {
                let field_attribute = FieldAttributeBuilder {
                    enable_ignore: true,
                    enable_method: true,
                }
                .build_from_attributes(&field.attrs, traits)?;

                if field_attribute.ignore {
                    continue;
                }

                let field_name = if let Some(ident) = field.ident.as_ref() {
                    IdentOrIndex::from(ident)
                } else {
                    IdentOrIndex::from(index)
                };

                if is_packed {
                    copy_types.push(&field.ty);
                }

                let hash = field_attribute.method.as_ref().unwrap_or_else(|| {
                    hash_types.push(&field.ty);
                    &built_in_hash
                });

                let field_ref = borrow_field(is_packed, &this, &field_name);

                hash_token_stream.extend(quote_mixed!( #hash(#field_ref, state); ));
            }
        }

        let ident = &ast.ident;
        let hasher_ident = crate::common::generics::unused_ident(&ast.generics, "H");

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
                &syn::parse2(quote_mixed!(::core::hash::Hash)).unwrap(),
                &hash_types,
                &ast.ident,
                &BOUND_EXCEPTIONS_HASH,
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
            impl #impl_generics ::core::hash::Hash for #ident #ty_generics #where_clause {
                #[inline]
                fn hash<#hasher_ident: ::core::hash::Hasher>(&self, state: &mut #hasher_ident) {
                    #hash_token_stream
                }
            }
        });

        Ok(())
    }
}
