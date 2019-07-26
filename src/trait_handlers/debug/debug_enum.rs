use std::str::FromStr;
use std::fmt::Write;

use super::super::TraitHandler;
use super::models::{TypeAttributeBuilder, TypeAttributeName};

use crate::Trait;
use crate::proc_macro2::TokenStream;
use crate::syn::{DeriveInput, Meta, Data, Fields};
use crate::panic;

pub struct DebugEnumHandler;

impl TraitHandler for DebugEnumHandler {
    fn trait_meta_handler(ast: &DeriveInput, tokens: &mut TokenStream, traits: &[Trait], meta: &Meta) {
        let type_attribute = TypeAttributeBuilder {
            name: TypeAttributeName::Disable,
            enable_name: true,
            named_field: false,
            enable_named_field: false,
        }.from_debug_meta(meta);

        let name = type_attribute.name.into_string_by_ident(&ast.ident);

        let mut builder_tokens = TokenStream::new();
        let mut has_variants = false;

        let mut match_tokens = String::from("match self { ");

        if let Data::Enum(data) = &ast.data {
            for variant in data.variants.iter() {
                let type_attribute = TypeAttributeBuilder {
                    name: TypeAttributeName::Default,
                    enable_name: true,
                    named_field: if let Fields::Named(_) = &variant.fields {
                        true
                    } else {
                        false
                    },
                    enable_named_field: true,
                }.from_attributes(&variant.attrs, traits);

                let variant_name = type_attribute.name.into_string_by_ident(&variant.ident);

                let named_field = type_attribute.named_field;

                let variant_ident = variant.ident.to_string();

                let variant_name = combine_names(&name, variant_name);

                match &variant.fields {
                    Fields::Unit => { // TODO Unit
                        if variant_name.is_empty() {
                            panic::unit_variant_need_name();
                        }

                        match_tokens.write_fmt(format_args!("Self::{variant_ident} => {{ formatter.write_str({variant_name:?}) }}", variant_ident = variant_ident, variant_name = variant_name)).unwrap();

                        has_variants = true;
                    }
                    Fields::Named(fields) => {  // TODO Struct
                    }
                    Fields::Unnamed(fields) => {  // TODO Tuple
                    }
                }
            }
        }

        match_tokens.push_str(" }");

        println!("{}", match_tokens);

        builder_tokens.extend(TokenStream::from_str(&match_tokens).unwrap());

        if name.is_empty() && !has_variants {
            panic::unit_enum_need_name();
        }

        let ident = &ast.ident;

        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

        let debug_impl = quote! {
            impl #impl_generics core::fmt::Debug for #ident #ty_generics #where_clause {
                fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                    #builder_tokens
                }
            }
        };

        tokens.extend(debug_impl);
    }
}

fn combine_names(name: &str, variant_name: String) -> String {
    if name.is_empty() {
        if variant_name.is_empty() {
            String::new()
        } else {
            variant_name
        }
    } else {
        let mut name = name.to_string();

        if !variant_name.is_empty() {
            if !variant_name.starts_with("::") {
                name.push_str("::");
            }

            name.push_str(&variant_name);
        }

        name
    }
}