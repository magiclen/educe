#[cfg(any(feature = "PartialOrd", feature = "Ord"))]
// Small standalone tools that need to be shared by several trait handlers.
mod discriminant_type;

#[cfg(any(feature = "PartialOrd", feature = "Ord"))]
pub(crate) use discriminant_type::*;

#[cfg(feature = "Into")]
mod hash_type;

#[cfg(feature = "Into")]
pub(crate) use hash_type::*;
