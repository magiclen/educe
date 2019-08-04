use super::super::super::{create_where_predicates_from_lit_str, create_where_predicates_from_generic_parameters};

use crate::Trait;
use crate::syn::{Meta, NestedMeta, Lit, Attribute, WherePredicate, GenericParam, punctuated::Punctuated, token::Comma};
use crate::panic;

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
            TypeAttributeBound::Auto => create_where_predicates_from_generic_parameters(params, &syn::parse(quote!(core::cmp::Ord).into()).unwrap()),
            TypeAttributeBound::Custom(where_predicates) => where_predicates
        }
    }
}

#[derive(Clone)]
pub struct TypeAttribute {
    pub flag: bool,
    pub bound: TypeAttributeBound,
    pub value: isize,
}

#[derive(Debug, Clone)]
pub struct TypeAttributeBuilder {
    pub enable_flag: bool,
    pub enable_bound: bool,
    pub value: isize,
    pub enable_value: bool,
}

impl TypeAttributeBuilder {
    pub fn from_ord_meta(&self, meta: &Meta) -> TypeAttribute {
        let mut flag = false;
        let mut bound = TypeAttributeBound::None;
        let mut value = self.value;

        let correct_usage_for_ord_attribute = {
            let mut usage = vec![];

            if self.enable_flag {
                usage.push(stringify!(#[educe(Ord)]));
            }

            usage
        };

        let correct_usage_for_bound = {
            let usage = vec![stringify!(#[educe(Ord(bound))]), stringify!(#[educe(Ord(bound = "where_predicates"))]), stringify!(#[educe(Ord(bound("where_predicates")))])];

            usage
        };

        let correct_usage_for_value = {
            let usage = vec![stringify!(#[educe(Ord(value = comparison_value))]), stringify!(#[educe(Ord(value(comparison_value)))])];

            usage
        };

        match meta {
            Meta::List(list) => {
                let mut bound_is_set = false;
                let mut value_is_set = false;

                for p in list.nested.iter() {
                    match p {
                        NestedMeta::Meta(meta) => {
                            let meta_name = meta.name().to_string();

                            match meta_name.as_str() {
                                "bound" => {
                                    if !self.enable_bound {
                                        panic::unknown_parameter("Ord", meta_name.as_str());
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
                                "value" => {
                                    if !self.enable_value {
                                        panic::unknown_parameter("Ord", meta_name.as_str());
                                    }

                                    match meta {
                                        Meta::List(list) => {
                                            for p in list.nested.iter() {
                                                match p {
                                                    NestedMeta::Literal(lit) => match lit {
                                                        Lit::Int(i) => {
                                                            if value_is_set {
                                                                panic::reset_parameter("value");
                                                            }

                                                            let i = i.value();

                                                            value_is_set = true;

                                                            if i > isize::max_value() as u64 {
                                                                value = isize::max_value();
                                                            } else {
                                                                value = i as isize;
                                                            }
                                                        }
                                                        _ => panic::parameter_incorrect_format(meta_name.as_str(), &correct_usage_for_value)
                                                    }
                                                    _ => panic::parameter_incorrect_format(meta_name.as_str(), &correct_usage_for_value)
                                                }
                                            }
                                        }
                                        Meta::NameValue(named_value) => {
                                            let lit = &named_value.lit;

                                            match lit {
                                                Lit::Int(i) => {
                                                    if value_is_set {
                                                        panic::reset_parameter("value");
                                                    }

                                                    let i = i.value();

                                                    value_is_set = true;

                                                    if i > isize::max_value() as u64 {
                                                        value = isize::max_value();
                                                    } else {
                                                        value = i as isize;
                                                    }
                                                }
                                                _ => panic::parameter_incorrect_format(meta_name.as_str(), &correct_usage_for_value)
                                            }
                                        }
                                        _ => panic::parameter_incorrect_format(meta_name.as_str(), &correct_usage_for_value)
                                    }
                                }
                                _ => panic::unknown_parameter("Ord", meta_name.as_str())
                            }
                        }
                        _ => panic::attribute_incorrect_format("Ord", &correct_usage_for_ord_attribute)
                    }
                }
            }
            Meta::NameValue(_) => panic::attribute_incorrect_format("Ord", &correct_usage_for_ord_attribute),
            Meta::Word(_) => {
                if !self.enable_flag {
                    panic::attribute_incorrect_format("Ord", &correct_usage_for_ord_attribute);
                }

                flag = true;
            }
        }

        TypeAttribute {
            flag,
            bound,
            value,
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

                                    if t == Trait::Ord {
                                        if result.is_some() {
                                            panic::reuse_a_trait(t.as_str());
                                        }

                                        result = Some(self.from_ord_meta(&meta));
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
            flag: false,
            bound: TypeAttributeBound::None,
            value: self.value,
        })
    }
}