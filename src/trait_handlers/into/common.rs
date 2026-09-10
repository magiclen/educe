use syn::{Lifetime, Type};

use crate::common::tools::HashType;

#[inline]
/// Normalizes a field type into the key used to match it against an `Into` target type.
///
/// Reference lifetimes are ignored, but mutability and the number of references are preserved.
pub(crate) fn to_hash_type(ty: &Type) -> HashType {
    let mut ty = ty.clone();
    let mut current = &mut ty;
    while let Type::Reference(reference) = current {
        reference.lifetime = Some(Lifetime::new("'static", reference.and_token.span));
        current = &mut reference.elem;
    }
    HashType::from(ty)
}

/// Keeps the existing `'static` default only for omitted reference lifetimes.
pub(crate) fn target_type(mut ty: Type) -> Type {
    let mut current = &mut ty;
    while let Type::Reference(reference) = current {
        if reference.lifetime.is_none() {
            reference.lifetime = Some(Lifetime::new("'static", reference.and_token.span));
        }
        current = &mut reference.elem;
    }
    ty
}

/// A returned mutable reference can also be coerced to a shared reference by Rust.
pub(crate) fn field_matches_target(field: &Type, target: &Type) -> bool {
    if to_hash_type(field) == to_hash_type(target) {
        return true;
    }
    if let (Type::Reference(field), Type::Reference(target)) = (field, target)
        && field.mutability.is_some()
        && target.mutability.is_none()
    {
        return to_hash_type(&field.elem) == to_hash_type(&target.elem);
    }
    false
}
