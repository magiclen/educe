use syn::{Data, DeriveInput, ExprPath, Fields, Meta, Type};

use super::{
    TraitHandler,
    models::{FieldAttributeBuilder, TypeAttributeBuilder},
};
use crate::{
    Trait,
    common::{bound::BOUND_EXCEPTIONS_HASH, ident_index::IdentOrIndex, quote_mixed},
    trait_handlers::TraitHandlerContext,
};

/// Generates the `Hash` implementation for an enum.
pub(crate) struct HashEnumHandler;

impl TraitHandler for HashEnumHandler {
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
                enable_flag: true, enable_unsafe: false, enable_bound: true
            }
            .build_from_hash_meta(meta)?;

        let mut hash_types: Vec<&Type> = Vec::new();

        let mut hash_token_stream = proc_macro2::TokenStream::new();

        let mut arms_token_stream = proc_macro2::TokenStream::new();

        if let Data::Enum(data) = &ast.data {
            let built_in_hash: ExprPath =
                syn::parse2(quote_mixed!(::core::hash::Hash::hash)).unwrap();

            for (variant_index, variant) in data.variants.iter().enumerate() {
                let _ = TypeAttributeBuilder {
                    enable_flag:   false,
                    enable_unsafe: false,
                    enable_bound:  false,
                }
                .build_from_attributes(&variant.attrs, traits)?;

                let variant_ident = &variant.ident;

                if let Fields::Unit = &variant.fields {
                    arms_token_stream.extend(quote_mixed! {
                        Self::#variant_ident => {
                            ::core::hash::Hash::hash(&#variant_index, state);
                        }
                    });

                    continue;
                }

                let mut pattern_token_stream = proc_macro2::TokenStream::new();
                let mut block_token_stream = proc_macro2::TokenStream::new();

                for (index, field) in variant.fields.iter().enumerate() {
                    let field_attribute = FieldAttributeBuilder {
                        enable_ignore: true,
                        enable_method: true,
                    }
                    .build_from_attributes(&field.attrs, traits)?;

                    let field_name =
                        IdentOrIndex::from_ident_with_index(field.ident.as_ref(), index);

                    if field_attribute.ignore {
                        pattern_token_stream.extend(field_name.to_field(&quote_mixed!(_)));

                        continue;
                    }

                    let field_name_var = field_name.to_binding("_v_");

                    pattern_token_stream
                        .extend(field_name.to_field(&quote_mixed!(#field_name_var)));

                    let hash = field_attribute.method.as_ref().unwrap_or_else(|| {
                        hash_types.push(&field.ty);
                        &built_in_hash
                    });

                    block_token_stream.extend(quote_mixed!( #hash(#field_name_var, state); ));
                }

                let pattern = if let Fields::Named(_) = &variant.fields {
                    quote_mixed!(Self::#variant_ident { #pattern_token_stream })
                } else {
                    quote_mixed!(Self::#variant_ident ( #pattern_token_stream ))
                };

                arms_token_stream.extend(quote_mixed! {
                    #pattern => {
                        ::core::hash::Hash::hash(&#variant_index, state);

                        #block_token_stream
                    }
                });
            }
        }

        if !arms_token_stream.is_empty() {
            hash_token_stream.extend(quote_mixed! {
                match self {
                    #arms_token_stream
                }
            });
        }

        let ident = &ast.ident;
        let hasher_ident = crate::common::generics::UsedIdents::new(ast).select("H");

        let bound = type_attribute.bound.into_where_predicates_by_generic_parameters_check_types(
            &ast.generics.params,
            &syn::parse2(quote_mixed!(::core::hash::Hash)).unwrap(),
            &hash_types,
            &ast.ident,
            &BOUND_EXCEPTIONS_HASH,
        );

        let generics = crate::common::generics::with_predicates(ast.generics.clone(), bound);

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        token_stream.extend(quote_mixed! {
            #generated_impl_attributes
            impl #impl_generics ::core::hash::Hash for #ident #ty_generics #where_clause {
                #[inline]
                fn hash<#hasher_ident: ::core::hash::Hasher>(&self, state: &mut #hasher_ident) {
                    #hash_token_stream
                }
            }
        });

        Ok(())
    }
}
