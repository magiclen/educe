#![cfg(feature = "Clone")]
#![no_std]
// The types in these tests only exist to exercise the derived impls, and `#[automatically_derived]` impls do not count as uses for dead-code analysis.
#![allow(dead_code)]

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

#[test]
fn bound_4() {
    #[derive(Educe)]
    #[educe(Clone(bound(*)))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(Clone(bound(*)))]
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
fn bound_5() {
    #[derive(Educe)]
    #[educe(Clone)]
    struct Struct<T> {
        f1: Option<T>,
    }

    #[derive(Educe)]
    #[educe(Clone)]
    struct Tuple<T>(Option<T>);

    let s = Struct {
        f1: Some(1)
    }
    .clone();
    let t = Tuple(Some(1)).clone();

    assert_eq!(Some(1), s.f1);
    assert_eq!(Some(1), t.0);
}

#[test]
fn bound_6() {
    extern crate alloc;

    struct NotClone;

    // These std types clone by duplicating a pointer or a marker, so their type arguments never receive a `Clone` bound.
    #[derive(Educe)]
    #[educe(Clone)]
    struct Struct<'a, T> {
        f1: alloc::sync::Arc<T>,
        f2: alloc::rc::Rc<T>,
        f3: &'a T,
        f4: core::marker::PhantomData<T>,
    }

    let not_clone = NotClone;
    let arc = alloc::sync::Arc::new(NotClone);
    let rc = alloc::rc::Rc::new(NotClone);

    let s = Struct {
        f1: arc, f2: rc, f3: &not_clone, f4: core::marker::PhantomData
    };

    let _ = s.clone();
}

#[test]
fn bound_7() {
    // A field type with a conditional `Clone` impl gets a precise `NotAlwaysClone<T>: Clone` predicate, so the derive works for exactly the type arguments that support it.
    struct NotAlwaysClone<T>(T);

    impl Clone for NotAlwaysClone<u8> {
        fn clone(&self) -> Self {
            NotAlwaysClone(self.0)
        }
    }

    #[derive(Educe)]
    #[educe(Clone)]
    struct Struct<T> {
        f1: NotAlwaysClone<T>,
    }

    let s = Struct {
        f1: NotAlwaysClone(1u8)
    };

    assert_eq!(1, s.clone().f1.0);
}
