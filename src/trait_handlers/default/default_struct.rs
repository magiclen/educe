use syn::{Data, DeriveInput, Fields, Meta, Type};

use super::{
    TraitHandler,
    models::{FieldAttributeBuilder, TypeAttributeBuilder},
};
use crate::{
    Trait,
    common::{bound::BOUND_EXCEPTIONS_DEFAULT, ident_index::IdentOrIndex, quote_mixed},
    trait_handlers::TraitHandlerContext,
};

/// Generates the `Default` implementation for a struct.
pub(crate) struct DefaultStructHandler;

impl TraitHandler for DefaultStructHandler {
    fn trait_meta_handler<'a>(
        ast: &'a DeriveInput,
        _ctx: &mut TraitHandlerContext<'a>,
        token_stream: &mut proc_macro2::TokenStream,
        traits: &[Trait],
        meta: &Meta,
    ) -> syn::Result<()> {
        let generated_impl_attributes =
            crate::common::attributes::generated_impl_attributes(&ast.attrs);

        let type_attribute = TypeAttributeBuilder {
            enable_flag:       true,
            enable_new:        true,
            enable_expression: true,
            enable_bound:      true,
        }
        .build_from_default_meta(meta)?;

        let mut default_types: Vec<&Type> = Vec::new();

        let mut default_token_stream = proc_macro2::TokenStream::new();

        if let Data::Struct(data) = &ast.data {
            if let Some(expression) = type_attribute.expression {
                for field in data.fields.iter() {
                    let _ = FieldAttributeBuilder {
                        enable_flag:       false,
                        enable_expression: false,
                    }
                    .build_from_attributes(&field.attrs, traits, &field.ty)?;
                }

                default_token_stream.extend(quote_mixed!(#expression));
            } else {
                let mut fields_token_stream = proc_macro2::TokenStream::new();

                for (index, field) in data.fields.iter().enumerate() {
                    let field_attribute = FieldAttributeBuilder {
                        enable_flag:       false,
                        enable_expression: true,
                    }
                    .build_from_attributes(&field.attrs, traits, &field.ty)?;

                    let field_name =
                        IdentOrIndex::from_ident_with_index(field.ident.as_ref(), index);

                    let value = if let Some(expression) = field_attribute.expression {
                        quote_mixed!(#expression)
                    } else {
                        let ty = &field.ty;

                        default_types.push(ty);

                        quote_mixed!(<#ty as ::core::default::Default>::default())
                    };

                    fields_token_stream.extend(field_name.to_field(&value));
                }

                default_token_stream.extend(match &data.fields {
                    Fields::Unit => quote_mixed!(Self),
                    Fields::Named(_) => quote_mixed!(Self { #fields_token_stream }),
                    Fields::Unnamed(_) => quote_mixed!(Self ( #fields_token_stream )),
                });
            }
        }

        let ident = &ast.ident;

        let bound = type_attribute.bound.into_where_predicates_by_generic_parameters_check_types(
            &ast.generics.params,
            &syn::parse2(quote_mixed!(::core::default::Default)).unwrap(),
            &default_types,
            &ast.ident,
            &BOUND_EXCEPTIONS_DEFAULT,
        );

        let generics = crate::common::generics::with_predicates(ast.generics.clone(), bound);

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        token_stream.extend(quote_mixed! {
            #generated_impl_attributes
            impl #impl_generics ::core::default::Default for #ident #ty_generics #where_clause {
                #[inline]
                fn default() -> Self {
                    #default_token_stream
                }
            }
        });

        if type_attribute.new {
            // An inherent impl is not a trait impl, so it takes the lint attributes alone instead of the full set with `#[automatically_derived]`.
            let lint_attributes = crate::common::attributes::generated_lint_attributes(&ast.attrs);

            token_stream.extend(quote_mixed! {
                #lint_attributes
                impl #impl_generics #ident #ty_generics #where_clause {
                    /// Returns the "default value" for a type.
                    #[inline]
                    pub fn new() -> Self {
                        <Self as ::core::default::Default>::default()
                    }
                }
            });
        }

        Ok(())
    }
}
