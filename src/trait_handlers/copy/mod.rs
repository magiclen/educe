mod models;

use models::{FieldAttributeBuilder, TypeAttributeBuilder};
use quote::quote;
use syn::{Data, DeriveInput, Meta};

use super::TraitHandler;
use crate::{
    Trait,
    common::bound::{BOUND_EXCEPTIONS_COPY, Bound},
    trait_handlers::TraitHandlerContext,
};

/// Returns the traits whose recorded bounds `Copy` inherits when its own bound is automatic.
pub(crate) fn prerequisites() -> Vec<Trait> {
    vec![
        #[cfg(feature = "Clone")]
        Trait::Clone,
    ]
}

pub(crate) struct CopyHandler;

impl TraitHandler for CopyHandler {
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
        .build_from_copy_meta(meta)?;

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
                for field in data.fields.named.iter() {
                    let _ = FieldAttributeBuilder.build_from_attributes(&field.attrs, traits)?;

                    field_types.push(&field.ty);
                }
            },
        }

        let ident = &ast.ident;

        let bound_is_auto = matches!(type_attribute.bound, Bound::Auto);

        let mut bound =
            type_attribute.bound.into_where_predicates_by_generic_parameters_check_types(
                &ast.generics.params,
                &syn::parse2(quote!(::core::marker::Copy)).unwrap(),
                &field_types,
                &ast.ident,
                &BOUND_EXCEPTIONS_COPY,
            );

        // `Copy` requires `Clone`, so an automatic bound also has to carry the predicates of the `Clone` impl that Educe just emitted.
        if bound_is_auto {
            ctx.inherit_from(&prerequisites(), &mut bound);
        }

        ctx.record(Trait::Copy, &bound);

        let mut generics = ast.generics.clone();

        let where_clause = generics.make_where_clause();

        for where_predicate in bound {
            where_clause.predicates.push(where_predicate);
        }

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        token_stream.extend(quote! {
            #generated_impl_attributes
            impl #impl_generics ::core::marker::Copy for #ident #ty_generics #where_clause {
            }
        });

        Ok(())
    }
}
