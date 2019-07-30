#![no_std]

#[macro_use]
extern crate educe;

#[test]
fn basic() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    enum Enum {
        Unit,
        Unit2,
        Struct {
            f1: u8
        },
        Tuple(u8),
    }

    assert!(Enum::Unit == Enum::Unit);
    assert!(Enum::Unit != Enum::Unit2);

    assert!(Enum::Struct {
        f1: 1
    } == Enum::Struct {
        f1: 1
    });

    assert!(Enum::Struct {
        f1: 1
    } != Enum::Struct {
        f1: 2
    });

    assert!(Enum::Tuple(1) == Enum::Tuple(1));
    assert!(Enum::Tuple(1) != Enum::Tuple(2));
}

#[test]
fn bound_1() {
    #[derive(Educe)]
    #[educe(PartialEq(bound), Eq(bound))]
    enum Enum<T> {
        Unit,
        Struct {
            f1: T
        },
        Tuple(T),
    }

    assert!(Enum::<u8>::Unit == Enum::<u8>::Unit);

    assert!(Enum::Struct {
        f1: 1
    } == Enum::Struct {
        f1: 1
    });

    assert!(Enum::Struct {
        f1: 1
    } != Enum::Struct {
        f1: 2
    });

    assert!(Enum::Tuple(1) == Enum::Tuple(1));
    assert!(Enum::Tuple(1) != Enum::Tuple(2));
}

#[test]
fn bound_2() {
    #[derive(Educe)]
    #[educe(PartialEq(bound), Eq(bound = "T: core::cmp::Eq"))]
    enum Enum<T> {
        Unit,
        Struct {
            f1: T
        },
        Tuple(T),
    }

    assert!(Enum::<u8>::Unit == Enum::<u8>::Unit);

    assert!(Enum::Struct {
        f1: 1
    } == Enum::Struct {
        f1: 1
    });

    assert!(Enum::Struct {
        f1: 1
    } != Enum::Struct {
        f1: 2
    });

    assert!(Enum::Tuple(1) == Enum::Tuple(1));
    assert!(Enum::Tuple(1) != Enum::Tuple(2));
}

#[test]
fn bound_3() {
    #[derive(Educe)]
    #[educe(PartialEq(bound), Eq(bound("T: core::cmp::Eq")))]
    enum Enum<T> {
        Unit,
        Struct {
            f1: T
        },
        Tuple(T),
    }

    assert!(Enum::<u8>::Unit == Enum::<u8>::Unit);

    assert!(Enum::Struct {
        f1: 1
    } == Enum::Struct {
        f1: 1
    });

    assert!(Enum::Struct {
        f1: 1
    } != Enum::Struct {
        f1: 2
    });

    assert!(Enum::Tuple(1) == Enum::Tuple(1));
    assert!(Enum::Tuple(1) != Enum::Tuple(2));
}