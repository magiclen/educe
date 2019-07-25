use std::str::FromStr;

use super::super::{TraitHandler, create_path_string_from_lit_str};

use crate::Trait;
use crate::proc_macro2::TokenStream;
use crate::syn::{DeriveInput, Meta, NestedMeta, Lit, Data};
use crate::panic;

pub struct DebugUnionHandler;

impl TraitHandler for DebugUnionHandler {
    fn trait_meta_handler(ast: &DeriveInput, tokens: &mut TokenStream, traits: &[Trait], meta: &Meta) {
        let mut name: Option<Option<String>> = Some(None);

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
                                                        } else {
                                                            name = None;
                                                        }
                                                    }
                                                    Lit::Bool(s) => {
                                                        if name_is_set {
                                                            panic::reset_parameter(meta_name.as_str());
                                                        }

                                                        name_is_set = true;

                                                        if !s.value {
                                                            name = None;
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
                                                    name = Some(s);
                                                } else {
                                                    name = None;
                                                }
                                            }
                                            Lit::Bool(s) => {
                                                if name_is_set {
                                                    panic::reset_parameter(meta_name.as_str());
                                                }

                                                name_is_set = true;

                                                if !s.value {
                                                    name = None;
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
                                    name = Some(s);
                                } else {
                                    name = None;
                                }
                            }
                            Lit::Bool(s) => {
                                if name_is_set {
                                    panic::reset_parameter("name");
                                }

                                name_is_set = true;

                                if !s.value {
                                    name = None;
                                }
                            }
                            _ => panic::attribute_incorrect_format("Debug", &[stringify!(#[educe(Debug("new_name"))]), stringify!(#[educe(Debug(false))])])
                        }
                    }
                }
            }
            Meta::NameValue(named_value) => {
                let lit = &named_value.lit;

                match lit {
                    Lit::Str(s) => {
                        let s = create_path_string_from_lit_str(s);

                        if s.is_some() {
                            name = Some(s);
                        } else {
                            name = None;
                        }
                    }
                    __ => panic::attribute_incorrect_format("Debug", &[stringify!(#[educe(Debug = "new_name")])])
                }
            }
            Meta::Word(_) => ()
        }

        let mut builder_tokens = TokenStream::new();

        match name {
            Some(name) => {
                let name = match name {
                    Some(name) => name,
                    None => {
                        ast.ident.to_string()
                    }
                };

                builder_tokens.extend(quote!(let mut builder = formatter.debug_tuple(#name);));

                if let Data::Union(data) = &ast.data {
                    for field in data.fields.named.iter() {
                        for attr in field.attrs.iter() {
                            let attr_meta = attr.parse_meta().unwrap();

                            let attr_meta_name = attr_meta.name().to_string();

                            match attr_meta_name.as_str() {
                                "educe" => match attr_meta {
                                    Meta::List(list) => {
                                        for p in list.nested {
                                            match p {
                                                NestedMeta::Meta(meta) => {
                                                    let meta_name = meta.name().to_string();

                                                    let t = Trait::from_str(meta_name);

                                                    if let Err(_) = traits.binary_search(&t) {
                                                        panic::trait_not_used(t.as_str());
                                                    }

                                                    if t == Trait::Debug {
                                                        panic::attribute_incorrect_format_without_correct_usage("Debug");
                                                    }
                                                }
                                                _ => panic::attribute_incorrect_format("educe", &[stringify!(#[educe(Trait1, Trait2, ..., TraitN)])])
                                            }
                                        }
                                    }
                                    _ => panic::attribute_incorrect_format("educe", &[stringify!(#[educe(Trait1, Trait2, ..., TraitN)])])
                                }
                                _ => ()
                            }
                        }
                    }

                    builder_tokens.extend(TokenStream::from_str(&format!("
                        let size = core::mem::size_of::<Self>();

                        let mut data = alloc::vec::Vec::with_capacity(size);

                        unsafe {{
                            data.set_len(size);
                            core::ptr::copy(self as *const Self as *const u8, data.as_mut_ptr(), size);
                        }}

                        builder.field(&data);
                    ")).unwrap());
                }

                builder_tokens.extend(TokenStream::from_str("builder.finish()").unwrap());
            }
            None => {
                if let Data::Union(data) = &ast.data {
                    for field in data.fields.named.iter() {
                        for attr in field.attrs.iter() {
                            let attr_meta = attr.parse_meta().unwrap();

                            let attr_meta_name = attr_meta.name().to_string();

                            match attr_meta_name.as_str() {
                                "educe" => match attr_meta {
                                    Meta::List(list) => {
                                        for p in list.nested {
                                            match p {
                                                NestedMeta::Meta(meta) => {
                                                    let meta_name = meta.name().to_string();

                                                    let t = Trait::from_str(meta_name);

                                                    if let Err(_) = traits.binary_search(&t) {
                                                        panic::trait_not_used(t.as_str());
                                                    }

                                                    if t == Trait::Debug {
                                                        panic::attribute_incorrect_format_without_correct_usage("Debug");
                                                    }
                                                }
                                                _ => panic::attribute_incorrect_format("educe", &[stringify!(#[educe(Trait1, Trait2, ..., TraitN)])])
                                            }
                                        }
                                    }
                                    _ => panic::attribute_incorrect_format("educe", &[stringify!(#[educe(Trait1, Trait2, ..., TraitN)])])
                                }
                                _ => ()
                            }
                        }
                    }

                    builder_tokens.extend(TokenStream::from_str(&format!("
                        let size = core::mem::size_of::<Self>();
                        let data = unsafe {{ core::slice::from_raw_parts(self as *const Self as *const u8, size) }};

                        core::fmt::Debug::fmt(data, formatter)
                    ")).unwrap());
                }
            }
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