use quote::ToTokens;
use syn::{Attribute, Meta, NestedMeta};

use crate::{panic, Trait};

#[derive(Clone)]
pub struct TypeAttribute {
    pub flag: bool,
}

#[derive(Debug, Clone)]
pub struct TypeAttributeBuilder {
    pub enable_flag: bool,
}

impl TypeAttributeBuilder {
    #[allow(clippy::wrong_self_convention)]
    pub fn from_deref_mut_meta(&self, meta: &Meta) -> TypeAttribute {
        let correct_usage_for_deref_mut_attribute = {
            let mut usage = vec![];

            if self.enable_flag {
                usage.push(stringify!(#[educe(DerefMut)]));
            }

            usage
        };

        match meta {
            Meta::List(_) => panic::attribute_incorrect_format(
                "DerefMut",
                &correct_usage_for_deref_mut_attribute,
            ),
            Meta::NameValue(_) => panic::attribute_incorrect_format(
                "DerefMut",
                &correct_usage_for_deref_mut_attribute,
            ),
            Meta::Path(_) => {
                if !self.enable_flag {
                    panic::attribute_incorrect_format(
                        "DerefMut",
                        &correct_usage_for_deref_mut_attribute,
                    );
                }
            },
        }

        TypeAttribute {
            flag: true
        }
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_attributes(self, attributes: &[Attribute], traits: &[Trait]) -> TypeAttribute {
        let mut result = None;

        for attribute in attributes.iter() {
            if attribute.path.is_ident("educe") {
                let meta = attribute.parse_meta().unwrap();

                match meta {
                    Meta::List(list) => {
                        for p in list.nested.iter() {
                            match p {
                                NestedMeta::Meta(meta) => {
                                    let meta_name = meta.path().into_token_stream().to_string();

                                    let t = Trait::from_str(meta_name);

                                    if traits.binary_search(&t).is_err() {
                                        panic::trait_not_used(t);
                                    }

                                    if t == Trait::DerefMut {
                                        if result.is_some() {
                                            panic::reuse_a_trait(t);
                                        }

                                        result = Some(self.from_deref_mut_meta(meta));
                                    }
                                },
                                _ => panic::educe_format_incorrect(),
                            }
                        }
                    },
                    _ => panic::educe_format_incorrect(),
                }
            }
        }

        result.unwrap_or(TypeAttribute {
            flag: false
        })
    }
}
