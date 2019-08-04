use super::super::super::create_path_string_from_lit_str;

use crate::panic;
use crate::syn::{Attribute, Lit, Meta, NestedMeta};
use crate::Trait;

#[derive(Debug, Clone)]
pub struct FieldAttribute {
    pub ignore: bool,
    pub hash_method: Option<String>,
    pub hash_trait: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FieldAttributeBuilder {
    pub enable_ignore: bool,
    pub enable_impl: bool,
}

impl FieldAttributeBuilder {
    pub fn from_hash_meta(&self, meta: &Meta) -> FieldAttribute {
        let mut ignore = false;

        let mut hash_method = None;
        let mut hash_trait = None;

        let correct_usage_for_hash_attribute = {
            let mut usage = vec![];

            if self.enable_ignore {
                usage.push(stringify!(#[educe(Hash = false)]));
                usage.push(stringify!(#[educe(Hash(false))]));
            }

            usage
        };

        let correct_usage_for_ignore = {
            let usage = vec![stringify!(#[educe(Hash(ignore))])];

            usage
        };

        let correct_usage_for_impl = {
            let usage = vec![
                stringify!(#[educe(Hash(method = "path_to_method"))]),
                stringify!(#[educe(Hash(trait = "path_to_trait"))]),
                stringify!(#[educe(Hash(trait = "path_to_trait", method = "path_to_method_in_trait"))]),
                stringify!(#[educe(Hash(method("path_to_method")))]),
                stringify!(#[educe(Hash(trait("path_to_trait")))]),
                stringify!(#[educe(Hash(trait("path_to_trait"), method("path_to_method_in_trait")))]),
            ];

            usage
        };

        match meta {
            Meta::List(list) => {
                let mut ignore_is_set = false;

                for p in list.nested.iter() {
                    match p {
                        NestedMeta::Meta(meta) => {
                            let meta_name = meta.name().to_string();

                            match meta_name.as_str() {
                                "ignore" => {
                                    if !self.enable_ignore {
                                        panic::unknown_parameter("Hash", meta_name.as_str());
                                    }

                                    match meta {
                                        Meta::Word(_) => {
                                            if ignore_is_set {
                                                panic::reset_parameter(meta_name.as_str());
                                            }

                                            ignore_is_set = true;

                                            ignore = true;
                                        }
                                        _ => panic::parameter_incorrect_format(
                                            meta_name.as_str(),
                                            &correct_usage_for_ignore,
                                        ),
                                    }
                                }
                                "method" => {
                                    if !self.enable_impl {
                                        panic::unknown_parameter("Hash", meta_name.as_str());
                                    }

                                    match meta {
                                        Meta::List(list) => {
                                            for p in list.nested.iter() {
                                                match p {
                                                    NestedMeta::Literal(lit) => match lit {
                                                        Lit::Str(s) => {
                                                            if hash_method.is_some() {
                                                                panic::reset_parameter(
                                                                    meta_name.as_str(),
                                                                );
                                                            }

                                                            let s =
                                                                create_path_string_from_lit_str(s);

                                                            if let Some(s) = s {
                                                                hash_method = Some(s);
                                                            } else {
                                                                panic::empty_parameter(
                                                                    meta_name.as_str(),
                                                                );
                                                            }
                                                        }
                                                        _ => panic::parameter_incorrect_format(
                                                            meta_name.as_str(),
                                                            &correct_usage_for_impl,
                                                        ),
                                                    },
                                                    _ => panic::parameter_incorrect_format(
                                                        meta_name.as_str(),
                                                        &correct_usage_for_impl,
                                                    ),
                                                }
                                            }
                                        }
                                        Meta::NameValue(named_value) => {
                                            let lit = &named_value.lit;

                                            match lit {
                                                Lit::Str(s) => {
                                                    if hash_method.is_some() {
                                                        panic::reset_parameter(meta_name.as_str());
                                                    }

                                                    let s = create_path_string_from_lit_str(s);

                                                    if let Some(s) = s {
                                                        hash_method = Some(s);
                                                    } else {
                                                        panic::empty_parameter(meta_name.as_str());
                                                    }
                                                }
                                                _ => panic::parameter_incorrect_format(
                                                    meta_name.as_str(),
                                                    &correct_usage_for_impl,
                                                ),
                                            }
                                        }
                                        _ => panic::parameter_incorrect_format(
                                            meta_name.as_str(),
                                            &correct_usage_for_impl,
                                        ),
                                    }
                                }
                                "trait" => {
                                    if !self.enable_impl {
                                        panic::unknown_parameter("Hash", meta_name.as_str());
                                    }

                                    match meta {
                                        Meta::List(list) => {
                                            for p in list.nested.iter() {
                                                match p {
                                                    NestedMeta::Literal(lit) => match lit {
                                                        Lit::Str(s) => {
                                                            if hash_trait.is_some() {
                                                                panic::reset_parameter(
                                                                    meta_name.as_str(),
                                                                );
                                                            }

                                                            let s =
                                                                create_path_string_from_lit_str(s);

                                                            if let Some(s) = s {
                                                                hash_trait = Some(s);
                                                            } else {
                                                                panic::empty_parameter(
                                                                    meta_name.as_str(),
                                                                );
                                                            }
                                                        }
                                                        _ => panic::parameter_incorrect_format(
                                                            meta_name.as_str(),
                                                            &correct_usage_for_impl,
                                                        ),
                                                    },
                                                    _ => panic::parameter_incorrect_format(
                                                        meta_name.as_str(),
                                                        &correct_usage_for_impl,
                                                    ),
                                                }
                                            }
                                        }
                                        Meta::NameValue(named_value) => {
                                            let lit = &named_value.lit;

                                            match lit {
                                                Lit::Str(s) => {
                                                    if hash_trait.is_some() {
                                                        panic::reset_parameter(meta_name.as_str());
                                                    }

                                                    let s = create_path_string_from_lit_str(s);

                                                    if let Some(s) = s {
                                                        hash_trait = Some(s);
                                                    } else {
                                                        panic::empty_parameter(meta_name.as_str());
                                                    }
                                                }
                                                _ => panic::parameter_incorrect_format(
                                                    meta_name.as_str(),
                                                    &correct_usage_for_impl,
                                                ),
                                            }
                                        }
                                        _ => panic::parameter_incorrect_format(
                                            meta_name.as_str(),
                                            &correct_usage_for_impl,
                                        ),
                                    }
                                }
                                _ => panic::unknown_parameter("Hash", meta_name.as_str()),
                            }
                        }
                        _ => panic::attribute_incorrect_format(
                            "Hash",
                            &correct_usage_for_hash_attribute,
                        ),
                    }
                }
            }
            _ => panic::attribute_incorrect_format("Hash", &correct_usage_for_hash_attribute),
        }

        if hash_trait.is_some() {
            if hash_method.is_none() {
                hash_method = Some("hash".to_string());
            }
        }

        FieldAttribute {
            ignore,
            hash_method,
            hash_trait,
        }
    }

    pub fn from_attributes(self, attributes: &[Attribute], traits: &[Trait]) -> FieldAttribute {
        let mut result = None;

        for attribute in attributes.iter() {
            let meta = attribute.parse_meta().unwrap();

            let meta_name = meta.name().to_string();

            match meta_name.as_str() {
                "educe" => match meta {
                    Meta::List(list) => {
                        for p in list.nested.iter() {
                            match p {
                                NestedMeta::Meta(meta) => {
                                    let meta_name = meta.name().to_string();

                                    let t = Trait::from_str(meta_name);

                                    if let Err(_) = traits.binary_search(&t) {
                                        panic::trait_not_used(t.as_str());
                                    }

                                    if t == Trait::Hash {
                                        if result.is_some() {
                                            panic::reuse_a_trait(t.as_str());
                                        }

                                        result = Some(self.from_hash_meta(&meta));
                                    }
                                }
                                _ => panic::educe_format_incorrect(),
                            }
                        }
                    }
                    _ => panic::educe_format_incorrect(),
                },
                _ => (),
            }
        }

        result.unwrap_or(FieldAttribute {
            ignore: false,
            hash_method: None,
            hash_trait: None,
        })
    }
}
