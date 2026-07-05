#![cfg(all(feature = "Copy", feature = "Clone"))]
#![no_std]
#![allow(clippy::clone_on_copy)]
// The types in these tests only exist to exercise the derived impls, and `#[automatically_derived]` impls do not count as uses for dead-code analysis.
#![allow(dead_code)]

use educe::Educe;

#[test]
fn empty() {
    #[allow(dead_code)]
    #[derive(Educe)]
    #[educe(Copy, Clone)]
    struct Struct {}

    #[allow(dead_code)]
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

#[test]
fn generic_1() {
    #[derive(Educe)]
    #[educe(Copy, Clone)]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(Copy, Clone)]
    struct Tuple<T>(T);

    fn assert_copy<T: Copy>(_v: &T) {}

    // A generic type argument that satisfies `Copy` makes the whole type `Copy`.
    let s = Struct {
        f1: 1
    };
    let t = Tuple(1);

    assert_copy(&s);
    assert_copy(&t);

    assert_eq!(1, s.clone().f1);
    assert_eq!(1, t.clone().0);
}

#[test]
fn generic_2() {
    struct NotCopy;

    impl Clone for NotCopy {
        fn clone(&self) -> Self {
            NotCopy
        }
    }

    #[derive(Educe)]
    #[educe(Copy, Clone)]
    struct Struct<T> {
        f1: T,
    }

    // A type argument that is `Clone` but not `Copy` can still be cloned, like the built-in derives.
    let s = Struct {
        f1: NotCopy
    };

    let _ = s.clone();
}

#[test]
fn generic_3() {
    // The Copy impl can come from educe while the Clone impl comes from the built-in derive.
    #[derive(Clone, Educe)]
    #[educe(Copy)]
    struct Struct<T> {
        f1: Option<T>,
    }

    fn assert_copy<T: Copy>(_v: &T) {}

    let s = Struct {
        f1: Some(1)
    };

    assert_copy(&s);

    assert_eq!(Some(1), s.clone().f1);
}
