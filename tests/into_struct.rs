#![cfg(feature = "Into")]
#![no_std]

use educe::Educe;

#[allow(dead_code)]
#[test]
fn basic_1() {
    #[derive(Educe)]
    #[educe(Into(u8))]
    struct Struct {
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Into(u8))]
    struct Struct2 {
        f1: u8,
        #[educe(Into(u8))]
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(Into(u8))]
    struct Tuple(u8);

    #[derive(Educe)]
    #[educe(Into(u8))]
    struct Tuple2(u8, #[educe(Into(u8))] u8);

    let s1 = Struct {
        f1: 1
    };

    let s2 = Struct2 {
        f1: 1, f2: 2
    };

    let t1 = Tuple(1);
    let t2 = Tuple2(1, 2);

    assert_eq!(1u8, s1.into());
    assert_eq!(2u8, s2.into());

    assert_eq!(1u8, t1.into());
    assert_eq!(2u8, t2.into());
}

#[allow(dead_code)]
#[test]
fn basic_2() {
    #[derive(Copy, Clone, Educe)]
    #[educe(Into(u8), Into(u16))]
    struct Struct {
        f1: u8,
        f2: u16,
    }

    #[derive(Copy, Clone, Educe)]
    #[educe(Into(u8), Into(u16))]
    struct Struct2 {
        f1: u8,
        #[educe(Into(u8))]
        f2: u8,
        f3: u16,
        #[educe(Into(u16))]
        f4: u16,
    }

    #[derive(Copy, Clone, Educe)]
    #[educe(Into(u8), Into(u16))]
    struct Tuple(u8, u16);

    #[derive(Copy, Clone, Educe)]
    #[educe(Into(u8), Into(u16))]
    struct Tuple2(u8, #[educe(Into(u8))] u8, u16, #[educe(Into(u16))] u16);

    let s1 = Struct {
        f1: 1, f2: 2
    };

    let s2 = Struct2 {
        f1: 1, f2: 2, f3: 3, f4: 4
    };

    let t1 = Tuple(1, 2);
    let t2 = Tuple2(1, 2, 3, 4);

    assert_eq!(1u8, s1.into());
    assert_eq!(2u16, s1.into());
    assert_eq!(2u8, s2.into());
    assert_eq!(4u16, s2.into());

    assert_eq!(1u8, t1.into());
    assert_eq!(2u16, t1.into());
    assert_eq!(2u8, t2.into());
    assert_eq!(4u16, t2.into());
}

#[test]
fn method_1() {
    fn into(v: u16) -> u8 {
        v as u8
    }

    #[derive(Educe)]
    #[educe(Into(u8))]
    struct Struct {
        #[educe(Into(u8, method = into))]
        f1: u16,
    }

    let s1 = Struct {
        f1: 1
    };

    assert_eq!(1u8, s1.into());
}

#[test]
fn method_2() {
    fn into(v: u16) -> u8 {
        v as u8
    }

    #[derive(Educe)]
    #[educe(Into(u8))]
    struct Struct {
        #[educe(Into(u8, method(into)))]
        f1: u16,
    }

    let s1 = Struct {
        f1: 1
    };

    assert_eq!(1u8, s1.into());
}

#[test]
fn bound_1() {
    #[derive(Educe)]
    #[educe(Into(u8))]
    struct Struct<T> {
        #[educe(Into(u8))]
        f1: T,
    }

    let s1 = Struct {
        f1: 1
    };

    assert_eq!(1u8, s1.into());
}

#[test]
fn bound_2() {
    #[derive(Educe)]
    #[educe(Into(u8, bound = "T: Into<u8>"))]
    struct Struct<T> {
        #[educe(Into(u8))]
        f1: T,
    }

    let s1 = Struct {
        f1: 1
    };

    assert_eq!(1u8, s1.into());
}

#[test]
fn bound_3() {
    #[derive(Educe)]
    #[educe(Into(u8, bound(T: Into<u8>)))]
    struct Struct<T> {
        #[educe(Into(u8))]
        f1: T,
    }

    let s1 = Struct {
        f1: 1
    };

    assert_eq!(1u8, s1.into());
}

#[allow(dead_code)]
#[test]
fn from_impl() {
    // A concrete target type gets a `From` impl, so both directions of the conversion are available.
    #[derive(Educe)]
    #[educe(Into(u8))]
    struct Struct {
        f1: u8,
    }

    let s = Struct {
        f1: 1
    };

    assert_eq!(1u8, u8::from(s));
}

#[allow(dead_code)]
#[test]
fn generic_target() {
    extern crate alloc;

    use alloc::vec::Vec;

    // A generic target type whose type parameter is covered (here by `Vec`) gets a `From` impl, so both directions of the conversion are available.
    #[derive(Educe)]
    #[educe(Into(Vec<T>))]
    struct Struct<T> {
        f1: Vec<T>,
    }

    let s = Struct {
        f1: alloc::vec![1, 2]
    };

    assert_eq!(alloc::vec![1, 2], Vec::from(s));
}

#[allow(dead_code)]
#[test]
fn force_into() {
    // The `into` flag forces an `Into` impl for a concrete target type.
    #[derive(Educe)]
    #[educe(Into(u8, into))]
    struct Struct {
        f1: u8,
    }

    let s = Struct {
        f1: 1
    };

    assert_eq!(1u8, Into::<u8>::into(s));
}
