use super::super::super::create_path_string_from_lit_str;

use crate::Trait;
use crate::syn::{Meta, NestedMeta, Lit, Ident, Attribute};
use crate::panic;

#[derive(Debug, Clone)]
pub enum TypeAttributeName {
    Disable,
    Default,
    Custom(String),
}

impl TypeAttributeName {
    pub fn into_string_by_ident(self, ident: &Ident) -> String {
        match self {
            Self::Disable => String::new(),
            Self::Default => ident.to_string(),
            Self::Custom(s) => s
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeAttribute {
    pub name: TypeAttributeName,
    pub named_field: bool,
}

#[derive(Debug, Clone)]
pub struct TypeAttributeBuilder {
    pub name: TypeAttributeName,
    pub enable_name: bool,
    pub named_field: bool,
    pub enable_named_field: bool,
}

impl TypeAttributeBuilder {
    pub fn from_debug_meta(&self, meta: &Meta) -> TypeAttribute {
        let mut name = self.name.clone();

        let mut named_field = self.named_field.clone();

        let correct_usage_for_debug = {
            let mut usage = vec![];

            if self.enable_name {
                usage.push(stringify!(#[educe(Debug = "new_name")]));
                usage.push(stringify!(#[educe(Debug("new_name"))]));
            }

            usage
        };

        let correct_usage_for_name = {
            let mut usage = vec![stringify!(#[educe(Debug(name = "new_name"))]), stringify!(#[educe(Debug(name("new_name")))])];

            if let TypeAttributeName::Disable = &name {
                usage.push(stringify!(#[educe(Debug(name = true))]));
                usage.push(stringify!(#[educe(Debug(name(true)))]));
            } else {
                usage.push(stringify!(#[educe(Debug(name = false))]));
                usage.push(stringify!(#[educe(Debug(name(false)))]));
            }

            usage
        };

        let correct_usage_for_named_field = {
            let mut usage = vec![];

            if !self.named_field {
                usage.push(stringify!(#[educe(Debug(named_field = true))]));
                usage.push(stringify!(#[educe(Debug(named_field(true)))]));
            } else {
                usage.push(stringify!(#[educe(Debug(named_field = false))]));
                usage.push(stringify!(#[educe(Debug(named_field(false)))]));
            }

            usage
        };

        match meta {
            Meta::List(list) => {
                let mut name_is_set = false;
                let mut named_field_is_set = false;

                for p in list.nested.iter() {
                    match p {
                        NestedMeta::Meta(meta) => {
                            let meta_name = meta.name().to_string();

                            match meta_name.as_str() {
                                "name" | "rename" => {
                                    if !self.enable_name {
                                        panic::unknown_parameter("Debug", meta_name.as_str());
                                    }

                                    match meta {
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

                                                            name = match s {
                                                                Some(s) => TypeAttributeName::Custom(s),
                                                                None => TypeAttributeName::Disable
                                                            };
                                                        }
                                                        Lit::Bool(s) => {
                                                            if name_is_set {
                                                                panic::reset_parameter(meta_name.as_str());
                                                            }

                                                            name_is_set = true;

                                                            if s.value {
                                                                name = TypeAttributeName::Default;
                                                            } else {
                                                                name = TypeAttributeName::Disable;
                                                            }
                                                        }
                                                        _ => panic::parameter_incorrect_format(meta_name.as_str(), &correct_usage_for_name)
                                                    }
                                                    _ => panic::parameter_incorrect_format(meta_name.as_str(), &correct_usage_for_name)
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

                                                    name = match s {
                                                        Some(s) => TypeAttributeName::Custom(s),
                                                        None => TypeAttributeName::Disable
                                                    };
                                                }
                                                Lit::Bool(s) => {
                                                    if name_is_set {
                                                        panic::reset_parameter(meta_name.as_str());
                                                    }

                                                    name_is_set = true;

                                                    if s.value {
                                                        name = TypeAttributeName::Default;
                                                    } else {
                                                        name = TypeAttributeName::Disable;
                                                    }
                                                }
                                                _ => panic::parameter_incorrect_format(meta_name.as_str(), &correct_usage_for_name)
                                            }
                                        }
                                        _ => panic::parameter_incorrect_format(meta_name.as_str(), &correct_usage_for_name)
                                    }
                                }
                                "named_field" => {
                                    if !self.enable_named_field {
                                        panic::unknown_parameter("Debug", meta_name.as_str());
                                    }

                                    match meta {
                                        Meta::List(list) => {
                                            for p in list.nested.iter() {
                                                match p {
                                                    NestedMeta::Literal(lit) => match lit {
                                                        Lit::Bool(s) => {
                                                            if named_field_is_set {
                                                                panic::reset_parameter(meta_name.as_str());
                                                            }

                                                            named_field_is_set = true;

                                                            named_field = s.value;
                                                        }
                                                        _ => panic::parameter_incorrect_format(meta_name.as_str(), &correct_usage_for_named_field)
                                                    }
                                                    _ => panic::parameter_incorrect_format(meta_name.as_str(), &correct_usage_for_named_field)
                                                }
                                            }
                                        }
                                        Meta::NameValue(named_value) => {
                                            let lit = &named_value.lit;

                                            match lit {
                                                Lit::Bool(s) => {
                                                    if named_field_is_set {
                                                        panic::reset_parameter(meta_name.as_str());
                                                    }

                                                    named_field_is_set = true;

                                                    named_field = s.value;
                                                }
                                                _ => panic::parameter_incorrect_format(meta_name.as_str(), &correct_usage_for_named_field)
                                            }
                                        }
                                        _ => panic::parameter_incorrect_format(meta_name.as_str(), &correct_usage_for_named_field)
                                    }
                                }
                                _ => panic::unknown_parameter("Debug", meta_name.as_str())
                            }
                        }
                        NestedMeta::Literal(lit) => {
                            match lit {
                                Lit::Str(s) => {
                                    if !self.enable_name {
                                        panic::attribute_incorrect_format("Debug", &correct_usage_for_debug)
                                    }

                                    if name_is_set {
                                        panic::reset_parameter("name");
                                    }

                                    name_is_set = true;

                                    let s = create_path_string_from_lit_str(s);

                                    name = match s {
                                        Some(s) => TypeAttributeName::Custom(s),
                                        None => TypeAttributeName::Disable
                                    };
                                }
                                _ => panic::attribute_incorrect_format("Debug", &correct_usage_for_debug)
                            }
                        }
                    }
                }
            }
            Meta::NameValue(named_value) => {
                let lit = &named_value.lit;

                match lit {
                    Lit::Str(s) => {
                        if !self.enable_name {
                            panic::attribute_incorrect_format("Debug", &correct_usage_for_debug)
                        }

                        let s = create_path_string_from_lit_str(s);

                        name = match s {
                            Some(s) => TypeAttributeName::Custom(s),
                            None => TypeAttributeName::Disable
                        };
                    }
                    _ => panic::attribute_incorrect_format("Debug", &correct_usage_for_debug)
                }
            }
            Meta::Word(_) => ()
        }

        TypeAttribute {
            name,
            named_field,
        }
    }

    pub fn from_attributes(self, attributes: &[Attribute], traits: &[Trait]) -> TypeAttribute {
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

                                    if t == Trait::Debug {
                                        if result.is_some() {
                                            panic::reuse_a_trait(t.as_str());
                                        }

                                        result = Some(self.from_debug_meta(&meta));
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

        result.unwrap_or(TypeAttribute {
            name: self.name,
            named_field: self.named_field,
        })
    }
}