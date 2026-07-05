use quote::quote;
use syn::{Data, DeriveInput, Meta};

use super::models::{FieldAttributeBuilder, TypeAttributeBuilder};
use crate::{
    common::where_predicates_bool::WherePredicates,
    supported_traits::Trait,
    trait_handlers::{TraitHandler, TraitHandlerContext},
};

/// Generates the `PartialEq` implementation for a union.
pub(crate) struct PartialEqUnionHandler;

impl TraitHandler for PartialEqUnionHandler {
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

        let type_attribute =
            TypeAttributeBuilder {
                enable_flag: true, enable_unsafe: true, enable_bound: false
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

        let ident = &ast.ident;

        // The union implementation adds no extra bounds, so record an empty predicate set for later inheritance.
        ctx.record(Trait::PartialEq, &WherePredicates::new());

        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

        token_stream.extend(quote! {
            #generated_impl_attributes
            impl #impl_generics ::core::cmp::PartialEq for #ident #ty_generics #where_clause {
                #[inline]
                fn eq(&self, other: &Self) -> bool {
                    // The whole memory of the two unions is compared byte by byte on purpose, including any padding bytes, because the active field cannot be known at runtime.
                    // The user opted in to this behavior with the `unsafe` keyword in the attribute.
                    let size = ::core::mem::size_of::<Self>();
                    let self_data = unsafe { ::core::slice::from_raw_parts(self as *const Self as *const u8, size) };
                    let other_data = unsafe { ::core::slice::from_raw_parts(other as *const Self as *const u8, size) };

                    ::core::cmp::PartialEq::eq(self_data, other_data)
                }
            }
        });

        Ok(())
    }
}
