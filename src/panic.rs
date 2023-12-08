use core::fmt::{self, Display, Formatter};

use proc_macro2::Span;
use syn::{spanned::Spanned, Ident, Path, Variant};

use crate::{common::path::path_to_string, Trait};

struct DisplayStringSlice<'a>(&'a [&'static str]);

impl<'a> Display for DisplayStringSlice<'a> {
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
            f.write_fmt(format_args!("{:?}", t))?;
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
    syn::Error::new(name.span(), format!("the `{}` attribute cannot be placed here", name,))
}

#[inline]
pub(crate) fn attribute_incorrect_format(
    name: &Ident,
    correct_usage: &[&'static str],
) -> syn::Error {
    if correct_usage.is_empty() {
        attribute_incorrect_place(name)
    } else {
        syn::Error::new(
            name.span(),
            format!(
                "you are using an incorrect format of the `{}` attribute{}",
                name,
                DisplayStringSlice(correct_usage)
            ),
        )
    }
}

#[inline]
pub(crate) fn parameter_reset(name: &Ident) -> syn::Error {
    syn::Error::new(name.span(), format!("you are trying to reset the `{}` parameter", name))
}

#[inline]
pub(crate) fn educe_format_incorrect(name: &Ident) -> syn::Error {
    attribute_incorrect_format(name, &[stringify!(#[educe(Trait1, Trait2, ..., TraitN)])])
}

#[inline]
pub(crate) fn unsupported_trait(name: &Path) -> syn::Error {
    match name.get_ident() {
        Some(name) => syn::Error::new(
            name.span(),
            format!("unsupported trait `{}`, available traits:{}", name, DisplayTraits),
        ),
        None => {
            let name = path_to_string(name);

            syn::Error::new(
                name.span(),
                format!("unsupported trait `{}`, available traits:{}", name, DisplayTraits),
            )
        },
    }
}

#[inline]
pub(crate) fn reuse_a_trait(name: &Ident) -> syn::Error {
    syn::Error::new(name.span(), format!("the trait `{}` is used repeatedly", name))
}

#[inline]
pub(crate) fn trait_not_used(name: &Ident) -> syn::Error {
    syn::Error::new(name.span(), format!("the trait `{}` is not used", name))
}

#[inline]
pub(crate) fn trait_not_support_union(name: &Ident) -> syn::Error {
    syn::Error::new(name.span(), format!("the trait `{}` does not support to a union", name))
}

#[inline]
pub(crate) fn trait_not_support_unit_variant(name: &Ident, variant: &Variant) -> syn::Error {
    syn::Error::new(
        variant.span(),
        format!("the trait `{}` cannot be implemented for an enum which has unit variants", name),
    )
}
