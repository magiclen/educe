#![cfg(feature = "Clone")]
#![no_std]

#[macro_use]
extern crate educe;

#[test]
#[allow(irrefutable_let_patterns)]
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
    let s = Struct { f1: 1 }.clone();
    let t = Tuple(1).clone();

    assert!(if let Unit = u { true } else { false });

    assert_eq!(1, s.f1);
    assert_eq!(1, t.0);
}

#[test]
fn clone_without_trait_1() {
    fn clone(v: &u8) -> u8 {
        v + 100
    }

    #[derive(Educe)]
    #[educe(Clone)]
    struct Struct {
        #[educe(Clone(method = "clone"))]
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Clone)]
    struct Tuple(#[educe(Clone(method = "clone"))] u8);

    let s = Struct { f1: 1 }.clone();
    let t = Tuple(1).clone();

    assert_eq!(101, s.f1);
    assert_eq!(101, t.0);
}

#[test]
fn clone_without_trait_2() {
    fn clone(v: &u8) -> u8 {
        v + 100
    }

    #[derive(Educe)]
    #[educe(Clone)]
    struct Struct {
        #[educe(Clone(method("clone")))]
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Clone)]
    struct Tuple(#[educe(Clone(method("clone")))] u8);

    let s = Struct { f1: 1 }.clone();
    let t = Tuple(1).clone();

    assert_eq!(101, s.f1);
    assert_eq!(101, t.0);
}

#[test]
fn clone_with_trait_1() {
    trait A {
        fn clone(&self) -> Self;
    }

    impl A for u8 {
        fn clone(&self) -> u8 {
            self + 100
        }
    }

    #[derive(Educe)]
    #[educe(Clone)]
    struct Struct {
        #[educe(Clone(trait = "A"))]
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Clone)]
    struct Tuple(#[educe(Clone(trait = "A"))] u8);

    let s = Struct { f1: 1 }.clone();
    let t = Tuple(1).clone();

    assert_eq!(101, s.f1);
    assert_eq!(101, t.0);
}

#[test]
fn clone_with_trait_2() {
    trait A {
        fn clone(&self) -> Self;
    }

    impl A for u8 {
        fn clone(&self) -> u8 {
            self + 100
        }
    }

    #[derive(Educe)]
    #[educe(Clone)]
    struct Struct {
        #[educe(Clone(trait("A")))]
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Clone)]
    struct Tuple(#[educe(Clone(trait("A")))] u8);

    let s = Struct { f1: 1 }.clone();
    let t = Tuple(1).clone();

    assert_eq!(101, s.f1);
    assert_eq!(101, t.0);
}

#[test]
fn clone_with_trait_3() {
    trait A {
        fn cloner(&self) -> Self;
    }

    impl A for u8 {
        fn cloner(&self) -> u8 {
            self + 100
        }
    }

    #[derive(Educe)]
    #[educe(Clone)]
    struct Struct {
        #[educe(Clone(trait = "A", method = "cloner"))]
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Clone)]
    struct Tuple(#[educe(Clone(trait = "A", method = "cloner"))] u8);

    let s = Struct { f1: 1 }.clone();
    let t = Tuple(1).clone();

    assert_eq!(101, s.f1);
    assert_eq!(101, t.0);
}

#[test]
fn clone_with_trait_4() {
    trait A {
        fn cloner(&self) -> Self;
    }

    impl A for u8 {
        fn cloner(&self) -> u8 {
            self + 100
        }
    }

    #[derive(Educe)]
    #[educe(Clone)]
    struct Struct {
        #[educe(Clone(trait("A"), method("cloner")))]
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Clone)]
    struct Tuple(#[educe(Clone(trait("A"), method("cloner")))] u8);

    let s = Struct { f1: 1 }.clone();
    let t = Tuple(1).clone();

    assert_eq!(101, s.f1);
    assert_eq!(101, t.0);
}

#[test]
fn bound_1() {
    #[derive(Educe)]
    #[educe(Clone(bound))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(Clone(bound))]
    struct Tuple<T>(T);

    let s = Struct { f1: 1 }.clone();
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

    let s = Struct { f1: 1 }.clone();
    let t = Tuple(1).clone();

    assert_eq!(1, s.f1);
    assert_eq!(1, t.0);
}

#[test]
fn bound_3() {
    #[derive(Educe)]
    #[educe(Clone(bound("T: core::clone::Clone")))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(Clone(bound("T: core::clone::Clone")))]
    struct Tuple<T>(T);

    let s = Struct { f1: 1 }.clone();
    let t = Tuple(1).clone();

    assert_eq!(1, s.f1);
    assert_eq!(1, t.0);
}
