use std::str::FromStr;

use super::super::TraitHandler;
use super::models::{TypeAttributeBuilder, TypeAttributeName};

use crate::Trait;
use crate::proc_macro2::TokenStream;
use crate::syn::{DeriveInput, Meta, NestedMeta, Data};
use crate::panic;

pub struct DebugUnionHandler;

impl TraitHandler for DebugUnionHandler {
    fn trait_meta_handler(ast: &DeriveInput, tokens: &mut TokenStream, traits: &[Trait], meta: &Meta) {
        let type_attribute = TypeAttributeBuilder {
            name: TypeAttributeName::Default,
            enable_name: true,
            named_field: false,
            enable_named_field: false,
        }.from_debug_meta(meta);

        let name = type_attribute.name.into_string_by_ident(&ast.ident);

        let mut builder_tokens = TokenStream::new();

        if name.is_empty() {
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
                                                    panic::attribute_incorrect_format("Debug", &[]);
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
                }

                builder_tokens.extend(TokenStream::from_str(&format!("
                    let size = core::mem::size_of::<Self>();
                    let data = unsafe {{ core::slice::from_raw_parts(self as *const Self as *const u8, size) }};

                    core::fmt::Debug::fmt(data, formatter)
                ")).unwrap());
            }
        } else {
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
                                                    panic::attribute_incorrect_format("Debug", &[]);
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