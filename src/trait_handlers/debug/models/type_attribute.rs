use super::super::super::{create_path_string_from_lit_str, create_where_predicates_from_lit_str, create_where_predicates_from_generic_parameters};

use crate::Trait;
use crate::syn::{Meta, NestedMeta, Lit, Ident, Attribute, WherePredicate, GenericParam, punctuated::Punctuated, token::Comma};
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
            TypeAttributeName::Disable => String::new(),
            TypeAttributeName::Default => ident.to_string(),
            TypeAttributeName::Custom(s) => s
        }
    }
}

#[derive(Clone)]
pub enum TypeAttributeBound {
    None,
    Auto,
    Custom(Punctuated<WherePredicate, Comma>),
}

impl TypeAttributeBound {
    pub fn into_punctuated_where_predicates_by_generic_parameters(self, params: &Punctuated<GenericParam, Comma>) -> Punctuated<WherePredicate, Comma> {
        match self {
            TypeAttributeBound::None => Punctuated::new(),
            TypeAttributeBound::Auto => create_where_predicates_from_generic_parameters(params, &syn::parse(quote!(core::fmt::Debug).into()).unwrap()),
            TypeAttributeBound::Custom(where_predicates) => where_predicates
        }
    }
}

#[derive(Clone)]
pub struct TypeAttribute {
    pub name: TypeAttributeName,
    pub named_field: bool,
    pub bound: TypeAttributeBound,
}

#[derive(Debug, Clone)]
pub struct TypeAttributeBuilder {
    pub name: TypeAttributeName,
    pub enable_name: bool,
    pub named_field: bool,
    pub enable_named_field: bool,
    pub enable_bound: bool,
}

impl TypeAttributeBuilder {
    pub fn from_debug_meta(&self, meta: &Meta) -> TypeAttribute {
        let mut name = self.name.clone();

        let mut named_field = self.named_field.clone();

        let mut bound = TypeAttributeBound::None;

        let correct_usage_for_debug_attribute = {
            let mut usage = vec![stringify!(#[educe(Debug)])];

            if self.enable_name {
                usage.push(stringify!(#[educe(Debug = "new_name")]));
                usage.push(stringify!(#[educe(Debug("new_name"))]));
            }

            if self.enable_bound {
                usage.push(stringify!(#[educe(Debug(ignore))]));
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

        let correct_usage_for_bound = {
            let usage = vec![stringify!(#[educe(Debug(bound))]), stringify!(#[educe(Debug(bound = "where_predicates"))]), stringify!(#[educe(Debug(bound("where_predicates")))])];

            usage
        };

        match meta {
            Meta::List(list) => {
                let mut name_is_set = false;
                let mut named_field_is_set = false;
                let mut bound_is_set = false;

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
                                "bound" => {
                                    if !self.enable_bound {
                                        panic::unknown_parameter("Debug", meta_name.as_str());
                                    }

                                    match meta {
                                        Meta::List(list) => {
                                            for p in list.nested.iter() {
                                                match p {
                                                    NestedMeta::Literal(lit) => match lit {
                                                        Lit::Str(s) => {
                                                            if bound_is_set {
                                                                panic::reset_parameter(meta_name.as_str());
                                                            }

                                                            bound_is_set = true;

                                                            let where_predicates = create_where_predicates_from_lit_str(s);

                                                            bound = match where_predicates {
                                                                Some(where_predicates) => TypeAttributeBound::Custom(where_predicates),
                                                                None => panic::empty_parameter(meta_name.as_str())
                                                            };
                                                        }
                                                        _ => panic::parameter_incorrect_format(meta_name.as_str(), &correct_usage_for_bound)
                                                    }
                                                    _ => panic::parameter_incorrect_format(meta_name.as_str(), &correct_usage_for_bound)
                                                }
                                            }
                                        }
                                        Meta::NameValue(named_value) => {
                                            let lit = &named_value.lit;

                                            match lit {
                                                Lit::Str(s) => {
                                                    if bound_is_set {
                                                        panic::reset_parameter(meta_name.as_str());
                                                    }

                                                    bound_is_set = true;

                                                    let where_predicates = create_where_predicates_from_lit_str(s);

                                                    bound = match where_predicates {
                                                        Some(where_predicates) => TypeAttributeBound::Custom(where_predicates),
                                                        None => panic::empty_parameter(meta_name.as_str())
                                                    };
                                                }
                                                _ => panic::parameter_incorrect_format(meta_name.as_str(), &correct_usage_for_bound)
                                            }
                                        }
                                        Meta::Word(_) => {
                                            if bound_is_set {
                                                panic::reset_parameter(meta_name.as_str());
                                            }

                                            bound_is_set = true;

                                            bound = TypeAttributeBound::Auto;
                                        }
                                    }
                                }
                                _ => panic::unknown_parameter("Debug", meta_name.as_str())
                            }
                        }
                        NestedMeta::Literal(lit) => {
                            match lit {
                                Lit::Str(s) => {
                                    if !self.enable_name {
                                        panic::attribute_incorrect_format("Debug", &correct_usage_for_debug_attribute)
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
                                _ => panic::attribute_incorrect_format("Debug", &correct_usage_for_debug_attribute)
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
                            panic::attribute_incorrect_format("Debug", &correct_usage_for_debug_attribute)
                        }

                        let s = create_path_string_from_lit_str(s);

                        name = match s {
                            Some(s) => TypeAttributeName::Custom(s),
                            None => TypeAttributeName::Disable
                        };
                    }
                    _ => panic::attribute_incorrect_format("Debug", &correct_usage_for_debug_attribute)
                }
            }
            Meta::Word(_) => ()
        }

        TypeAttribute {
            name,
            named_field,
            bound,
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
            bound: TypeAttributeBound::None,
        })
    }
}