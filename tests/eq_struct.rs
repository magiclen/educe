#![cfg(all(feature = "PartialEq", feature = "Eq"))]
#![no_std]

#[macro_use]
extern crate educe;

#[test]
fn basic() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Unit;

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Struct {
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Tuple(u8);

    assert!(Unit == Unit);

    assert!(Struct { f1: 1 } == Struct { f1: 1 });

    assert!(Struct { f1: 1 } != Struct { f1: 2 });

    assert!(Tuple(1) == Tuple(1));
    assert!(Tuple(1) != Tuple(2));
}

#[test]
fn bound_1() {
    #[derive(Educe)]
    #[educe(PartialEq(bound), Eq(bound))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(PartialEq(bound), Eq(bound))]
    struct Tuple<T>(T);

    assert!(Struct { f1: 1 } == Struct { f1: 1 });

    assert!(Struct { f1: 1 } != Struct { f1: 2 });

    assert!(Tuple(1) == Tuple(1));
    assert!(Tuple(1) != Tuple(2));
}

#[test]
fn bound_2() {
    #[derive(Educe)]
    #[educe(PartialEq(bound), Eq(bound = "T: core::cmp::Eq"))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(PartialEq(bound), Eq(bound = "T: core::cmp::Eq"))]
    struct Tuple<T>(T);

    assert!(Struct { f1: 1 } == Struct { f1: 1 });

    assert!(Struct { f1: 1 } != Struct { f1: 2 });

    assert!(Tuple(1) == Tuple(1));
    assert!(Tuple(1) != Tuple(2));
}

#[test]
fn bound_3() {
    #[derive(Educe)]
    #[educe(PartialEq(bound), Eq(bound("T: core::cmp::Eq")))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(PartialEq(bound), Eq(bound("T: core::cmp::Eq")))]
    struct Tuple<T>(T);

    assert!(Struct { f1: 1 } == Struct { f1: 1 });

    assert!(Struct { f1: 1 } != Struct { f1: 2 });

    assert!(Tuple(1) == Tuple(1));
    assert!(Tuple(1) != Tuple(2));
}
