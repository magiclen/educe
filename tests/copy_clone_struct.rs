#![cfg(all(feature = "Copy", feature = "Clone"))]
#![no_std]
#![allow(clippy::clone_on_copy)]

use core::marker::PhantomData;

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

#[test]
fn bound_4() {
    #[derive(Educe)]
    #[educe(Copy, Clone)]
    struct Struct<T, U> {
        f1: Option<T>,
        f2: PhantomData<U>,
    }

    #[derive(Educe)]
    #[educe(Copy, Clone)]
    struct Tuple<T, U>(Option<T>, PhantomData<U>);

    let s = Struct {
        f1: Some(1), f2: PhantomData::<core::fmt::Formatter>
    }
    .clone();
    let t = Tuple(Some(1), PhantomData::<core::fmt::Formatter>).clone();

    assert_eq!(Some(1), s.f1);
    assert_eq!(Some(1), t.0);
}

#[test]
fn bound_5() {
    trait Suitable {}
    struct SuitableNotClone;
    impl Suitable for SuitableNotClone {}
    let phantom = PhantomData::<SuitableNotClone>;

    fn copy<T: Copy>(t: &T) -> T {
        *t
    }

    #[derive(Educe)]
    #[educe(Copy)]
    struct Struct<T, U> {
        f1: Option<T>,
        f2: PhantomData<U>,
    }

    impl<T: Clone, U: Suitable> Clone for Struct<T, U> {
        fn clone(&self) -> Self {
            Struct {
                f1: self.f1.clone(), f2: PhantomData
            }
        }
    }

    #[derive(Educe)]
    #[educe(Copy)]
    struct Tuple<T, U>(Option<T>, PhantomData<U>);

    impl<T: Clone, U: Suitable> Clone for Tuple<T, U> {
        fn clone(&self) -> Self {
            Tuple(self.0.clone(), PhantomData)
        }
    }

    let s = copy(&Struct {
        f1: Some(1), f2: phantom
    });

    let t = copy(&Tuple(Some(1), phantom));

    assert_eq!(Some(1), s.f1);
    assert_eq!(Some(1), t.0);
}
