#![cfg(any(feature = "Debug", feature = "PartialEq", feature = "Hash"))]
#![allow(non_camel_case_types, dead_code)]

use educe::Educe;

type u8 = u32;
type bool = u16;
type str = u32;

// The only field fills the union storage and has no padding.
#[derive(Educe)]
#[cfg_attr(feature = "Debug", educe(Debug(unsafe)))]
#[cfg_attr(feature = "PartialEq", educe(PartialEq(unsafe)))]
#[cfg_attr(feature = "Hash", educe(Hash(unsafe)))]
union Union {
    value: u8,
}

#[cfg(feature = "Debug")]
#[test]
fn debug() {
    #[derive(Educe)]
    #[educe(Debug(unsafe, name = false))]
    union Unnamed {
        value: u8,
    }

    #[derive(Educe)]
    #[educe(Debug(name = false))]
    struct Map {
        value: u8,
    }

    let value = 0x12345678u32;
    let bytes = value.to_ne_bytes();

    assert_eq!(
        format!("Union({bytes:?})"),
        format!("{:?}", Union {
            value
        })
    );
    assert_eq!(
        format!("{bytes:?}"),
        format!("{:?}", Unnamed {
            value
        })
    );
    assert_eq!(
        "{value: 7}",
        format!("{:?}", Map {
            value: 7
        })
    );
}

#[cfg(feature = "PartialEq")]
#[test]
fn equality() {
    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Struct(u8);

    #[derive(Educe)]
    #[educe(PartialEq)]
    enum Enum {
        Value(u8),
    }

    assert!(
        Union {
            value: 7
        } == Union {
            value: 7
        }
    );
    assert!(
        Union {
            value: 7
        } != Union {
            value: 8
        }
    );
    assert!(Struct(7) == Struct(7));
    assert!(Enum::Value(7) == Enum::Value(7));
}

#[cfg(feature = "Hash")]
#[test]
fn hash() {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    let value = 0x12345678u32;
    let mut expected = DefaultHasher::new();
    let mut actual = DefaultHasher::new();

    value.to_ne_bytes().as_slice().hash(&mut expected);
    Union {
        value,
    }
    .hash(&mut actual);

    assert_eq!(expected.finish(), actual.finish());
}
