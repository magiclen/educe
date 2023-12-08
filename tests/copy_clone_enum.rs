#![cfg(all(feature = "Copy", feature = "Clone"))]
#![no_std]
#![allow(clippy::clone_on_copy)]

use educe::Educe;

#[allow(dead_code)]
#[test]
fn empty() {
    #[derive(Educe)]
    #[educe(Copy, Clone)]
    enum Enum {}

    #[derive(Educe)]
    #[educe(Copy, Clone)]
    enum Enum2 {
        Struct {},
        Tuple(),
    }

    let s = Enum2::Struct {}.clone();
    let t = Enum2::Tuple().clone();

    assert!(matches!(s, Enum2::Struct {}));
    assert!(matches!(t, Enum2::Tuple()));
}

#[test]
fn basic() {
    #[derive(Educe)]
    #[educe(Copy, Clone)]
    enum Enum {
        Unit,
        Struct { f1: u8 },
        Tuple(u8),
    }

    let u = Enum::Unit.clone();
    let s = Enum::Struct {
        f1: 1
    }
    .clone();
    let t = Enum::Tuple(1).clone();

    assert!(matches!(u, Enum::Unit));

    if let Enum::Struct {
        f1,
    } = s
    {
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

#[allow(clippy::non_canonical_clone_impl)]
#[test]
fn method() {
    fn clone(v: &u8) -> u8 {
        v + 100
    }

    #[derive(Educe)]
    #[educe(Copy, Clone)]
    enum Enum {
        Struct {
            #[educe(Clone(method = "clone"))]
            f1: u8,
        },
        Tuple(#[educe(Clone(method = "clone"))] u8),
    }

    let s = Enum::Struct {
        f1: 1
    }
    .clone();
    let t = Enum::Tuple(1).clone();

    if let Enum::Struct {
        f1,
    } = s
    {
        assert_eq!(101, f1);
    } else {
        panic!();
    }

    if let Enum::Tuple(f1) = t {
        assert_eq!(101, f1);
    } else {
        panic!();
    }
}

#[test]
fn bound_1() {
    #[derive(Educe)]
    #[educe(Copy, Clone)]
    enum Enum<T> {
        Struct { f1: T },
        Tuple(T),
    }

    let s = Enum::Struct {
        f1: 1
    }
    .clone();
    let t = Enum::Tuple(1).clone();

    if let Enum::Struct {
        f1,
    } = s
    {
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
        Struct { f1: T },
        Tuple(T),
    }

    let s = Enum::Struct {
        f1: 1
    }
    .clone();
    let t = Enum::Tuple(1).clone();

    if let Enum::Struct {
        f1,
    } = s
    {
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
    #[educe(Copy(bound("T: core::marker::Copy")), Clone(bound("T: core::marker::Copy")))]
    enum Enum<T> {
        Struct { f1: T },
        Tuple(T),
    }

    let s = Enum::Struct {
        f1: 1
    }
    .clone();
    let t = Enum::Tuple(1).clone();

    if let Enum::Struct {
        f1,
    } = s
    {
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
fn bound_4() {
    #[derive(Educe)]
    #[educe(Copy(bound(T: core::marker::Copy)), Clone(bound(T: core::marker::Copy)))]
    enum Enum<T> {
        Struct { f1: T },
        Tuple(T),
    }

    let s = Enum::Struct {
        f1: 1
    }
    .clone();
    let t = Enum::Tuple(1).clone();

    if let Enum::Struct {
        f1,
    } = s
    {
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
