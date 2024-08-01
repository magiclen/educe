use std::ops::Deref;

use syn::Field;

use crate::common::ident_index::IdentOrIndex;

/// A field, in a `Fields::Named` or a `Fields::Unnamed`
///
/// Allows unification of the data handling code,
/// to always use `{ }` syntax even for tuple data.
pub(crate) struct FieldInfo<'f> {
    pub(crate) name:  IdentOrIndex,
    pub(crate) field: &'f Field,
}

impl<'f> FieldInfo<'f> {
    pub(crate) fn new(index: usize, field: &'f Field) -> Self {
        let name = IdentOrIndex::from_ident_with_index(field.ident.as_ref(), index);
        FieldInfo {
            name,
            field,
        }
    }
}

impl<'f> Deref for FieldInfo<'f> {
    type Target = &'f Field;

    fn deref(&self) -> &&'f Field {
        &self.field
    }
}
