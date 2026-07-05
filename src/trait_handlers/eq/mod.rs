mod models;

use models::{FieldAttributeBuilder, TypeAttributeBuilder};
use quote::quote;
use syn::{Data, DeriveInput, Meta};

use super::TraitHandler;
use crate::{
    Trait,
    common::bound::{BOUND_EXCEPTIONS_EQUALITY, Bound},
    trait_handlers::TraitHandlerContext,
};

/// Returns the traits whose recorded bounds `Eq` inherits when its own bound is automatic.
pub(crate) fn prerequisites() -> Vec<Trait> {
    vec![
        #[cfg(feature = "PartialEq")]
        Trait::PartialEq,
    ]
}

pub(crate) struct EqHandler;

impl TraitHandler for EqHandler {
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
        .build_from_eq_meta(meta)?;

        let mut field_types = Vec::new();

        match &ast.data {
            Data::Struct(data) => {
                for field in data.fields.iter() {
                    let _ = FieldAttributeBuilder.build_from_attributes(&field.attrs, traits)?;

                    field_types.push(&field.ty);
                }
            },
            Data::Enum(data) => {
                for variant in data.variants.iter() {
                    let _ = TypeAttributeBuilder {
                        enable_flag: false, enable_bound: false
                    }
                    .build_from_attributes(&variant.attrs, traits)?;

                    for field in variant.fields.iter() {
                        let _ =
                            FieldAttributeBuilder.build_from_attributes(&field.attrs, traits)?;

                        field_types.push(&field.ty);
                    }
                }
            },
            Data::Union(data) => {
                // A union compares itself byte by byte, so the field types never need an `Eq` bound of their own.
                for field in data.fields.named.iter() {
                    let _ = FieldAttributeBuilder.build_from_attributes(&field.attrs, traits)?;
                }
            },
        }

        let ident = &ast.ident;

        let bound_is_auto = matches!(type_attribute.bound, Bound::Auto);

        // The automatic bound uses the `Eq` trait itself (not `PartialEq`), matching the behavior of the built-in `#[derive(Eq)]`.
        let mut bound =
            type_attribute.bound.into_where_predicates_by_generic_parameters_check_types(
                &ast.generics.params,
                &syn::parse2(quote!(::core::cmp::Eq)).unwrap(),
                &field_types,
                &ast.ident,
                &BOUND_EXCEPTIONS_EQUALITY,
            );

        if bound_is_auto {
            ctx.inherit_from(&prerequisites(), &mut bound);
        }

        ctx.record(Trait::Eq, &bound);

        let mut generics = ast.generics.clone();

        let where_clause = generics.make_where_clause();

        for where_predicate in bound {
            where_clause.predicates.push(where_predicate);
        }

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        token_stream.extend(quote! {
            #generated_impl_attributes
            impl #impl_generics ::core::cmp::Eq for #ident #ty_generics #where_clause {
            }
        });

        Ok(())
    }
}
