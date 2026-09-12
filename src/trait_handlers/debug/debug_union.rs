use proc_macro2::{Ident, Span};
use syn::{Data, DeriveInput, Meta};

use super::{
    TraitHandler,
    models::{FieldAttributeBuilder, FieldName, TypeAttributeBuilder, TypeName},
};
use crate::{
    common::{bound::BOUND_EXCEPTIONS_DEBUG, quote_mixed, union::union_bytes},
    supported_traits::Trait,
    trait_handlers::TraitHandlerContext,
};

/// Generates the `Debug` implementation for a union.
pub(crate) struct DebugUnionHandler;

impl TraitHandler for DebugUnionHandler {
    fn trait_meta_handler(
        ast: &DeriveInput,
        _ctx: &mut TraitHandlerContext,
        token_stream: &mut proc_macro2::TokenStream,
        traits: &[Trait],
        meta: &Meta,
    ) -> syn::Result<()> {
        let generated_impl_attributes =
            crate::common::attributes::generated_impl_attributes(&ast.attrs);

        let type_attribute = TypeAttributeBuilder {
            enable_flag:        true,
            enable_unsafe:      true,
            enable_name:        true,
            enable_named_field: false,
            enable_bound:       true,
            name:               TypeName::Default,
            named_field:        false,
        }
        .build_from_debug_meta(meta)?;

        if !type_attribute.has_unsafe {
            return Err(super::panic::union_without_unsafe(meta));
        }

        let name = type_attribute.name.to_ident_by_ident(&ast.ident);

        let mut builder_token_stream = proc_macro2::TokenStream::new();

        if let Data::Union(data) = &ast.data {
            for field in data.fields.named.iter() {
                let _ = FieldAttributeBuilder {
                    enable_name:   false,
                    enable_ignore: false,
                    enable_method: false,
                    name:          FieldName::Default,
                }
                .build_from_attributes(&field.attrs, traits)?;
            }

            let data = Ident::new("data", Span::mixed_site());
            let bytes = union_bytes(&data, &quote_mixed!(self));

            if let Some(name) = name {
                builder_token_stream.extend(quote_mixed!(
                    let mut builder = f.debug_tuple(stringify!(#name));

                    #bytes

                    builder.field(&#data);

                    builder.finish()
                ));
            } else {
                builder_token_stream.extend(quote_mixed!(
                    #bytes

                    ::core::fmt::Debug::fmt(#data, f)
                ));
            }
        }

        let ident = &ast.ident;

        // Formatting the storage as bytes needs nothing from the field types, so the automatic bound stays empty and only an explicit one contributes predicates.
        let bound = type_attribute.bound.into_where_predicates_by_generic_parameters_check_types(
            &ast.generics.params,
            &syn::parse2(quote_mixed!(::core::fmt::Debug)).unwrap(),
            &[],
            &ast.ident,
            &BOUND_EXCEPTIONS_DEBUG,
        );

        let mut generics = ast.generics.clone();

        let where_clause = generics.make_where_clause();

        for where_predicate in bound {
            where_clause.predicates.push(where_predicate);
        }

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        token_stream.extend(quote_mixed! {
            #generated_impl_attributes
            impl #impl_generics ::core::fmt::Debug for #ident #ty_generics #where_clause {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    #builder_token_stream
                }
            }
        });

        Ok(())
    }
}
