use std::collections::BTreeMap;

use quote::quote;
use syn::{Data, DeriveInput, Field, Meta, Path, Type, spanned::Spanned};

use super::{
    TraitHandler,
    models::{FieldAttribute, FieldAttributeBuilder, TypeAttributeBuilder},
};
use crate::{
    Trait,
    common::{
        bound::{BOUND_EXCEPTIONS_ORDER, Bound},
        ident_index::IdentOrIndex,
    },
    trait_handlers::TraitHandlerContext,
};

/// Generates the `Ord` implementation for a struct.
pub(crate) struct OrdStructHandler;

impl TraitHandler for OrdStructHandler {
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
        .build_from_ord_meta(meta)?;

        let mut ord_types: Vec<&Type> = Vec::new();

        let mut cmp_token_stream = proc_macro2::TokenStream::new();

        if let Data::Struct(data) = &ast.data {
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

            let built_in_cmp: Path = syn::parse2(quote!(::core::cmp::Ord::cmp)).unwrap();

            for (index, field, field_attribute) in fields.values() {
                let field_name = IdentOrIndex::from_ident_with_index(field.ident.as_ref(), *index);

                let cmp = field_attribute.method.as_ref().unwrap_or_else(|| {
                    ord_types.push(&field.ty);

                    &built_in_cmp
                });

                cmp_token_stream.extend(quote! {
                    match #cmp(&self.#field_name, &other.#field_name) {
                        ::core::cmp::Ordering::Equal => (),
                        ::core::cmp::Ordering::Greater => return ::core::cmp::Ordering::Greater,
                        ::core::cmp::Ordering::Less => return ::core::cmp::Ordering::Less,
                    }
                });
            }
        }

        let ident = &ast.ident;

        let bound_is_auto = matches!(type_attribute.bound, Bound::Auto);

        let mut bound =
            type_attribute.bound.into_where_predicates_by_generic_parameters_check_types(
                &ast.generics.params,
                &syn::parse2(quote!(::core::cmp::Ord)).unwrap(),
                &ord_types,
                &ast.ident,
                &BOUND_EXCEPTIONS_ORDER,
            );

        if bound_is_auto {
            ctx.inherit_from(&super::prerequisites(), &mut bound);
        }

        ctx.record(Trait::Ord, &bound);

        let mut generics = ast.generics.clone();

        let where_clause = generics.make_where_clause();

        for where_predicate in bound {
            where_clause.predicates.push(where_predicate);
        }

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        token_stream.extend(quote! {
            #generated_impl_attributes
            impl #impl_generics ::core::cmp::Ord for #ident #ty_generics #where_clause {
                #[inline]
                fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                    #cmp_token_stream

                    ::core::cmp::Ordering::Equal
                }
            }
        });

        Ok(())
    }
}
