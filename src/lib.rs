extern crate proc_macro;
extern crate proc_macro2;

extern crate alloc;

extern crate syn;

#[macro_use]
extern crate quote;

mod support_traits;
mod trait_handlers;
mod panic;

use proc_macro2::TokenStream;
use syn::{DeriveInput, Meta, NestedMeta};

use support_traits::Trait;

use trait_handlers::TraitHandler;

use trait_handlers::DebugHandler;

fn derive_input_handler(ast: DeriveInput) -> TokenStream {
    let mut tokens = TokenStream::new();
    let mut traits = Vec::new();
    let mut metas = Vec::new();

    for attr in ast.attrs.iter() {
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

                                if traits.contains(&t) {
                                    panic::reuse_a_trait(t.as_str());
                                }

                                traits.push(t);
                                metas.push(meta);
                            }
                            NestedMeta::Literal(_) => panic::attribute_incorrect_format("educe", &[stringify!(#[educe(Trait1, Trait2, ..., TraitN)])])
                        }
                    }
                }
                _ => panic::attribute_incorrect_format("educe", &[stringify!(#[educe(Trait1, Trait2, ..., TraitN)])])
            }
            _ => ()
        }
    }

    traits.sort();

    if let Ok(index) = traits.binary_search(&Trait::Debug) {
        DebugHandler::trait_meta_handler(&ast, &mut tokens, &traits, &metas[index]);
    }

    if tokens.is_empty() {
        panic::derive_attribute_not_set_up_yet("Educe");
    }

    tokens
}

#[proc_macro_derive(Educe, attributes(educe))]
pub fn educe_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_input_handler(syn::parse(input).unwrap()).into()
}