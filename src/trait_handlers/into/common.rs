use syn::{ExprPath, Field, Fields, Lifetime, Type};

use super::models::FieldAttribute;
use crate::common::tools::HashType;

/// Selects the only field, an explicitly marked field, or the only matching field, in that order.
pub(crate) fn select_field<'a, 'b>(
    fields: &'a Fields,
    attributes: &'b [FieldAttribute],
    target: &HashType,
    matcher: &TargetMatcher,
) -> syn::Result<(usize, &'a Field, Option<&'b ExprPath>)> {
    if fields.len() == 1 {
        let field = fields.iter().next().unwrap();
        let method = attributes
            .first()
            .and_then(|attribute| attribute.types.get(target))
            .and_then(Option::as_ref);

        return Ok((0, field, method));
    }

    let mut selected = None;

    for (index, field) in fields.iter().enumerate() {
        if let Some(attribute) = attributes.get(index)
            && let Some((key, method)) = attribute.types.get_key_value(target)
        {
            if selected.is_some() {
                return Err(super::panic::multiple_into_fields(key));
            }

            selected = Some((index, field, method.as_ref()));
        }
    }

    if let Some(selected) = selected {
        return Ok(selected);
    }

    for (index, field) in fields.iter().enumerate() {
        if matcher.matches(&field.ty) {
            if selected.is_some() {
                return Err(super::panic::no_into_field(target));
            }

            selected = Some((index, field, None));
        }
    }

    selected.ok_or_else(|| super::panic::no_into_field(target))
}

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
