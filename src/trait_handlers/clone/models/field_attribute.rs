use syn::{Attribute, Meta, Path, Token, punctuated::Punctuated};

use crate::{common::path::meta_2_path, panic, supported_traits::Trait};

/// The parsed settings of a field-level `Clone` attribute.
pub(crate) struct FieldAttribute {
    pub(crate) method: Option<Path>,
}

/// Parses field-level `Clone` metas; the `enable_*` switches describe which parameters are allowed for the current shape of data.
pub(crate) struct FieldAttributeBuilder {
    pub(crate) enable_method: bool,
}

impl FieldAttributeBuilder {
    /// Parses one field-level `Clone` meta into a `FieldAttribute`, rejecting parameters that are not enabled here.
    pub(crate) fn build_from_clone_meta(&self, meta: &Meta) -> syn::Result<FieldAttribute> {
        debug_assert!(meta.path().is_ident("Clone"));

        let mut method = None;

        let correct_usage_for_clone_attribute = {
            let mut usage = vec![];

            if self.enable_method {
                usage.push(stringify!(#[educe(Clone(method(path_to_method)))]));
            }

            usage
        };

        match meta {
            Meta::Path(_) | Meta::NameValue(_) => {
                return Err(panic::attribute_incorrect_format(
                    meta.path().get_ident().unwrap(),
                    &correct_usage_for_clone_attribute,
                ));
            },
            Meta::List(list) => {
                let result =
                    list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

                let mut method_is_set = false;

                let mut handler = |meta: Meta| -> syn::Result<bool> {
                    if let Some(ident) = meta.path().get_ident()
                        && ident == "method"
                    {
                        if !self.enable_method {
                            return Ok(false);
                        }

                        let v = meta_2_path(&meta)?;

                        if method_is_set {
                            return Err(panic::parameter_reset(ident));
                        }

                        method_is_set = true;

                        method = Some(v);

                        return Ok(true);
                    }

                    Ok(false)
                };

                for p in result {
                    if !handler(p)? {
                        return Err(panic::attribute_incorrect_format(
                            meta.path().get_ident().unwrap(),
                            &correct_usage_for_clone_attribute,
                        ));
                    }
                }
            },
        }

        Ok(FieldAttribute {
            method,
        })
    }

    /// Scans the `#[educe(...)]` attributes of a field and parses its `Clone` meta if present.
    pub(crate) fn build_from_attributes(
        &self,
        attributes: &[Attribute],
        traits: &[Trait],
    ) -> syn::Result<FieldAttribute> {
        let mut output = None;

        for attribute in attributes.iter() {
            let path = attribute.path();

            if path.is_ident("educe")
                && let Meta::List(list) = &attribute.meta
            {
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

                    if t == Trait::Clone {
                        if output.is_some() {
                            return Err(panic::reuse_a_trait(path.get_ident().unwrap()));
                        }

                        output = Some(self.build_from_clone_meta(&meta)?);
                    }
                }
            }
        }

        Ok(output.unwrap_or(FieldAttribute {
            method: None
        }))
    }
}
