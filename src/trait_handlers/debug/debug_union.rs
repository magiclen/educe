use super::super::TraitHandler;
use super::models::{TypeAttribute, TypeAttributeBuilder, TypeAttributeName};

use crate::Trait;
use crate::proc_macro2::TokenStream;
use crate::syn::{DeriveInput, Meta, NestedMeta, Data, Generics};
use crate::panic;

pub struct DebugUnionHandler;

impl TraitHandler for DebugUnionHandler {
    fn trait_meta_handler(ast: &DeriveInput, tokens: &mut TokenStream, traits: &[Trait], meta: &Meta) {
        let type_attribute = TypeAttributeBuilder {
            name: TypeAttributeName::Default,
            enable_name: true,
            named_field: false,
            enable_named_field: false,
            enable_bound: true,
        }.from_debug_meta(meta);

        let name = type_attribute.name.into_string_by_ident(&ast.ident);

        let bound = TypeAttribute::make_bound(type_attribute.bound, &ast.generics.params);

        let mut builder_tokens = TokenStream::new();

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

            if name.is_empty() {
                builder_tokens.extend(quote!(
                    let size = core::mem::size_of::<Self>();
                    let data = unsafe {{ core::slice::from_raw_parts(self as *const Self as *const u8, size) }};

                    core::fmt::Debug::fmt(data, formatter)
                ));
            } else {
                builder_tokens.extend(quote!(
                    let mut builder = formatter.debug_tuple(#name);

                    let size = core::mem::size_of::<Self>();

                    let data = unsafe {{ core::slice::from_raw_parts(self as *const Self as *const u8, size) }};

                    builder.field(&data);

                    builder.finish()
                ));
            }
        }

        let ident = &ast.ident;

        let mut generics_cloned: Generics = ast.generics.clone();

        let where_clause = generics_cloned.make_where_clause();

        where_clause.predicates.extend(bound.iter().cloned());

        let (impl_generics, ty_generics, where_clause) = generics_cloned.split_for_impl();

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