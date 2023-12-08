mod models;

use models::{FieldAttributeBuilder, TypeAttributeBuilder};
use quote::quote;
use syn::{Data, DeriveInput, Meta};

use super::TraitHandler;
use crate::Trait;

pub(crate) struct CopyHandler;

impl TraitHandler for CopyHandler {
    #[inline]
    fn trait_meta_handler(
        ast: &mut DeriveInput,
        token_stream: &mut proc_macro2::TokenStream,
        traits: &[Trait],
        meta: &Meta,
    ) -> syn::Result<()> {
        let type_attribute = TypeAttributeBuilder {
            enable_flag: true, enable_bound: true
        }
        .build_from_copy_meta(meta)?;

        match &ast.data {
            Data::Struct(data) => {
                for field in data.fields.iter() {
                    let _ = FieldAttributeBuilder.build_from_attributes(&field.attrs, traits)?;
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
                    }
                }
            },
            Data::Union(data) => {
                for field in data.fields.named.iter() {
                    let _ = FieldAttributeBuilder.build_from_attributes(&field.attrs, traits)?;
                }
            },
        }

        let ident = &ast.ident;

        let bound = type_attribute.bound.into_where_predicates_by_generic_parameters(
            &ast.generics.params,
            &syn::parse2(quote!(::core::marker::Copy)).unwrap(),
        );

        let where_clause = ast.generics.make_where_clause();

        for where_predicate in bound {
            where_clause.predicates.push(where_predicate);
        }

        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

        token_stream.extend(quote! {
            impl #impl_generics ::core::marker::Copy for #ident #ty_generics #where_clause {
            }
        });

        Ok(())
    }
}
