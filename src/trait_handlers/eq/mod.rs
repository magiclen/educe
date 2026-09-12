//! Automatic bounds check every ordinary field, including concrete types.
//!
//! ```compile_fail
//! use educe::Educe;
//! #[derive(PartialEq, Educe)]
//! #[educe(Eq)]
//! struct Value(f64);
//! ```
//!
//! ```compile_fail
//! use educe::Educe;
//! #[derive(PartialEq, Educe)]
//! #[educe(Eq)]
//! enum Value { Number(f64) }
//! ```

mod models;

use models::{FieldAttributeBuilder, TypeAttributeBuilder};
use syn::{Data, DeriveInput, Meta, Type, visit_mut::VisitMut};

use super::TraitHandler;
use crate::{
    Trait,
    common::{
        bound::{BOUND_EXCEPTIONS_EQUALITY, Bound},
        quote_mixed,
    },
    trait_handlers::TraitHandlerContext,
};

/// Returns the traits whose recorded bounds `Eq` inherits when its own bound is automatic.
pub(crate) fn prerequisites() -> &'static [Trait] {
    &[
        #[cfg(feature = "PartialEq")]
        Trait::PartialEq,
    ]
}

pub(crate) struct EqHandler;

impl TraitHandler for EqHandler {
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

        let type_attribute = TypeAttributeBuilder {
            enable_flag: true, enable_bound: true
        }
        .build_from_eq_meta(meta)?;

        // The `PartialEq` handler has already worked out which fields take part in the comparison, so its list is reused; without it every ordinary field is checked.
        #[cfg(feature = "PartialEq")]
        let compared_types = ctx.partial_eq_types().map(<[&Type]>::to_vec);
        #[cfg(not(feature = "PartialEq"))]
        let compared_types: Option<Vec<&Type>> = None;

        let mut field_types = Vec::new();

        match &ast.data {
            Data::Struct(data) => {
                for field in data.fields.iter() {
                    let _ = FieldAttributeBuilder.build_from_attributes(&field.attrs, traits)?;

                    if compared_types.is_none() {
                        field_types.push(&field.ty);
                    }
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

                        if compared_types.is_none() {
                            field_types.push(&field.ty);
                        }
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

        // A union records an empty list, so both sides agree that its bound carries no field predicates.
        let field_types = compared_types.unwrap_or(field_types);

        let ident = &ast.ident;

        let bound_is_auto = matches!(type_attribute.bound, Bound::Auto);

        // The automatic bound uses the `Eq` trait itself (not `PartialEq`), matching the behavior of the built-in `#[derive(Eq)]`.
        let mut bound =
            type_attribute.bound.into_where_predicates_by_generic_parameters_check_types(
                &ast.generics.params,
                &syn::parse2(quote_mixed!(::core::cmp::Eq)).unwrap(),
                &field_types,
                &ast.ident,
                &BOUND_EXCEPTIONS_EQUALITY,
            );

        if bound_is_auto {
            ctx.inherit_from(prerequisites(), &mut bound);
        }

        ctx.record(Trait::Eq, &bound);

        let mut generics = ast.generics.clone();

        let where_clause = generics.make_where_clause();

        for where_predicate in bound {
            where_clause.predicates.push(where_predicate);
        }

        if bound_is_auto && !field_types.is_empty() {
            let lint_attributes = crate::common::attributes::generated_lint_attributes(&ast.attrs);
            let mut helper_generics = generics.clone();
            let mut replace_self = crate::common::generics::ReplaceSelf::new(ast);
            replace_self.visit_generics_mut(&mut helper_generics);
            let field_types: Vec<_> = field_types
                .into_iter()
                .map(|ty| {
                    let mut ty = ty.clone();
                    replace_self.visit_type_mut(&mut ty);
                    ty
                })
                .collect();
            let (impl_generics, _, where_clause) = helper_generics.split_for_impl();

            // This unused function checks full field types without adding public bounds or runtime calls.
            token_stream.extend(quote_mixed! {
                #lint_attributes
                const _: () = {
                    #[allow(dead_code, clippy::all)]
                    fn __educe_eq_fields #impl_generics () #where_clause {
                        fn __educe_assert_eq<T: ?Sized + ::core::cmp::Eq>() {}
                        #(__educe_assert_eq::<#field_types>();)*
                    }
                };
            });
        }

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        token_stream.extend(quote_mixed! {
            #generated_impl_attributes
            impl #impl_generics ::core::cmp::Eq for #ident #ty_generics #where_clause {
            }
        });

        Ok(())
    }
}
