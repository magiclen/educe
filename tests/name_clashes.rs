// These types test that we don't have name clashes between field names and our local variables and parameter names.

#![cfg(all(
    feature = "Debug",
    feature = "Clone",
    feature = "PartialEq",
    feature = "Eq",
    feature = "PartialOrd",
    feature = "Ord",
    feature = "Hash",
    feature = "Into"
))]
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

#[test]
fn debug_method_names() {
    use core::fmt::{self, Formatter};

    fn f(value: &u8, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{value}")
    }
    use f as builder;
    use f as arg;
    use f as _field;
    use f as _0;
    use f as educe__value;

    #[derive(Educe)]
    #[educe(Debug)]
    struct Struct(
        #[educe(Debug(method = f))] u8,
        #[educe(Debug(method = builder))] u8,
        #[educe(Debug(method = arg))] u8,
        #[educe(Debug(method = educe__value))] u8,
    );

    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        Named {
            #[educe(Debug(method = _field))]
            field: u8,
        },
        Tuple(#[educe(Debug(method = _0))] u8),
    }

    assert_eq!("Struct(1, 2, 3, 4)", format!("{:?}", Struct(1, 2, 3, 4)));
    assert_eq!(
        "Named { field: 5 }",
        format!("{:?}", Enum::Named {
            field: 5
        })
    );
    assert_eq!("Tuple(6)", format!("{:?}", Enum::Tuple(6)));
}

#[test]
fn clone_method_names() {
    fn source(value: &u8) -> u8 {
        *value
    }
    use source as _s_field;
    use source as _0;

    #[derive(Educe)]
    #[educe(Clone)]
    struct Struct(#[educe(Clone(method = source))] u8);

    #[derive(Educe)]
    #[educe(Clone)]
    enum Enum {
        Named {
            #[educe(Clone(method = _s_field))]
            field: u8,
        },
        Tuple(#[educe(Clone(method = _0))] u8),
    }

    let mut value = Struct(1).clone();
    value.clone_from(&Struct(2));
    assert_eq!(2, value.0);
    let mut value = Enum::Named {
        field: 3
    }
    .clone();
    value.clone_from(&Enum::Named {
        field: 4
    });
    assert!(matches!(value, Enum::Named {
        field: 4
    }));
    value.clone_from(&Enum::Tuple(5));
    value.clone_from(&Enum::Tuple(6));
    assert!(matches!(value, Enum::Tuple(6)));
}

#[test]
fn comparison_and_hash_method_names() {
    use core::{
        cmp::Ordering,
        hash::{Hash, Hasher},
    };
    use std::collections::hash_map::DefaultHasher;

    fn other(left: &u8, right: &u8) -> bool {
        left == right
    }
    fn state<H: Hasher>(value: &u8, hasher: &mut H) {
        value.hash(hasher);
    }
    fn compare(left: &u8, right: &u8) -> Ordering {
        left.cmp(right)
    }
    fn partial_compare(left: &u8, right: &u8) -> Option<Ordering> {
        left.partial_cmp(right)
    }
    use compare as _s_field;
    use partial_compare as _0;
    use state as v_field;

    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Struct(
        #[educe(PartialEq(method = other), Ord(method = _s_field), Hash(method = state))] u8,
    );

    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum Enum {
        Named {
            #[educe(PartialEq(method = other), Ord(method = _s_field), Hash(method = v_field))]
            field: u8,
        },
    }

    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
    enum Partial {
        Tuple(#[educe(PartialOrd(method = _0))] u8),
    }

    let mut actual = DefaultHasher::new();
    let mut expected = DefaultHasher::new();
    Struct(3).hash(&mut actual);
    3u8.hash(&mut expected);
    assert_eq!(expected.finish(), actual.finish());
    Enum::Named {
        field: 3
    }
    .hash(&mut actual);
    assert!(Struct(1) == Struct(1));
    assert_eq!(Ordering::Less, Struct(1).cmp(&Struct(2)));
    assert_eq!(
        Ordering::Less,
        Enum::Named {
            field: 1
        }
        .cmp(&Enum::Named {
            field: 2
        })
    );
    assert_eq!(Some(Ordering::Less), Partial::Tuple(1).partial_cmp(&Partial::Tuple(2)));
}

#[test]
fn into_method_names() {
    fn value(number: u8) -> u16 {
        u16::from(number)
    }
    use value as _0;

    #[derive(Educe)]
    #[educe(Into(u16))]
    struct Struct(#[educe(Into(u16, method = value))] u8);

    #[derive(Educe)]
    #[educe(Into(u16))]
    enum Enum {
        Named {
            #[educe(Into(u16, method = value))]
            value: u8,
        },
        Tuple(#[educe(Into(u16, method = _0))] u8),
    }

    assert_eq!(1, u16::from(Struct(1)));
    assert_eq!(
        2,
        u16::from(Enum::Named {
            value: 2
        })
    );
    assert_eq!(3, u16::from(Enum::Tuple(3)));
}
