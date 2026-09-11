#![cfg(all(
    feature = "Debug",
    feature = "Clone",
    feature = "Copy",
    feature = "PartialEq",
    feature = "Eq",
    feature = "PartialOrd",
    feature = "Ord",
    feature = "Hash"
))]
// The types in these tests only exist to exercise the derived impls, and `#[automatically_derived]` impls do not count as uses for dead-code analysis.
#![allow(dead_code)]
// A bare `#[repr(packed)]` is exactly the form the derive has to recognize, and the ABI of these test types never leaves the test.
#![allow(clippy::repr_packed_without_abi)]

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use educe::Educe;

fn hash<T: Hash>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();

    value.hash(&mut hasher);

    hasher.finish()
}

#[test]
fn named() {
    #[derive(Educe)]
    #[educe(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(packed)]
    struct Struct {
        f1: i32,
        f2: u64,
    }

    let a = Struct {
        f1: 1, f2: 2
    };
    let b = a.clone();

    assert_eq!("Struct { f1: 1, f2: 2 }", format!("{a:?}"));
    assert_eq!(a, b);
    assert_eq!(hash(&a), hash(&b));

    let c = Struct {
        f1: 1, f2: 3
    };

    assert!(a < c);

    let mut d = Struct {
        f1: 9, f2: 9
    };

    d.clone_from(&a);

    assert_eq!(a, d);
}

#[test]
fn tuple() {
    #[derive(Educe)]
    #[educe(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(packed)]
    struct Tuple(i32, u64);

    let a = Tuple(1, 2);
    let b = a.clone();

    assert_eq!("Tuple(1, 2)", format!("{a:?}"));
    assert_eq!(a, b);
    assert_eq!(hash(&a), hash(&b));
    assert!(a < Tuple(1, 3));

    let mut c = Tuple(9, 9);

    c.clone_from(&a);

    assert_eq!(a, c);
}

#[test]
fn generic() {
    #[derive(Educe)]
    #[educe(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(packed)]
    struct Struct<T>(i32, T);

    let a = Struct(1, 2u64);
    let b = a.clone();

    assert_eq!("Struct(1, 2)", format!("{a:?}"));
    assert_eq!(a, b);
    assert_eq!(hash(&a), hash(&b));
    assert!(a < Struct(1, 3u64));
}

#[test]
fn no_bound_when_no_field_needs_one() {
    // This is the shape from the issue: neither `i32` nor `PhantomData<T>` needs a predicate, so the generated impl works for any `T`, even a non-`Copy` one.
    #[derive(Educe)]
    #[educe(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(packed)]
    struct Struct<T>(i32, PhantomData<T>);

    let a: Struct<String> = Struct(1, PhantomData);
    let b = a.clone();

    assert_eq!(a, b);
    assert_eq!(hash(&a), hash(&b));
}

#[test]
fn packing_levels() {
    #[derive(Educe)]
    #[educe(Debug, Clone, PartialEq, Hash)]
    #[repr(C, packed)]
    struct Combined(i32, u64);

    #[derive(Educe)]
    #[educe(Debug, Clone, PartialEq, Hash)]
    #[repr(packed(2))]
    struct Limited(i32, u64);

    let a = Combined(1, 2);
    let b = Limited(1, 2);

    assert_eq!(a, a.clone());
    assert_eq!(b, b.clone());
    assert_eq!("Combined(1, 2)", format!("{a:?}"));
    assert_eq!("Limited(1, 2)", format!("{b:?}"));
}

#[test]
fn copy_uses_the_bitwise_shortcut() {
    #[derive(Educe)]
    #[educe(Copy, Clone, Debug, PartialEq)]
    #[repr(packed)]
    struct Struct(i32, u64);

    let a = Struct(1, 2);
    let b = a;

    assert_eq!(a, b);
    assert_eq!(a, a.clone());
}

#[test]
fn methods() {
    fn fmt(value: &i32, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(if *value > 0 { "positive" } else { "other" })
    }

    fn clone(value: &i32) -> i32 {
        *value + 1
    }

    fn eq(_: &i32, _: &i32) -> bool {
        true
    }

    fn hash_always_zero<H: Hasher>(_: &i32, state: &mut H) {
        Hash::hash(&0u8, state);
    }

    #[derive(Educe)]
    #[educe(Debug, Clone, PartialEq, Hash)]
    #[repr(packed)]
    struct Struct(
        #[educe(
            Debug(method = fmt),
            Clone(method = clone),
            PartialEq(method = eq),
            Hash(method = hash_always_zero)
        )]
        i32,
        u64,
    );

    let a = Struct(1, 2);

    assert_eq!("Struct(positive, 2)", format!("{a:?}"));
    // Reading a packed field in the test needs the same copy-out that the derive does.
    assert_eq!(2, { a.clone().0 });
    assert_eq!(a, Struct(-5, 2));
    assert_eq!(hash(&a), hash(&Struct(-5, 2)));
}
