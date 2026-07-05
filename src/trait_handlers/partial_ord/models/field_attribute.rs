use proc_macro2::Span;
use syn::{Attribute, Meta, Path, Token, punctuated::Punctuated, spanned::Spanned};

use crate::{
    common::{
        ident_bool::{meta_2_bool_allow_path, meta_name_value_2_bool},
        int::meta_2_isize,
        path::meta_2_path,
    },
    panic,
    supported_traits::Trait,
};

/// The parsed settings of a field-level `PartialOrd` attribute.
pub(crate) struct FieldAttribute {
    pub(crate) ignore:                  bool,
    pub(crate) method:                  Option<Path>,
    /// Whether the method comes from a fallback `Ord` field attribute; such a method returns `Ordering` instead of `Option<Ordering>`, so the generated `partial_cmp` has to wrap its result in `Some`.
    pub(crate) method_returns_ordering: bool,
    pub(crate) rank:                    isize,
    pub(crate) rank_span:               Option<Span>,
}

/// Parses field-level `PartialOrd` metas; the `enable_*` switches describe which parameters are allowed for the current shape of data.
pub(crate) struct FieldAttributeBuilder {
    pub(crate) enable_ignore: bool,
    pub(crate) enable_method: bool,
    pub(crate) enable_rank:   bool,
    pub(crate) rank:          isize,
}

impl FieldAttributeBuilder {
    /// Parses one field-level `PartialOrd` meta into a `FieldAttribute`, rejecting parameters that are not enabled here.
    pub(crate) fn build_from_partial_ord_meta(&self, meta: &Meta) -> syn::Result<FieldAttribute> {
        // An `Ord` meta is also accepted here because `build_from_attributes` may fall back to the `Ord` field attribute when `Ord` is derived together.
        debug_assert!(meta.path().is_ident("PartialOrd") || meta.path().is_ident("Ord"));

        let mut ignore = false;
        let mut method = None;
        let mut rank = self.rank;
        let mut rank_span = None;

        let correct_usage_for_partial_eq_attribute = {
            let mut usage = vec![];

            if self.enable_ignore {
                usage.push(stringify!(#[educe(PartialOrd = false)]));
                usage.push(stringify!(#[educe(PartialOrd(ignore))]));
            }

            if self.enable_method {
                usage.push(stringify!(#[educe(PartialOrd(method(path_to_method)))]));
            }

            if self.enable_rank {
                usage.push(stringify!(#[educe(PartialOrd(rank = integer))]));
            }

            usage
        };

        match meta {
            Meta::Path(_) => {
                return Err(panic::attribute_incorrect_format(
                    meta.path().get_ident().unwrap(),
                    &correct_usage_for_partial_eq_attribute,
                ));
            },
            Meta::NameValue(name_value) => {
                if self.enable_ignore {
                    ignore = !meta_name_value_2_bool(name_value)?;
                } else {
                    return Err(panic::attribute_incorrect_format(
                        meta.path().get_ident().unwrap(),
                        &correct_usage_for_partial_eq_attribute,
                    ));
                }
            },
            Meta::List(list) => {
                let result =
                    list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

                let mut ignore_is_set = false;
                let mut method_is_set = false;
                let mut rank_is_set = false;

                let mut handler = |meta: Meta| -> syn::Result<bool> {
                    if let Some(ident) = meta.path().get_ident() {
                        match ident.to_string().as_str() {
                            "ignore" => {
                                if !self.enable_ignore {
                                    return Ok(false);
                                }

                                let v = meta_2_bool_allow_path(&meta)?;

                                if ignore_is_set {
                                    return Err(panic::parameter_reset(ident));
                                }

                                ignore_is_set = true;

                                ignore = v;

                                return Ok(true);
                            },
                            "method" => {
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
                            },
                            "rank" => {
                                if !self.enable_rank {
                                    return Ok(false);
                                }

                                let v = meta_2_isize(&meta)?;

                                if rank_is_set {
                                    return Err(panic::parameter_reset(ident));
                                }

                                rank_is_set = true;

                                rank = v;
                                rank_span = Some(meta.span());

                                return Ok(true);
                            },
                            _ => (),
                        }
                    }

                    Ok(false)
                };

                for p in result {
                    if !handler(p)? {
                        return Err(panic::attribute_incorrect_format(
                            meta.path().get_ident().unwrap(),
                            &correct_usage_for_partial_eq_attribute,
                        ));
                    }
                }
            },
        }

        Ok(FieldAttribute {
            ignore,
            method,
            method_returns_ordering: false,
            rank,
            rank_span,
        })
    }

    /// Scans the `#[educe(...)]` attributes of a field and parses its `PartialOrd` meta if present.
    pub(crate) fn build_from_attributes(
        &self,
        attributes: &[Attribute],
        traits: &[Trait],
    ) -> syn::Result<FieldAttribute> {
        let mut output = None;

        // When `Ord` is derived together and a field has no `PartialOrd` attribute of its own, the field's `Ord` attribute is used instead, so that `partial_cmp` stays consistent with `cmp` like in educe 0.5.x where the `Ord` handler generated both impls.
        #[cfg(feature = "Ord")]
        let mut fallback = None;

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

                    if t == Trait::PartialOrd {
                        if output.is_some() {
                            return Err(panic::reuse_a_trait(path.get_ident().unwrap()));
                        }

                        output = Some(self.build_from_partial_ord_meta(&meta)?);
                    }

                    // A malformed `Ord` attribute is silently skipped here; the `Ord` handler itself reports it with the proper usage message.
                    #[cfg(feature = "Ord")]
                    if t == Trait::Ord
                        && fallback.is_none()
                        && let Ok(mut field_attribute) = self.build_from_partial_ord_meta(&meta)
                    {
                        // An `Ord` comparison method returns `Ordering`, so the generated `partial_cmp` has to wrap its result in `Some`.
                        field_attribute.method_returns_ordering = field_attribute.method.is_some();

                        fallback = Some(field_attribute);
                    }
                }
            }
        }

        #[cfg(feature = "Ord")]
        let output = output.or(fallback);

        Ok(output.unwrap_or(FieldAttribute {
            ignore:                  false,
            method:                  None,
            method_returns_ordering: false,
            rank:                    self.rank,
            rank_span:               None,
        }))
    }
}
