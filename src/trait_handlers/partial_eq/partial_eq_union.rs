use proc_macro2::{Ident, Span};
use syn::{Data, DeriveInput, Meta};

use super::models::{FieldAttributeBuilder, TypeAttributeBuilder};
use crate::{
    common::{bound::BOUND_EXCEPTIONS_EQUALITY, quote_mixed, union::union_bytes},
    supported_traits::Trait,
    trait_handlers::{TraitHandler, TraitHandlerContext},
};

/// Generates the `PartialEq` implementation for a union.
pub(crate) struct PartialEqUnionHandler;

impl TraitHandler for PartialEqUnionHandler {
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
                enable_flag: true, enable_unsafe: true, enable_bound: true
            }
            .build_from_partial_eq_meta(meta)?;

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

        // A union is compared as bytes, so `Eq` has no field type to check either.
        #[cfg(feature = "Eq")]
        ctx.record_partial_eq_types(&[]);

        let ident = &ast.ident;

        // Comparing the storage as bytes needs nothing from the field types, so the automatic bound stays empty and only an explicit one contributes predicates.
        let bound = type_attribute.bound.into_where_predicates_by_generic_parameters_check_types(
            &ast.generics.params,
            &syn::parse2(quote_mixed!(::core::cmp::PartialEq)).unwrap(),
            &[],
            &ast.ident,
            &BOUND_EXCEPTIONS_EQUALITY,
        );

        ctx.record(Trait::PartialEq, &bound);

        let generics = crate::common::generics::with_predicates(ast.generics.clone(), bound);

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let self_data = Ident::new("self_data", Span::mixed_site());
        let other_data = Ident::new("other_data", Span::mixed_site());
        let self_bytes = union_bytes(&self_data, &quote_mixed!(self));
        let other_bytes = union_bytes(&other_data, &quote_mixed!(other));

        token_stream.extend(quote_mixed! {
            #generated_impl_attributes
            impl #impl_generics ::core::cmp::PartialEq for #ident #ty_generics #where_clause {
                #[inline]
                fn eq(&self, other: &Self) -> ::core::primitive::bool {
                    #self_bytes

                    #other_bytes

                    ::core::cmp::PartialEq::eq(#self_data, #other_data)
                }
            }
        });

        Ok(())
    }
}
