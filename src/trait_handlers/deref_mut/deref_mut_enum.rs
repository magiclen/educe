use syn::{Data, DeriveInput, Field, Fields, Meta};

use super::{
    TraitHandler,
    models::{FieldAttributeBuilder, TypeAttributeBuilder},
};
use crate::{
    common::quote_mixed, panic, supported_traits::Trait, trait_handlers::TraitHandlerContext,
};

/// Generates the `DerefMut` implementation for an enum.
pub(crate) struct DerefMutEnumHandler;

impl TraitHandler for DerefMutEnumHandler {
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

        let _ = TypeAttributeBuilder {
            enable_flag: true
        }
        .build_from_deref_mut_meta(meta)?;

        let mut arms_token_stream = proc_macro2::TokenStream::new();
        let mut bindings = ctx.value_bindings(ast);
        let field_binding = bindings.binding("target");

        if let Data::Enum(data) = &ast.data {
            for variant in data.variants.iter() {
                let _ = TypeAttributeBuilder {
                    enable_flag: false
                }
                .build_from_attributes(&variant.attrs, traits)?;

                if let Fields::Unit = &variant.fields {
                    return Err(panic::trait_not_support_unit_variant(
                        meta.path().get_ident().unwrap(),
                        variant,
                    ));
                }

                let fields = &variant.fields;

                // With exactly one field, that field is the `DerefMut` target automatically; otherwise exactly one field has to be marked with `#[educe(DerefMut)]`.
                let (index, field) = if fields.len() == 1 {
                    let field = fields.into_iter().next().unwrap();

                    let _ = FieldAttributeBuilder {
                        enable_flag: true
                    }
                    .build_from_attributes(&field.attrs, traits)?;

                    (0usize, field)
                } else {
                    let mut deref_field: Option<(usize, &Field)> = None;

                    for (index, field) in variant.fields.iter().enumerate() {
                        let field_attribute = FieldAttributeBuilder {
                            enable_flag: true
                        }
                        .build_from_attributes(&field.attrs, traits)?;

                        if field_attribute.flag {
                            if deref_field.is_some() {
                                return Err(super::panic::multiple_deref_mut_fields_of_variant(
                                    field_attribute.span,
                                    variant,
                                ));
                            }

                            deref_field = Some((index, field));
                        }
                    }

                    if let Some(deref_field) = deref_field {
                        deref_field
                    } else {
                        return Err(super::panic::no_deref_mut_field_of_variant(variant));
                    }
                };

                let variant_ident = &variant.ident;
                let pattern = if let Some(field_name) = &field.ident {
                    quote_mixed!(Self::#variant_ident { #field_name: #field_binding, .. })
                } else {
                    let mut skipped = proc_macro2::TokenStream::new();
                    for _ in 0..index {
                        skipped.extend(quote_mixed!(_,));
                    }

                    quote_mixed!(Self::#variant_ident(#skipped #field_binding, ..))
                };
                arms_token_stream.extend(quote_mixed!(#pattern => #field_binding,));
            }

            if arms_token_stream.is_empty() {
                return Err(super::panic::no_deref_mut_field(meta));
            }
        }

        let ident = &ast.ident;

        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
        let aliases = bindings.aliases();

        token_stream.extend(quote_mixed! {
            #generated_impl_attributes
            impl #impl_generics ::core::ops::DerefMut for #ident #ty_generics #where_clause {
                #[inline]
                fn deref_mut(&mut self) -> &mut <Self as ::core::ops::Deref>::Target {
                    #aliases
                    match self {
                        #arms_token_stream
                    }
                }
            }
        });

        Ok(())
    }
}
