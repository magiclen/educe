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
#[educe(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NameClashesStruct {
    f:       i8,
    builder: i16,
    source:  i32,
    other:   i64,
    state:   i128,
}

#[derive(Educe)]
#[educe(Debug(unsafe), PartialEq(unsafe), Eq)]
pub union NameClashesUnion {
    f:       i8,
    builder: i16,
    other:   i64,
}
