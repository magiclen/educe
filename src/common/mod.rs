#[allow(dead_code)]
// Shared building blocks for the trait handlers; most submodules are only compiled when a feature that needs them is enabled.
pub(crate) mod attributes;
#[allow(dead_code)]
pub(crate) mod bound;
#[allow(dead_code)]
pub(crate) mod path;
#[allow(dead_code)]
pub(crate) mod r#type;
#[allow(dead_code)]
pub(crate) mod where_predicates_bool;

#[cfg(feature = "Default")]
#[allow(dead_code)]
pub(crate) mod expr;
#[cfg(any(
    feature = "Debug",
    feature = "PartialEq",
    feature = "PartialOrd",
    feature = "Ord",
    feature = "Hash",
    feature = "Default"
))]
#[allow(dead_code)]
pub(crate) mod ident_bool;
#[cfg(any(
    feature = "Debug",
    feature = "PartialEq",
    feature = "PartialOrd",
    feature = "Ord",
    feature = "Hash",
    feature = "Deref",
    feature = "DerefMut",
    feature = "Into"
))]
#[allow(dead_code)]
pub(crate) mod ident_index;
#[cfg(any(feature = "PartialOrd", feature = "Ord"))]
#[allow(dead_code)]
pub(crate) mod int;
#[cfg(any(feature = "Debug", feature = "PartialEq", feature = "Hash"))]
#[allow(dead_code)]
pub(crate) mod unsafe_punctuated_meta;

#[cfg(any(feature = "PartialOrd", feature = "Ord", feature = "Into"))]
pub(crate) mod tools;
