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

#[test]
fn source_self_method() {
    #[derive(Educe)]
    #[educe(Into(u16, bound(T: Into<u16>)), Into(u32, into, bound(T: Into<u16>)))]
    struct Struct<T> {
        #[educe(Into(u16, method = Self::convert), Into(u32, method = Self::convert_wide))]
        value: T,
    }

    impl<T: Into<u16>> Struct<T> {
        fn convert(value: T) -> u16 {
            value.into()
        }

        fn convert_wide(value: T) -> u32 {
            u32::from(value.into())
        }
    }

    assert_eq!(
        7,
        u16::from(Struct {
            value: 7u8
        })
    );
    assert_eq!(
        7u32,
        Into::<u32>::into(Struct {
            value: 7u8
        })
    );
}

#[test]
fn reference_targets() {
    #[derive(Educe)]
    #[educe(Into(&'a u8))]
    struct Shared<'a>(&'a u8);

    #[derive(Educe)]
    #[educe(Into(&'a mut u8))]
    struct Mutable<'a>(&'a mut u8);

    #[derive(Educe)]
    #[educe(Into(&'a &'b u8))]
    struct Nested<'a, 'b>(&'a &'b u8);

    #[derive(Educe)]
    #[educe(Into(&u8))]
    struct Static(&'static u8);

    #[derive(Educe)]
    #[educe(Into(&'a u8, into))]
    struct Direct<'a>(&'a mut u8);

    #[derive(Educe)]
    #[educe(Into(&'a u8), Into(&'a &'a u8))]
    struct DifferentDepths<'a> {
        one: &'a u8,
        two: &'a &'a u8,
    }

    let mut number = 7;
    assert_eq!(7, *<&u8>::from(Shared(&number)));
    *<&mut u8>::from(Mutable(&mut number)) = 9;
    let reference = &number;
    assert_eq!(9, **<&&u8>::from(Nested(&reference)));
    assert_eq!(
        9,
        *<&u8>::from(DifferentDepths {
            one: &number, two: &reference
        })
    );
    assert_eq!(
        9,
        **<&&u8>::from(DifferentDepths {
            one: &number, two: &reference
        })
    );
    assert_eq!(3, *<&'static u8>::from(Static(&3)));
    assert_eq!(9, *Into::<&u8>::into(Direct(&mut number)));
}

#[test]
fn source_self_bounds_and_qualified_method() {
    trait Convert<T> {
        fn convert(value: T) -> u16;
    }
    trait Target {}
    impl Target for u16 {}

    #[derive(Educe)]
    #[educe(Into(u16, bound(Self: Target)))]
    struct Struct<T>(#[educe(Into(u16, method = <Self as Convert<T>>::convert))] T)
    where
        Self: Convert<T>;

    impl<T: Into<u16>> Convert<T> for Struct<T> {
        fn convert(value: T) -> u16 {
            value.into()
        }
    }

    #[derive(Educe)]
    #[educe(Into(u16, into, bound(Self: Convert<T>)))]
    struct Direct<T>(#[educe(Into(u16, method = <Self as Convert<T>>::convert))] T);

    impl<T: Into<u16>> Convert<T> for Direct<T> {
        fn convert(value: T) -> u16 {
            value.into()
        }
    }

    assert_eq!(7, u16::from(Struct(7u8)));
    assert_eq!(8u16, Into::<u16>::into(Direct(8u8)));
}
