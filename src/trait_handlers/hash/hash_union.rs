use proc_macro2::{Ident, Span};
use syn::{Data, DeriveInput, Meta};

use super::models::{FieldAttributeBuilder, TypeAttributeBuilder};
use crate::{
    common::{bound::BOUND_EXCEPTIONS_HASH, quote_mixed, union::union_bytes},
    supported_traits::Trait,
    trait_handlers::{TraitHandler, TraitHandlerContext},
};

/// Generates the `Hash` implementation for a union.
pub(crate) struct HashUnionHandler;

impl TraitHandler for HashUnionHandler {
    #[inline]
    fn trait_meta_handler<'a>(
        ast: &'a DeriveInput,
        _ctx: &mut TraitHandlerContext<'a>,
        token_stream: &mut proc_macro2::TokenStream,
        traits: &[Trait],
        meta: &Meta,
    ) -> syn::Result<()> {
        let generated_impl_attributes =
            crate::common::attributes::generated_impl_attributes(&ast.attrs);

        let type_attribute =
            TypeAttributeBuilder {
                enable_flag: true, enable_unsafe: true, enable_bound: true
            }
            .build_from_hash_meta(meta)?;

        if !type_attribute.has_unsafe {
            return Err(super::panic::union_without_unsafe(meta));
        }

        if let Data::Union(data) = &ast.data {
            for field in data.fields.named.iter() {
                let _ = FieldAttributeBuilder {
                    enable_ignore: false, enable_method: false
                }
                .build_from_attributes(&field.attrs, traits)?;
            }
        }

        let ident = &ast.ident;
        let hasher_ident = crate::common::generics::unused_ident(&ast.generics, "H");

        // Hashing the storage as bytes needs nothing from the field types, so the automatic bound stays empty and only an explicit one contributes predicates.
        let bound = type_attribute.bound.into_where_predicates_by_generic_parameters_check_types(
            &ast.generics.params,
            &syn::parse2(quote_mixed!(::core::hash::Hash)).unwrap(),
            &[],
            &ast.ident,
            &BOUND_EXCEPTIONS_HASH,
        );

        let generics = crate::common::generics::with_predicates(ast.generics.clone(), bound);

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let data = Ident::new("data", Span::mixed_site());
        let bytes = union_bytes(&data, &quote_mixed!(self));

        token_stream.extend(quote_mixed! {
            #generated_impl_attributes
            impl #impl_generics ::core::hash::Hash for #ident #ty_generics #where_clause {
                #[inline]
                fn hash<#hasher_ident: ::core::hash::Hasher>(&self, state: &mut #hasher_ident) {
                    #bytes

                    ::core::hash::Hash::hash(#data, state)
                }
            }
        });

        Ok(())
    }
}
