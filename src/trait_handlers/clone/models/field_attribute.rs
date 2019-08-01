use super::super::super::create_path_string_from_lit_str;

use crate::Trait;
use crate::syn::{Meta, NestedMeta, Lit, Attribute};
use crate::panic;

#[derive(Debug, Clone)]
pub struct FieldAttribute {
    pub clone_method: Option<String>,
    pub clone_trait: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FieldAttributeBuilder {
    pub enable_clone: bool,
}

impl FieldAttributeBuilder {
    pub fn from_clone_meta(&self, meta: &Meta) -> FieldAttribute {
        let mut clone_method = None;
        let mut clone_trait = None;

        let correct_usage_for_clone_attribute = {
            let usage = vec![];

            usage
        };

        let correct_usage_for_clone = {
            let usage = vec![stringify!(#[educe(Clone(clone(method = "path_to_method")))]), stringify!(#[educe(Clone(clone(trait = "path_to_trait")))]), stringify!(#[educe(Clone(clone(trait = "path_to_trait", method = "path_to_method_in_trait")))]), stringify!(#[educe(Clone(clone(method("path_to_method"))))]), stringify!(#[educe(Clone(clone(trait("path_to_trait"))))]), stringify!(#[educe(Clone(clone(trait("path_to_trait"), method("path_to_method_in_trait"))))]), stringify!(#[educe(Clone(clone = "path_to_method"))]), stringify!(#[educe(Clone(clone("path_to_method")))])];

            usage
        };

        match meta {
            Meta::List(list) => {
                for p in list.nested.iter() {
                    match p {
                        NestedMeta::Meta(meta) => {
                            let meta_name = meta.name().to_string();

                            match meta_name.as_str() {
                                "clone" | "impl" => {
                                    if !self.enable_clone {
                                        panic::unknown_parameter("Clone", meta_name.as_str());
                                    }

                                    match meta {
                                        Meta::List(list) => {
                                            for p in list.nested.iter() {
                                                match p {
                                                    NestedMeta::Meta(meta) => {
                                                        let meta_name = meta.name().to_string();

                                                        match meta_name.as_str() {
                                                            "method" => match meta {
                                                                Meta::List(list) => {
                                                                    for p in list.nested.iter() {
                                                                        match p {
                                                                            NestedMeta::Literal(lit) => match lit {
                                                                                Lit::Str(s) => {
                                                                                    if clone_method.is_some() {
                                                                                        panic::reset_parameter("clone_method");
                                                                                    }

                                                                                    let s = create_path_string_from_lit_str(s);

                                                                                    if let Some(s) = s {
                                                                                        clone_method = Some(s);
                                                                                    } else {
                                                                                        panic::empty_parameter("clone_method");
                                                                                    }
                                                                                }
                                                                                _ => panic::parameter_incorrect_format("clone_method", &correct_usage_for_clone)
                                                                            }
                                                                            _ => panic::parameter_incorrect_format("clone_method", &correct_usage_for_clone)
                                                                        }
                                                                    }
                                                                }
                                                                Meta::NameValue(named_value) => {
                                                                    let lit = &named_value.lit;

                                                                    match lit {
                                                                        Lit::Str(s) => {
                                                                            if clone_method.is_some() {
                                                                                panic::reset_parameter("clone_method");
                                                                            }

                                                                            let s = create_path_string_from_lit_str(s);

                                                                            if let Some(s) = s {
                                                                                clone_method = Some(s);
                                                                            } else {
                                                                                panic::empty_parameter("clone_method");
                                                                            }
                                                                        }
                                                                        _ => panic::parameter_incorrect_format("clone_method", &correct_usage_for_clone)
                                                                    }
                                                                }
                                                                _ => panic::parameter_incorrect_format("clone_method", &correct_usage_for_clone)
                                                            }
                                                            "trait" => match meta {
                                                                Meta::List(list) => {
                                                                    for p in list.nested.iter() {
                                                                        match p {
                                                                            NestedMeta::Literal(lit) => match lit {
                                                                                Lit::Str(s) => {
                                                                                    if clone_trait.is_some() {
                                                                                        panic::reset_parameter("clone_trait");
                                                                                    }

                                                                                    let s = create_path_string_from_lit_str(s);

                                                                                    if let Some(s) = s {
                                                                                        clone_trait = Some(s);
                                                                                    } else {
                                                                                        panic::empty_parameter("clone_trait");
                                                                                    }
                                                                                }
                                                                                _ => panic::parameter_incorrect_format("clone_trait", &correct_usage_for_clone)
                                                                            }
                                                                            _ => panic::parameter_incorrect_format("clone_trait", &correct_usage_for_clone)
                                                                        }
                                                                    }
                                                                }
                                                                Meta::NameValue(named_value) => {
                                                                    let lit = &named_value.lit;

                                                                    match lit {
                                                                        Lit::Str(s) => {
                                                                            if clone_trait.is_some() {
                                                                                panic::reset_parameter("clone_trait");
                                                                            }

                                                                            let s = create_path_string_from_lit_str(s);

                                                                            if let Some(s) = s {
                                                                                clone_trait = Some(s);
                                                                            } else {
                                                                                panic::empty_parameter("clone_trait");
                                                                            }
                                                                        }
                                                                        _ => panic::parameter_incorrect_format("clone_trait", &correct_usage_for_clone)
                                                                    }
                                                                }
                                                                _ => panic::parameter_incorrect_format("clone_trait", &correct_usage_for_clone)
                                                            }
                                                            _ => panic::parameter_incorrect_format(meta_name.as_str(), &correct_usage_for_clone)
                                                        }
                                                    }
                                                    NestedMeta::Literal(lit) => match lit {
                                                        Lit::Str(s) => {
                                                            if clone_method.is_some() {
                                                                panic::reset_parameter("clone_method");
                                                            }

                                                            let s = create_path_string_from_lit_str(s);

                                                            if let Some(s) = s {
                                                                clone_method = Some(s);
                                                            } else {
                                                                panic::empty_parameter("clone_method");
                                                            }
                                                        }
                                                        _ => panic::parameter_incorrect_format(meta_name.as_str(), &correct_usage_for_clone)
                                                    }
                                                }
                                            }
                                        }
                                        Meta::NameValue(named_value) => {
                                            let lit = &named_value.lit;

                                            match lit {
                                                Lit::Str(s) => {
                                                    if clone_method.is_some() {
                                                        panic::reset_parameter("clone_method");
                                                    }

                                                    let s = create_path_string_from_lit_str(s);

                                                    if let Some(s) = s {
                                                        clone_method = Some(s);
                                                    } else {
                                                        panic::empty_parameter("clone_method");
                                                    }
                                                }
                                                _ => panic::parameter_incorrect_format(meta_name.as_str(), &correct_usage_for_clone)
                                            }
                                        }
                                        _ => panic::parameter_incorrect_format(meta_name.as_str(), &correct_usage_for_clone)
                                    }
                                }
                                _ => panic::unknown_parameter("Clone", meta_name.as_str())
                            }
                        }
                        _ => panic::attribute_incorrect_format("Clone", &correct_usage_for_clone_attribute)
                    }
                }
            }
            _ => panic::attribute_incorrect_format("Clone", &correct_usage_for_clone_attribute)
        }

        if clone_trait.is_some() {
            if clone_method.is_none() {
                clone_method = Some("clone".to_string());
            }
        }

        FieldAttribute {
            clone_method,
            clone_trait,
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

                                    if t == Trait::Clone {
                                        if result.is_some() {
                                            panic::reuse_a_trait(t.as_str());
                                        }

                                        result = Some(self.from_clone_meta(&meta));
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

        result.unwrap_or(FieldAttribute {
            clone_method: None,
            clone_trait: None,
        })
    }
}