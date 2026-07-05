use syn::{Attribute, Meta, Token, punctuated::Punctuated};

use crate::{panic, supported_traits::Trait};

/// The parsed settings of a field-level `Eq` attribute.
pub(crate) struct FieldAttribute;

/// Parses field-level `Eq` metas; the `enable_*` switches describe which parameters are allowed for the current shape of data.
pub(crate) struct FieldAttributeBuilder;

impl FieldAttributeBuilder {
    /// Parses one field-level `Eq` meta into a `FieldAttribute`, rejecting parameters that are not enabled here.
    pub(crate) fn build_from_eq_meta(&self, meta: &Meta) -> syn::Result<FieldAttribute> {
        debug_assert!(meta.path().is_ident("Eq"));

        Err(panic::attribute_incorrect_place(meta.path().get_ident().unwrap()))
    }

    /// Scans the `#[educe(...)]` attributes of a field and parses its `Eq` meta if present.
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

                    if t == Trait::Eq {
                        if output.is_some() {
                            return Err(panic::reuse_a_trait(path.get_ident().unwrap()));
                        }

                        output = Some(self.build_from_eq_meta(&meta)?);
                    }
                }
            }
        }

        Ok(output.unwrap_or(FieldAttribute))
    }
}
