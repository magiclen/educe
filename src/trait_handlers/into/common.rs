use quote::quote_spanned;
use syn::{Type, spanned::Spanned};

use crate::common::{tools::HashType, r#type::dereference_changed};

#[inline]
/// Normalizes a field type into the key used to match it against an `Into` target type.
///
/// References are stripped and re-added with a `'static` lifetime, so that `&'a str` and `&'static str` compare as the same target.
pub(crate) fn to_hash_type(ty: &Type) -> HashType {
    let (ty, is_ref) = dereference_changed(ty);

    let ty = if is_ref {
        syn::parse2(quote_spanned!( ty.span() => &'static #ty )).unwrap()
    } else {
        ty.clone()
    };

    HashType::from(ty)
}
