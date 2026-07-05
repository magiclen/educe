use quote::quote;
use syn::{Data, DeriveInput, Meta};

use super::{
    TraitHandler,
    models::{FieldAttributeBuilder, TypeAttributeBuilder},
};
use crate::{
    common::bound::BOUND_EXCEPTIONS_COPY, supported_traits::Trait,
    trait_handlers::TraitHandlerContext,
};

/// Generates the `Clone` implementation for a union.
pub(crate) struct CloneUnionHandler;

impl TraitHandler for CloneUnionHandler {
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

        let mut field_types = Vec::new();

        if let Data::Union(data) = &ast.data {
            for field in data.fields.named.iter() {
                let _ = FieldAttributeBuilder {
                    enable_method: false
                }
                .build_from_attributes(&field.attrs, traits)?;

                field_types.push(&field.ty);
            }
        }

        let ident = &ast.ident;

        // A union can only be cloned by a bitwise copy, so the fields must satisfy `Copy` no matter whether `Copy` is derived together or not.
        let bound = type_attribute.bound.into_where_predicates_by_generic_parameters_check_types(
            &ast.generics.params,
            &syn::parse2(quote!(::core::marker::Copy)).unwrap(),
            &field_types,
            &ast.ident,
            &BOUND_EXCEPTIONS_COPY,
        );

        ctx.record(Trait::Clone, &bound);

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
                    *self
                }
            }
        });

        Ok(())
    }
}
