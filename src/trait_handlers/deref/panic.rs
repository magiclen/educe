use proc_macro2::Span;
use syn::{Meta, Variant};

#[inline]
pub(crate) fn multiple_deref_fields(span: Span) -> syn::Error {
    syn::Error::new(span, "multiple fields are set for `Deref`")
}

#[inline]
pub(crate) fn multiple_deref_fields_of_variant(span: Span, variant: &Variant) -> syn::Error {
    syn::Error::new(
        span,
        format!("multiple fields of the `{}` variant are set for `Deref`", variant.ident),
    )
}

#[inline]
pub(crate) fn no_deref_field(meta: &Meta) -> syn::Error {
    syn::Error::new_spanned(meta, "there is no field which is assigned for `Deref`")
}

#[inline]
pub(crate) fn no_deref_field_of_variant(variant: &Variant) -> syn::Error {
    syn::Error::new_spanned(
        variant,
        format!(
            "there is no field for the `{}` variant which is assigned for `Deref`",
            variant.ident
        ),
    )
}
