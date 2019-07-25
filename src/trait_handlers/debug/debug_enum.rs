use std::str::FromStr;
use std::fmt::Write;

use super::super::{TraitHandler, create_path_string_from_lit_str};

use crate::Trait;
use crate::proc_macro2::TokenStream;
use crate::syn::{DeriveInput, Meta, NestedMeta, Lit, Data, Fields, export::ToTokens};
use crate::panic;

pub struct DebugEnumHandler;

impl TraitHandler for DebugEnumHandler {
    fn trait_meta_handler(ast: &DeriveInput, tokens: &mut TokenStream, traits: &[Trait], meta: &Meta) {
        let mut name: Option<Option<String>> = None;

        match meta {
            Meta::List(list) => {
                let mut name_is_set = false;

                for p in list.nested.iter() {
                    match p {
                        NestedMeta::Meta(meta) => {
                            let meta_name = meta.name().to_string();

                            match meta_name.as_str() {
                                "name" | "rename" => match meta {
                                    Meta::List(list) => {
                                        for p in list.nested.iter() {
                                            match p {
                                                NestedMeta::Literal(lit) => match lit {
                                                    Lit::Str(s) => {
                                                        if name_is_set {
                                                            panic::reset_parameter(meta_name.as_str());
                                                        }

                                                        name_is_set = true;

                                                        let s = create_path_string_from_lit_str(s);

                                                        if s.is_some() {
                                                            name = Some(s);
                                                        }
                                                    }
                                                    Lit::Bool(s) => {
                                                        if name_is_set {
                                                            panic::reset_parameter(meta_name.as_str());
                                                        }

                                                        name_is_set = true;

                                                        if s.value {
                                                            name = Some(None);
                                                        }
                                                    }
                                                    _ => panic::parameter_incorrect_format(meta_name.as_str(), &[stringify!(#[educe(Debug(name("new_name")))]), stringify!(#[educe(Debug(name(true)))])])
                                                }
                                                _ => panic::parameter_incorrect_format(meta_name.as_str(), &[stringify!(#[educe(Debug(name("new_name")))]), stringify!(#[educe(Debug(name(true)))])])
                                            }
                                        }
                                    }
                                    Meta::NameValue(named_value) => {
                                        let lit = &named_value.lit;

                                        match lit {
                                            Lit::Str(s) => {
                                                if name_is_set {
                                                    panic::reset_parameter(meta_name.as_str());
                                                }

                                                name_is_set = true;

                                                let s = create_path_string_from_lit_str(s);

                                                if s.is_some() {
                                                    name = Some(s);
                                                }
                                            }
                                            Lit::Bool(s) => {
                                                if name_is_set {
                                                    panic::reset_parameter(meta_name.as_str());
                                                }

                                                name_is_set = true;

                                                if s.value {
                                                    name = Some(None);
                                                }
                                            }
                                            _ => panic::parameter_incorrect_format(meta_name.as_str(), &[stringify!(#[educe(Debug(name = "new_name"))]), stringify!(#[educe(Debug(name = true))])])
                                        }
                                    }
                                    _ => panic::parameter_incorrect_format(meta_name.as_str(), &[stringify!(#[educe(Debug(name("new_name")))]), stringify!(#[educe(Debug(name(true)))]), stringify!(#[educe(Debug(name = "new_name"))]), stringify!(#[educe(Debug(name = true))])])
                                }
                                _ => panic::unknown_parameter("Debug", meta_name.as_str())
                            }
                        }
                        NestedMeta::Literal(lit) => match lit {
                            Lit::Str(s) => {
                                if name_is_set {
                                    panic::reset_parameter("name");
                                }

                                name_is_set = true;

                                let s = create_path_string_from_lit_str(s);

                                if s.is_some() {
                                    name = Some(s);
                                }
                            }
                            _ => panic::attribute_incorrect_format("Debug", &[stringify!(#[educe(Debug("new_name"))])])
                        }
                    }
                }
            }
            Meta::NameValue(named_value) => {
                let lit = &named_value.lit;

                match lit {
                    Lit::Str(s) => {
                        let s = create_path_string_from_lit_str(s);

                        name = Some(s);
                    }
                    __ => panic::attribute_incorrect_format("Debug", &[stringify!(#[educe(Debug = "new_name")])])
                }
            }
            Meta::Word(_) => ()
        }

        let name = match name {
            Some(name) => {
                match name {
                    Some(name) => name,
                    None => {
                        ast.ident.to_string()
                    }
                }
            }
            None => String::new()
        };

        let mut builder_tokens = TokenStream::new();
        let mut has_variants = false;

        let mut match_tokens = String::from("match self { ");

        if let Data::Enum(data) = &ast.data {
            for variant in data.variants.iter() {
                let variant_ident = variant.ident.to_string();

                match &variant.fields {
                    Fields::Unit => { // TODO Unit
                        let mut variant_name: Option<Option<String>> = Some(None);

                        for attr in variant.attrs.iter() {
                            let attr_meta = attr.parse_meta().unwrap();

                            let attr_meta_name = attr_meta.name().to_string();

                            match attr_meta_name.as_str() {
                                "educe" => match attr_meta {
                                    Meta::List(list) => {
                                        let mut name_is_set = false;

                                        for p in list.nested.iter() {
                                            match p {
                                                NestedMeta::Meta(meta) => {
                                                    let meta_name = meta.name().to_string();

                                                    let t = Trait::from_str(meta_name);

                                                    if let Err(_) = traits.binary_search(&t) {
                                                        panic::trait_not_used(t.as_str());
                                                    }

                                                    if t == Trait::Debug {
                                                        match meta {
                                                            Meta::List(list) => {
                                                                for p in list.nested.iter() {
                                                                    match p {
                                                                        NestedMeta::Meta(meta) => {
                                                                            let meta_name = meta.name().to_string();

                                                                            match meta_name.as_str() {
                                                                                "name" | "rename" => match meta {
                                                                                    Meta::List(list) => {
                                                                                        for p in list.nested.iter() {
                                                                                            match p {
                                                                                                NestedMeta::Literal(lit) => match lit {
                                                                                                    Lit::Str(s) => {
                                                                                                        if name_is_set {
                                                                                                            panic::reset_parameter(meta_name.as_str());
                                                                                                        }

                                                                                                        name_is_set = true;

                                                                                                        let s = create_path_string_from_lit_str(s);

                                                                                                        if s.is_some() {
                                                                                                            variant_name = Some(s);
                                                                                                        } else {
                                                                                                            variant_name = None;
                                                                                                        }
                                                                                                    }
                                                                                                    Lit::Bool(s) => {
                                                                                                        if name_is_set {
                                                                                                            panic::reset_parameter(meta_name.as_str());
                                                                                                        }

                                                                                                        name_is_set = true;

                                                                                                        if !s.value {
                                                                                                            variant_name = None;
                                                                                                        }
                                                                                                    }
                                                                                                    _ => panic::parameter_incorrect_format(meta_name.as_str(), &[stringify!(#[educe(Debug(name("new_name")))]), stringify!(#[educe(Debug(name(false)))])])
                                                                                                }
                                                                                                _ => panic::parameter_incorrect_format(meta_name.as_str(), &[stringify!(#[educe(Debug(name("new_name")))]), stringify!(#[educe(Debug(name(false)))])])
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    Meta::NameValue(named_value) => {
                                                                                        let lit = &named_value.lit;

                                                                                        match lit {
                                                                                            Lit::Str(s) => {
                                                                                                if name_is_set {
                                                                                                    panic::reset_parameter(meta_name.as_str());
                                                                                                }

                                                                                                name_is_set = true;

                                                                                                let s = create_path_string_from_lit_str(s);

                                                                                                if s.is_some() {
                                                                                                    variant_name = Some(s);
                                                                                                } else {
                                                                                                    variant_name = None;
                                                                                                }
                                                                                            }
                                                                                            Lit::Bool(s) => {
                                                                                                if name_is_set {
                                                                                                    panic::reset_parameter(meta_name.as_str());
                                                                                                }

                                                                                                name_is_set = true;

                                                                                                if !s.value {
                                                                                                    variant_name = None;
                                                                                                }
                                                                                            }
                                                                                            _ => panic::parameter_incorrect_format(meta_name.as_str(), &[stringify!(#[educe(Debug(name = "new_name"))]), stringify!(#[educe(Debug(name = false))])])
                                                                                        }
                                                                                    }
                                                                                    _ => panic::parameter_incorrect_format(meta_name.as_str(), &[stringify!(#[educe(Debug(name("new_name")))]), stringify!(#[educe(Debug(name(false)))]), stringify!(#[educe(Debug(name = "new_name"))]), stringify!(#[educe(Debug(name = false))])])
                                                                                }
                                                                                _ => panic::unknown_parameter("Debug", meta_name.as_str())
                                                                            }
                                                                        }
                                                                        NestedMeta::Literal(lit) => match lit {
                                                                            Lit::Str(s) => {
                                                                                if name_is_set {
                                                                                    panic::reset_parameter("name");
                                                                                }

                                                                                name_is_set = true;

                                                                                let s = create_path_string_from_lit_str(s);

                                                                                if s.is_some() {
                                                                                    variant_name = Some(s);
                                                                                } else {
                                                                                    variant_name = None;
                                                                                }
                                                                            }
                                                                            _ => panic::attribute_incorrect_format("Debug", &[stringify!(#[educe(Debug("new_name"))])])
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            Meta::NameValue(named_value) => {
                                                                let lit = &named_value.lit;

                                                                match lit {
                                                                    Lit::Str(s) => {
                                                                        if name_is_set {
                                                                            panic::reset_parameter("name");
                                                                        }

                                                                        name_is_set = true;

                                                                        let s = create_path_string_from_lit_str(s);

                                                                        if s.is_some() {
                                                                            variant_name = Some(s);
                                                                        } else {
                                                                            variant_name = None;
                                                                        }
                                                                    }
                                                                    _ => panic::attribute_incorrect_format("Debug", &[stringify!(#[educe(Debug("new_name"))])])
                                                                }
                                                            }
                                                            _ => panic::attribute_incorrect_format_without_correct_usage("Debug")
                                                        }
                                                    }
                                                }
                                                _ => panic::educe_format_incorrect()
                                            }
                                        }
                                    }
                                    _ => panic::educe_format_incorrect()
                                }
                                _ => ()
                            }
                        }

                        let variant_name = combine_names(&name, variant_name, &variant_ident);

                        if variant_name.is_empty() {
                            panic::unit_variant_need_name();
                        }

                        match_tokens.write_fmt(format_args!("Self::{variant_ident} => {{ formatter.write_str({variant_name:?}) }}", variant_ident = variant_ident, variant_name = variant_name));

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

fn combine_names(name: &str, variant_name: Option<Option<String>>, variant_ident: &str) -> String {
    if name.is_empty() {
        match variant_name {
            Some(name) => {
                match name {
                    Some(name) => name,
                    None => {
                        variant_ident.to_string()
                    }
                }
            }
            None => String::new()
        }
    } else {
        let mut name = name.to_string();

        if let Some(variant_name) = variant_name {
            match variant_name {
                Some(variant_name) => {
                    if !variant_name.starts_with("::") {
                        name.push_str("::");
                    }

                    name.push_str(&variant_name);
                }
                None => {
                    name.push_str("::");

                    name.push_str(variant_ident);
                }
            }
        }

        name
    }
}