#![allow(clippy::trivially_copy_pass_by_ref)]
#![cfg(feature = "Clone")]
#![no_std]

#[macro_use]
extern crate educe;

#[test]
fn basic() {
    #[derive(Educe)]
    #[educe(Clone)]
    enum Enum {
        Unit,
        Struct {
            f1: u8,
        },
        Tuple(u8),
    }

    let u = Enum::Unit.clone();
    let s = Enum::Struct {
        f1: 1,
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

#[test]
fn clone_without_trait_1() {
    fn clone(v: &u8) -> u8 {
        v + 100
    }

    #[derive(Educe)]
    #[educe(Clone)]
    enum Enum {
        Struct {
            #[educe(Clone(method = "clone"))]
            f1: u8,
        },
        Tuple(#[educe(Clone(method = "clone"))] u8),
    }

    let s = Enum::Struct {
        f1: 1,
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
fn clone_without_trait_2() {
    fn clone(v: &u8) -> u8 {
        v + 100
    }

    #[derive(Educe)]
    #[educe(Clone)]
    enum Enum {
        Struct {
            #[educe(Clone(method("clone")))]
            f1: u8,
        },
        Tuple(#[educe(Clone(method("clone")))] u8),
    }

    let s = Enum::Struct {
        f1: 1,
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
    enum Enum {
        Struct {
            #[educe(Clone(trait = "A"))]
            f1: u8,
        },
        Tuple(#[educe(Clone(trait = "A"))] u8),
    }

    let s = Enum::Struct {
        f1: 1,
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
    enum Enum {
        Struct {
            #[educe(Clone(trait("A")))]
            f1: u8,
        },
        Tuple(#[educe(Clone(trait("A")))] u8),
    }

    let s = Enum::Struct {
        f1: 1,
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
    enum Enum {
        Struct {
            #[educe(Clone(trait = "A", method = "cloner"))]
            f1: u8,
        },
        Tuple(#[educe(Clone(trait = "A", method = "cloner"))] u8),
    }

    let s = Enum::Struct {
        f1: 1,
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
    enum Enum {
        Struct {
            #[educe(Clone(trait("A"), method("cloner")))]
            f1: u8,
        },
        Tuple(#[educe(Clone(trait("A"), method("cloner")))] u8),
    }

    let s = Enum::Struct {
        f1: 1,
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
    #[educe(Clone(bound))]
    enum Enum<T> {
        Struct {
            f1: T,
        },
        Tuple(T),
    }

    let s = Enum::Struct {
        f1: 1,
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
    #[educe(Clone(bound = "T: core::clone::Clone"))]
    enum Enum<T> {
        Struct {
            f1: T,
        },
        Tuple(T),
    }

    let s = Enum::Struct {
        f1: 1,
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
    #[educe(Clone(bound("T: core::clone::Clone")))]
    enum Enum<T> {
        Struct {
            f1: T,
        },
        Tuple(T),
    }

    let s = Enum::Struct {
        f1: 1,
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
