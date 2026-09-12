use syn::{Ident, Variant};

#[inline]
pub(crate) fn unit_struct_need_name(name: &Ident) -> syn::Error {
    syn::Error::new_spanned(name, "a unit struct needs to have a name")
}

#[inline]
pub(crate) fn unit_variant_need_name(variant: &Variant) -> syn::Error {
    syn::Error::new_spanned(
        variant,
        "a unit variant which doesn't use an enum name needs to have a name",
    )
}

#[inline]
pub(crate) fn unit_enum_need_name(name: &Ident) -> syn::Error {
    syn::Error::new_spanned(name, "a unit enum needs to have a name")
}
