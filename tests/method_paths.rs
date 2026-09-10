#![cfg(any(
    feature = "Clone",
    feature = "Debug",
    feature = "PartialEq",
    feature = "PartialOrd",
    feature = "Ord",
    feature = "Hash"
))]
#![no_std]

use educe::Educe;

#[cfg(feature = "Clone")]
#[test]
fn clone_qualified_path_syntaxes() {
    trait Method {
        fn copy<T: Copy>(value: &T) -> T {
            *value
        }
    }
    impl Method for () {}

    #[derive(Educe)]
    #[educe(Clone)]
    struct Value(
        #[educe(Clone(method = <() as Method>::copy::<u8>))] u8,
        #[educe(Clone(method(<() as Method>::copy::<u8>)))] u8,
        #[educe(Clone(method = "<() as Method>::copy::<u8>"))] u8,
        #[educe(Clone(method("<() as Method>::copy::<u8>")))] u8,
    );

    let mut value = Value(1, 2, 3, 4).clone();
    assert_eq!((1, 2, 3, 4), (value.0, value.1, value.2, value.3));
    value.clone_from(&Value(5, 6, 7, 8));
    assert_eq!((5, 6, 7, 8), (value.0, value.1, value.2, value.3));
}

#[cfg(feature = "Debug")]
#[test]
fn debug_qualified_self_path() {
    extern crate alloc;
    use core::fmt::{self, Debug, Formatter};

    trait Format<T> {
        fn format(value: &T, f: &mut Formatter<'_>) -> fmt::Result;
    }

    #[derive(Educe)]
    #[educe(Debug(bound(T: Debug)))]
    struct Value<T>(#[educe(Debug(method = <Self as Format<T>>::format))] T);

    impl<T: Debug> Format<T> for Value<T> {
        fn format(value: &T, f: &mut Formatter<'_>) -> fmt::Result {
            value.fmt(f)
        }
    }

    assert_eq!("Value(7)", alloc::format!("{:?}", Value(7)));
}

#[cfg(feature = "PartialEq")]
#[test]
fn partial_eq_qualified_path() {
    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Value(#[educe(PartialEq(method = <u8 as PartialEq>::eq))] u8);

    assert!(Value(1) == Value(1));
    assert!(Value(1) != Value(2));
}

#[cfg(feature = "PartialOrd")]
#[test]
fn partial_ord_qualified_path() {
    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
    struct Value(#[educe(PartialOrd(method = <u8 as PartialOrd>::partial_cmp))] u8);

    assert_eq!(Some(core::cmp::Ordering::Less), Value(1).partial_cmp(&Value(2)));
}

#[cfg(feature = "Ord")]
#[test]
fn ord_qualified_path() {
    #[derive(PartialEq, Eq, PartialOrd, Educe)]
    #[educe(Ord)]
    struct Value(#[educe(Ord(method = <u8 as Ord>::cmp))] u8);

    assert_eq!(core::cmp::Ordering::Less, Value(1).cmp(&Value(2)));
}

#[cfg(feature = "Hash")]
#[test]
fn hash_qualified_path() {
    extern crate std;
    use core::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    #[derive(Educe)]
    #[educe(Hash)]
    struct Value(#[educe(Hash(method = <u8 as Hash>::hash))] u8);

    let mut expected = DefaultHasher::new();
    let mut actual = DefaultHasher::new();
    7u8.hash(&mut expected);
    Value(7).hash(&mut actual);
    assert_eq!(expected.finish(), actual.finish());
}
