use std::collections::HashMap;

use syn::{punctuated::Punctuated, Attribute, Meta, Path, Token};

use crate::{
    common::{path::meta_2_path, r#type::TypeWithPunctuatedMeta, tools::HashType},
    panic, Trait,
};

pub(crate) struct FieldAttribute {
    pub(crate) types: HashMap<HashType, Option<Path>>,
}

#[derive(Debug)]
pub(crate) struct FieldAttributeBuilder {
    pub(crate) enable_types: bool,
}

impl FieldAttributeBuilder {
    pub(crate) fn build_from_into_meta(&self, meta: &[Meta]) -> syn::Result<FieldAttribute> {
        debug_assert!(!meta.is_empty());

        let mut types = HashMap::new();

        for meta in meta {
            debug_assert!(meta.path().is_ident("Into"));

            let correct_usage_for_into_attribute = {
                let mut usage = vec![];

                if self.enable_types {
                    usage.push(stringify!(#[educe(Into(type))]));
                    usage.push(stringify!(#[educe(Into(type, method(path_to_method)))]));
                }

                usage
            };

            match meta {
                Meta::Path(_) | Meta::NameValue(_) => {
                    return Err(panic::attribute_incorrect_format(
                        meta.path().get_ident().unwrap(),
                        &correct_usage_for_into_attribute,
                    ));
                },
                Meta::List(list) => {
                    if !self.enable_types {
                        return Err(panic::attribute_incorrect_format(
                            meta.path().get_ident().unwrap(),
                            &correct_usage_for_into_attribute,
                        ));
                    }

                    let TypeWithPunctuatedMeta {
                        ty,
                        list: result,
                    } = list.parse_args()?;

                    let ty = super::super::common::to_hash_type(&ty);

                    let mut method = None;
                    let mut method_is_set = false;

                    let mut handler = |meta: Meta| -> syn::Result<bool> {
                        if let Some(ident) = meta.path().get_ident() {
                            if ident == "method" {
                                let v = meta_2_path(&meta)?;

                                if method_is_set {
                                    return Err(panic::parameter_reset(ident));
                                }

                                method_is_set = true;

                                method = Some(v);

                                return Ok(true);
                            }
                        }

                        Ok(false)
                    };

                    for p in result {
                        if !handler(p)? {
                            return Err(panic::attribute_incorrect_format(
                                meta.path().get_ident().unwrap(),
                                &correct_usage_for_into_attribute,
                            ));
                        }
                    }

                    if types.contains_key(&ty) {
                        return Err(super::super::panic::reset_a_type(&ty));
                    }

                    types.insert(ty, method);
                },
            }
        }

        Ok(FieldAttribute {
            types,
        })
    }

    pub(crate) fn build_from_attributes(
        &self,
        attributes: &[Attribute],
        traits: &[Trait],
    ) -> syn::Result<FieldAttribute> {
        let mut output: Option<FieldAttribute> = None;

        let mut v_meta = Vec::new();

        for attribute in attributes.iter() {
            let path = attribute.path();

            if path.is_ident("educe") {
                if let Meta::List(list) = &attribute.meta {
                    let result =
                        list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

                    for meta in result {
                        let path = meta.path();

                        let t = match Trait::from_path(path) {
                            Some(t) => t,
                            None => return Err(panic::unsupported_trait(meta.path())),
                        };

                        if !traits.contains(&t) {
                            return Err(panic::trait_not_used(path.get_ident().unwrap()));
                        }

                        if t == Trait::Into {
                            v_meta.push(meta);
                        }
                    }
                }
            }
        }

        if !v_meta.is_empty() {
            output = Some(self.build_from_into_meta(&v_meta)?);
        }

        Ok(output.unwrap_or_else(|| FieldAttribute {
            types: HashMap::new()
        }))
    }
}
