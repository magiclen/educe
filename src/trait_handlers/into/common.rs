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

/// One `Into` target prepared for matching, so that a field loop does not rebuild the target key for every field.
pub(crate) struct TargetMatcher {
    whole:   HashType,
    /// The pointee of a shared reference target, which a mutable field reference can also be coerced to.
    pointee: Option<HashType>,
}

impl TargetMatcher {
    pub(crate) fn new(target: &Type) -> Self {
        let pointee = match target {
            Type::Reference(target) if target.mutability.is_none() => {
                Some(to_hash_type(&target.elem))
            },
            _ => None,
        };

        Self {
            whole: to_hash_type(target),
            pointee,
        }
    }

    /// A returned mutable reference can also be coerced to a shared reference by Rust.
    pub(crate) fn matches(&self, field: &Type) -> bool {
        if to_hash_type(field) == self.whole {
            return true;
        }

        if let Type::Reference(field) = field
            && field.mutability.is_some()
            && let Some(pointee) = &self.pointee
        {
            return to_hash_type(&field.elem) == *pointee;
        }

        false
    }
}
