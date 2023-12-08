#![cfg(feature = "Into")]
#![no_std]

use educe::Educe;

#[allow(dead_code)]
#[test]
fn basic_1() {
    #[derive(Educe)]
    #[educe(Into(u8))]
    enum Enum {
        Struct { f1: u8 },
        Tuple(u8),
    }

    #[derive(Educe)]
    #[educe(Into(u8))]
    enum Enum2 {
        Struct {
            f1: u8,
            #[educe(Into(u8))]
            f2: u8,
        },
        Tuple(u8, #[educe(Into(u8))] u8),
    }

    let s1 = Enum::Struct {
        f1: 1
    };

    let s2 = Enum2::Struct {
        f1: 1, f2: 2
    };

    let t1 = Enum::Tuple(1);
    let t2 = Enum2::Tuple(1, 2);

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
    enum Enum {
        Struct { f1: u8, f2: u16 },
        Tuple(u8, u16),
    }

    #[derive(Copy, Clone, Educe)]
    #[educe(Into(u8), Into(u16))]
    enum Enum2 {
        Struct {
            f1: u8,
            #[educe(Into(u8))]
            f2: u8,
            f3: u8,
            #[educe(Into(u16))]
            f4: u8,
        },
        Tuple(u8, #[educe(Into(u8))] u8, u16, #[educe(Into(u16))] u16),
    }

    let s1 = Enum::Struct {
        f1: 1, f2: 2
    };

    let s2 = Enum2::Struct {
        f1: 1, f2: 2, f3: 3, f4: 4
    };

    let t1 = Enum::Tuple(1, 2);
    let t2 = Enum2::Tuple(1, 2, 3, 4);

    assert_eq!(1u8, s1.into());
    assert_eq!(2u16, s1.into());
    assert_eq!(2u8, s2.into());
    assert_eq!(4u16, s2.into());

    assert_eq!(1u8, t1.into());
    assert_eq!(2u16, t1.into());
    assert_eq!(2u8, t2.into());
    assert_eq!(4u16, t2.into());
}

#[allow(dead_code)]
fn method_1() {
    fn into(v: u16) -> u8 {
        v as u8
    }

    #[derive(Educe)]
    #[educe(Into(u8))]
    enum Enum {
        Struct {
            #[educe(Into(u8, method = into))]
            f1: u16,
        },
        Tuple(u8),
    }

    let s1 = Enum::Struct {
        f1: 1
    };

    assert_eq!(1u8, s1.into());
}

#[allow(dead_code)]
fn method_2() {
    fn into(v: u16) -> u8 {
        v as u8
    }

    #[derive(Educe)]
    #[educe(Into(u8))]
    enum Enum {
        Struct {
            #[educe(Into(u8, method(into)))]
            f1: u16,
        },
        Tuple(u8),
    }

    let s1 = Enum::Struct {
        f1: 1
    };

    assert_eq!(1u8, s1.into());
}

#[test]
fn bound_1() {
    #[derive(Educe)]
    #[educe(Into(u8))]
    enum Enum<T> {
        Struct {
            #[educe(Into(u8))]
            f1: T,
        },
    }

    let s1 = Enum::Struct {
        f1: 1
    };

    assert_eq!(1u8, s1.into());
}

#[test]
fn bound_2() {
    #[derive(Educe)]
    #[educe(Into(u8, bound = "T: Into<u8>"))]
    enum Enum<T> {
        Struct {
            #[educe(Into(u8))]
            f1: T,
        },
    }

    let s1 = Enum::Struct {
        f1: 1
    };

    assert_eq!(1u8, s1.into());
}

#[test]
fn bound_3() {
    #[derive(Educe)]
    #[educe(Into(u8, bound(T: Into<u8>)))]
    enum Enum<T> {
        Struct {
            #[educe(Into(u8))]
            f1: T,
        },
    }

    let s1 = Enum::Struct {
        f1: 1
    };

    assert_eq!(1u8, s1.into());
}
