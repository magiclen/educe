#![cfg(feature = "Clone")]
#![no_std]

use core::{fmt::Debug, marker::PhantomData};

use educe::Educe;

#[test]
fn empty() {
    #[derive(Educe)]
    #[educe(Clone)]
    struct Struct {}

    #[derive(Educe)]
    #[educe(Clone)]
    struct Tuple();

    let s = Struct {}.clone();
    let t = Tuple().clone();

    assert!(matches!(s, Struct {}));

    assert!(matches!(t, Tuple()));
}

#[test]
fn basic() {
    #[derive(Educe)]
    #[educe(Clone)]
    struct Unit;

    #[derive(Educe)]
    #[educe(Clone)]
    struct Struct {
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Clone)]
    struct Tuple(u8);

    let u = Unit.clone();
    let s = Struct {
        f1: 1
    }
    .clone();
    let t = Tuple(1).clone();

    assert!(matches!(u, Unit));

    assert_eq!(1, s.f1);
    assert_eq!(1, t.0);
}

#[test]
fn method_1() {
    fn clone(v: &u8) -> u8 {
        v + 100
    }

    #[derive(Educe)]
    #[educe(Clone)]
    struct Struct {
        #[educe(Clone(method = clone))]
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Clone)]
    struct Tuple(#[educe(Clone(method = clone))] u8);

    let s = Struct {
        f1: 1
    }
    .clone();
    let t = Tuple(1).clone();

    assert_eq!(101, s.f1);
    assert_eq!(101, t.0);
}

#[test]
fn method_2() {
    fn clone(v: &u8) -> u8 {
        v + 100
    }

    #[derive(Educe)]
    #[educe(Clone)]
    struct Struct {
        #[educe(Clone(method(clone)))]
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Clone)]
    struct Tuple(#[educe(Clone(method(clone)))] u8);

    let s = Struct {
        f1: 1
    }
    .clone();
    let t = Tuple(1).clone();

    assert_eq!(101, s.f1);
    assert_eq!(101, t.0);
}

#[test]
fn bound_1() {
    #[derive(Educe)]
    #[educe(Clone)]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(Clone)]
    struct Tuple<T>(T);

    let s = Struct {
        f1: 1
    }
    .clone();
    let t = Tuple(1).clone();

    assert_eq!(1, s.f1);
    assert_eq!(1, t.0);
}

#[test]
fn bound_2() {
    #[derive(Educe)]
    #[educe(Clone(bound = "T: core::clone::Clone"))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(Clone(bound = "T: core::clone::Clone"))]
    struct Tuple<T>(T);

    let s = Struct {
        f1: 1
    }
    .clone();
    let t = Tuple(1).clone();

    assert_eq!(1, s.f1);
    assert_eq!(1, t.0);
}

#[test]
fn bound_3() {
    #[derive(Educe)]
    #[educe(Clone(bound(T: core::clone::Clone)))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(Clone(bound(T: core::clone::Clone)))]
    struct Tuple<T>(T);

    let s = Struct {
        f1: 1
    }
    .clone();
    let t = Tuple(1).clone();

    assert_eq!(1, s.f1);
    assert_eq!(1, t.0);
}

#[cfg(feature = "Debug")]
#[test]
fn leaking_bounds() {
    #[derive(Educe)]
    #[educe(Debug(bound(T: Debug)), Clone(bound(T: Clone)))]
    struct Struct<T> {
        x: PhantomData<T>,
    }

    #[derive(Clone)]
    struct NotDebug;

    let a = Struct {
        x: PhantomData::<NotDebug>
    };
    let _b = a.clone();
}
