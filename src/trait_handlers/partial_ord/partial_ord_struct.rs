use std::collections::BTreeMap;

use syn::{Data, DeriveInput, ExprPath, Field, Meta, Type, spanned::Spanned};

use super::{
    TraitHandler,
    models::{FieldAttribute, FieldAttributeBuilder, TypeAttributeBuilder},
};
use crate::{
    Trait,
    common::{
        attributes::{borrow_field, is_packed},
        bound::{BOUND_EXCEPTIONS_ORDER, Bound},
        ident_index::IdentOrIndex,
        quote_mixed,
        where_predicates_bool::{WherePredicates, extend_where_predicates},
    },
    trait_handlers::TraitHandlerContext,
};

/// Generates the `PartialOrd` implementation for a struct.
pub(crate) struct PartialOrdStructHandler;

impl TraitHandler for PartialOrdStructHandler {
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
        .build_from_partial_ord_meta(meta)?;

        let mut partial_ord_types: Vec<&Type> = Vec::new();

        // A `#[repr(packed)]` type reads every compared field through a copy, so those field types additionally have to be `Copy`.
        let is_packed = is_packed(&ast.attrs);
        let mut copy_types: Vec<&Type> = Vec::new();

        let mut partial_cmp_token_stream = proc_macro2::TokenStream::new();

        if let Data::Struct(data) = &ast.data {
            let this = quote_mixed!(self);
            let that = quote_mixed!(other);

            // Fields are compared in ascending rank order, so they are collected into a map keyed by rank.
            // The default rank of a field is `isize::MIN` plus its ordinal position, which keeps the declaration order when no rank is given.
            let mut fields: BTreeMap<isize, (usize, &Field, FieldAttribute)> = BTreeMap::new();

            for (index, field) in data.fields.iter().enumerate() {
                let field_attribute = FieldAttributeBuilder {
                    enable_ignore: true,
                    enable_method: true,
                    enable_rank:   true,
                    rank:          isize::MIN + index as isize,
                }
                .build_from_attributes(&field.attrs, traits)?;

                if field_attribute.ignore {
                    continue;
                }

                let rank = field_attribute.rank;

                if fields.contains_key(&rank) {
                    return Err(super::panic::reuse_a_rank(
                        field_attribute.rank_span.unwrap_or_else(|| field.span()),
                        rank,
                    ));
                }

                fields.insert(rank, (index, field, field_attribute));
            }

            let built_in_partial_cmp: ExprPath =
                syn::parse2(quote_mixed!(::core::cmp::PartialOrd::partial_cmp)).unwrap();

            for (index, field, field_attribute) in fields.values() {
                let field_name = IdentOrIndex::from_ident_with_index(field.ident.as_ref(), *index);

                if is_packed {
                    copy_types.push(&field.ty);
                }

                let partial_cmp = field_attribute.method.as_ref().unwrap_or_else(|| {
                    partial_ord_types.push(&field.ty);

                    &built_in_partial_cmp
                });

                let self_ref = borrow_field(is_packed, &this, &field_name);
                let other_ref = borrow_field(is_packed, &that, &field_name);

                // A method taken from a fallback `Ord` field attribute returns `Ordering`, so its result has to be wrapped in `Some` here.
                let comparison = if field_attribute.method_returns_ordering {
                    quote_mixed!(::core::option::Option::Some(#partial_cmp(#self_ref, #other_ref)))
                } else {
                    quote_mixed!(#partial_cmp(#self_ref, #other_ref))
                };

                partial_cmp_token_stream.extend(quote_mixed! {
                    match #comparison {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                        ::core::option::Option::Some(::core::cmp::Ordering::Greater) => return ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        ::core::option::Option::Some(::core::cmp::Ordering::Less) => return ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        ::core::option::Option::None => return ::core::option::Option::None,
                    }
                });
            }
        }

        let ident = &ast.ident;

        let bound_is_auto = matches!(type_attribute.bound, Bound::Auto);

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
                &syn::parse2(quote_mixed!(::core::cmp::PartialOrd)).unwrap(),
                &partial_ord_types,
                &ast.ident,
                &BOUND_EXCEPTIONS_ORDER,
            );

        extend_where_predicates(&mut bound, packed_copy_predicates);

        if bound_is_auto {
            ctx.inherit_from(super::prerequisites(), &mut bound);
        }

        ctx.record(Trait::PartialOrd, &bound);

        let mut generics = ast.generics.clone();

        let where_clause = generics.make_where_clause();

        for where_predicate in bound {
            where_clause.predicates.push(where_predicate);
        }

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        token_stream.extend(quote_mixed! {
            #generated_impl_attributes
            impl #impl_generics ::core::cmp::PartialOrd for #ident #ty_generics #where_clause {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &Self,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    #partial_cmp_token_stream

                    ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                }
            }
        });

        Ok(())
    }
}
