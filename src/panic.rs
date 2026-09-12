use core::fmt::{self, Display, Formatter};

use proc_macro2::Span;
use syn::{Ident, Meta, Path, Variant};

use crate::{Trait, common::path::path_to_string};

// This module centralizes the error constructors, so the error messages stay consistent across the handlers.

struct DisplayStringSlice<'a>(&'a [&'static str]);

impl Display for DisplayStringSlice<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if !self.0.is_empty() {
            f.write_str(", which should be reformatted as follows:")?;

            for &s in self.0 {
                f.write_str("\n    ")?;
                f.write_str(s)?;
            }
        }

        Ok(())
    }
}

struct DisplayTraits;

impl Display for DisplayTraits {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for t in &Trait::VARIANTS[..Trait::VARIANTS.len() - 1] {
            f.write_str("\n    ")?;
            f.write_fmt(format_args!("{t:?}"))?;
        }

        Ok(())
    }
}

#[inline]
pub(crate) fn derive_attribute_not_set_up_yet() -> syn::Error {
    syn::Error::new(
        Span::call_site(),
        "you are using `Educe` in the `derive` attribute, but it has not been set up yet",
    )
}

#[inline]
pub(crate) fn attribute_incorrect_place(name: &Ident) -> syn::Error {
    syn::Error::new_spanned(name, format!("the `{name}` attribute cannot be placed here"))
}

#[inline]
pub(crate) fn attribute_incorrect_format(
    name: &Ident,
    correct_usage: &[&'static str],
) -> syn::Error {
    if correct_usage.is_empty() {
        attribute_incorrect_place(name)
    } else {
        syn::Error::new_spanned(
            name,
            format!(
                "you are using an incorrect format of the `{name}` attribute{}",
                DisplayStringSlice(correct_usage)
            ),
        )
    }
}

#[inline]
pub(crate) fn parameter_reset(name: &Ident) -> syn::Error {
    syn::Error::new_spanned(name, format!("you are trying to reset the `{name}` parameter"))
}

#[inline]
pub(crate) fn educe_format_incorrect(name: &Ident) -> syn::Error {
    attribute_incorrect_format(name, &[stringify!(#[educe(Trait1, Trait2, ..., TraitN)])])
}

#[inline]
pub(crate) fn unsupported_trait(name: &Path) -> syn::Error {
    let name_string = match name.get_ident() {
        Some(name) => name.to_string(),
        None => path_to_string(name),
    };

    syn::Error::new_spanned(
        name,
        format!("unsupported trait `{name_string}`, available traits:{DisplayTraits}"),
    )
}

#[inline]
pub(crate) fn reuse_a_trait(name: &Ident) -> syn::Error {
    syn::Error::new_spanned(name, format!("the trait `{name}` is used repeatedly"))
}

#[inline]
pub(crate) fn trait_not_used(name: &Ident) -> syn::Error {
    syn::Error::new_spanned(name, format!("the trait `{name}` is not used"))
}

/// The byte-reading union implementations are unsafe, so the user has to opt in with the `unsafe` keyword.
///
/// The derived trait is taken from the meta itself, so one message serves `Debug`, `PartialEq`, and `Hash`.
#[inline]
pub(crate) fn union_without_unsafe(meta: &Meta) -> syn::Error {
    let path = meta.path();
    let name = path_to_string(path);
    let suggestion = if let Meta::List(list) = meta
        && !list.tokens.is_empty()
    {
        let arguments = &list.tokens;
        quote::quote!(#[educe(#path(unsafe, #arguments))])
    } else {
        quote::quote!(#[educe(#path(unsafe))])
    };

    syn::Error::new_spanned(
        meta,
        format!(
            "a union's `{name}` implementation reads its entire storage as bytes; reading \
             uninitialized bytes is undefined behavior\n* Every byte must be initialized and \
             readable during each call, including padding and bytes outside the active field.\n* \
             The storage must not change during a call. Initialization must hold after \
             construction, writes, moves, and copies.\n* Only if you can uphold this safety \
             contract, use `{suggestion}`."
        ),
    )
}

#[inline]
pub(crate) fn reuse_a_rank(span: Span, rank: isize) -> syn::Error {
    syn::Error::new(span, format!("the rank `{rank}` is repeatedly used"))
}

#[inline]
pub(crate) fn trait_not_support_union(name: &Ident) -> syn::Error {
    syn::Error::new_spanned(name, format!("the trait `{name}` cannot be implemented for a union"))
}

#[inline]
pub(crate) fn trait_not_support_unit_variant(name: &Ident, variant: &Variant) -> syn::Error {
    syn::Error::new_spanned(
        variant,
        format!("the trait `{name}` cannot be implemented for an enum which has unit variants"),
    )
}
