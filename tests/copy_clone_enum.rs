#![cfg(all(feature = "Copy", feature = "Clone"))]

#![no_std]

#[macro_use]
extern crate educe;

#[test]
fn basic() {
    #[derive(Educe)]
    #[educe(Copy, Clone)]
    enum Enum {
        Unit,
        Struct {
            f1: u8
        },
        Tuple(u8),
    }

    let u = Enum::Unit.clone();
    let s = Enum::Struct {
        f1: 1
    }.clone();
    let t = Enum::Tuple(1).clone();

    assert!(if let Enum::Unit = u {
        true
    } else {
        false
    });

    if let Enum::Struct { f1 } = s {
        assert_eq!(1, f1);
    } else {
        panic!();
    }

    if let Enum::Tuple(f1) = t {
        assert_eq!(1, f1);
    } else {
        panic!();
    }
}

#[test]
fn bound_1() {
    #[derive(Educe)]
    #[educe(Copy(bound), Clone(bound))]
    enum Enum<T> {
        Struct {
            f1: T
        },
        Tuple(T),
    }

    let s = Enum::Struct {
        f1: 1
    }.clone();
    let t = Enum::Tuple(1).clone();

    if let Enum::Struct { f1 } = s {
        assert_eq!(1, f1);
    } else {
        panic!();
    }

    if let Enum::Tuple(f1) = t {
        assert_eq!(1, f1);
    } else {
        panic!();
    }
}

#[test]
fn bound_2() {
    #[derive(Educe)]
    #[educe(Copy(bound = "T: core::marker::Copy"), Clone(bound = "T: core::marker::Copy"))]
    enum Enum<T> {
        Struct {
            f1: T
        },
        Tuple(T),
    }

    let s = Enum::Struct {
        f1: 1
    }.clone();
    let t = Enum::Tuple(1).clone();

    if let Enum::Struct { f1 } = s {
        assert_eq!(1, f1);
    } else {
        panic!();
    }

    if let Enum::Tuple(f1) = t {
        assert_eq!(1, f1);
    } else {
        panic!();
    }
}

#[test]
fn bound_3() {
    #[derive(Educe)]
    #[educe(Copy(bound = "T: core::marker::Copy"), Clone(bound = "T: core::marker::Copy"))]
    enum Enum<T> {
        Struct {
            f1: T
        },
        Tuple(T),
    }

    let s = Enum::Struct {
        f1: 1
    }.clone();
    let t = Enum::Tuple(1).clone();

    if let Enum::Struct { f1 } = s {
        assert_eq!(1, f1);
    } else {
        panic!();
    }

    if let Enum::Tuple(f1) = t {
        assert_eq!(1, f1);
    } else {
        panic!();
    }
}