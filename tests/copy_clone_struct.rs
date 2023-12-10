#![cfg(all(feature = "Copy", feature = "Clone"))]
#![no_std]
#![allow(clippy::clone_on_copy)]

use educe::Educe;

#[test]
fn empty() {
    #[derive(Educe)]
    #[educe(Copy, Clone)]
    struct Struct {}

    #[derive(Educe)]
    #[educe(Copy, Clone)]
    struct Tuple();
}

#[test]
fn basic() {
    #[derive(Educe)]
    #[educe(Copy, Clone)]
    struct Unit;

    #[derive(Educe)]
    #[educe(Copy, Clone)]
    struct Struct {
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Copy, Clone)]
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
fn bound_1() {
    #[derive(Educe)]
    #[educe(Copy, Clone)]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(Copy, Clone)]
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
    #[educe(Copy, Clone(bound = "T: core::marker::Copy"))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(Copy, Clone(bound = "T: core::marker::Copy"))]
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
    #[educe(Copy, Clone(bound(T: core::marker::Copy)))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(Copy, Clone(bound(T: core::marker::Copy)))]
    struct Tuple<T>(T);

    let s = Struct {
        f1: 1
    }
    .clone();
    let t = Tuple(1).clone();

    assert_eq!(1, s.f1);
    assert_eq!(1, t.0);
}
