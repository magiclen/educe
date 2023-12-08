#[cfg(any(feature = "PartialOrd", feature = "Ord"))]
mod discriminant_type;

#[cfg(any(feature = "PartialOrd", feature = "Ord"))]
pub(crate) use discriminant_type::*;
