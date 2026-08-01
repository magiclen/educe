// These types test that we don't have name clashes between field names and our local variables and parameter names.

#![cfg(feature = "default")]
#![allow(dead_code)]

use educe::Educe;

#[derive(Educe)]
#[educe(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NameClashesEnum {
    Variant { f: i8, builder: i16, source: i32, other: i64, state: i128 },
}

#[derive(Educe)]
#[educe(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub enum NameClashesEnumNoOrd {
    Variant { f: i8, builder: i16, source: i32, other: i64, state: i128 },
}

#[derive(Educe)]
#[educe(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NameClashesStruct {
    f:       i8,
    builder: i16,
    source:  i32,
    other:   i64,
    state:   i128,
}

#[derive(Educe)]
#[educe(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct NameClashesStructNoOrd {
    f:       i8,
    builder: i16,
    source:  i32,
    other:   i64,
    state:   i128,
}

// A module can shadow the prelude, so the generated code must not depend on `Option`, `Some`, and `None` resolving to the standard library items.
mod shadowed_prelude {
    use educe::Educe;

    pub enum MyOption {
        Some,
        None,
    }

    // The import is only here to shadow the prelude, so nothing in this module refers to it by name.
    #[allow(unused_imports)]
    pub use MyOption::{None, Some};

    pub type Option = u8;

    #[derive(Educe)]
    #[educe(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Struct {
        f1: i8,
        f2: i16,
    }

    #[derive(Educe)]
    #[educe(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Enum {
        Unit,
        Struct { f1: i8 },
        Tuple(i16),
    }
}

#[derive(Educe)]
#[educe(Debug(unsafe), PartialEq(unsafe), Eq)]
pub union NameClashesUnion {
    f:       i8,
    builder: i16,
    other:   i64,
}
