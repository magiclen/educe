use proc_macro2::Span;
use syn::Meta;

#[inline]
pub(crate) fn multiple_default_fields(span: Span) -> syn::Error {
    syn::Error::new(span, "multiple default fields are set")
}

#[inline]
pub(crate) fn no_default_field(meta: &Meta) -> syn::Error {
    syn::Error::new_spanned(meta, "there is no field set as default")
}

#[inline]
pub(crate) fn multiple_default_variants(span: Span) -> syn::Error {
    syn::Error::new(span, "multiple default variants are set")
}

#[inline]
pub(crate) fn no_default_variant(meta: &Meta) -> syn::Error {
    syn::Error::new_spanned(meta, "there is no variant set as default")
}
