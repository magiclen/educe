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
            return Err(crate::panic::union_without_unsafe(meta));
        }

        let name = type_attribute.name.to_ident_by_ident(&ast.ident);

        let mut builder_token_stream = proc_macro2::TokenStream::new();
        let mut bindings = ctx.value_bindings(ast);
        let formatter = bindings.binding("formatter");

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

            let data = bindings.binding("data");
            let bytes = union_bytes(&data, &quote_mixed!(self));

            if let Some(name) = name {
                let builder = bindings.binding("builder");
                builder_token_stream.extend(quote_mixed!(
                    let mut #builder = #formatter.debug_tuple(::core::stringify!(#name));

                    #bytes

                    #builder.field(&#data);

                    #builder.finish()
                ));
            } else {
                builder_token_stream.extend(quote_mixed!(
                    #bytes

                    ::core::fmt::Debug::fmt(#data, #formatter)
                ));
            }
        }

        let ident = &ast.ident;

        // Formatting the storage as bytes needs nothing from the field types, so the automatic bound stays empty and only an explicit one contributes predicates.
        let bound = type_attribute.bound.into_where_predicates_by_generic_parameters_check_types(
            &ast.generics,
            &syn::parse2(quote_mixed!(::core::fmt::Debug)).unwrap(),
            &[],
            &ast.ident,
            &BOUND_EXCEPTIONS_DEBUG,
        );

        let generics = crate::common::generics::with_predicates(ast.generics.clone(), bound);

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        token_stream.extend(bindings.wrap(quote_mixed! {
            #generated_impl_attributes
            impl #impl_generics ::core::fmt::Debug for #ident #ty_generics #where_clause {
                #[inline]
                fn fmt(&self, #formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    #builder_token_stream
                }
            }
        }));

        Ok(())
    }
}
