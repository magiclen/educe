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
#[test]
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
#[test]
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

#[allow(dead_code)]
#[test]
fn from_impl() {
    // A concrete target type gets a `From` impl, so both directions of the conversion are available.
    #[derive(Educe)]
    #[educe(Into(u8))]
    enum Enum {
        Struct { f1: u8 },
        Tuple(u8),
    }

    assert_eq!(
        1u8,
        u8::from(Enum::Struct {
            f1: 1
        })
    );
    assert_eq!(2u8, u8::from(Enum::Tuple(2)));
}

#[test]
fn source_self_method() {
    #[derive(Educe)]
    #[educe(Into(u16, bound(T: Into<u16>)), Into(u32, into, bound(T: Into<u16>)))]
    enum Enum<T> {
        Tuple(
            #[educe(Into(u16, method = Self::convert), Into(u32, method = Self::convert_wide))] T,
        ),
    }

    impl<T: Into<u16>> Enum<T> {
        fn convert(value: T) -> u16 {
            value.into()
        }

        fn convert_wide(value: T) -> u32 {
            u32::from(value.into())
        }
    }

    assert_eq!(7, u16::from(Enum::Tuple(7u8)));
    assert_eq!(7u32, Into::<u32>::into(Enum::Tuple(7u8)));
}

#[allow(dead_code)]
#[test]
fn reference_targets() {
    #[derive(Educe)]
    #[educe(Into(&'a u8))]
    enum Shared<'a> {
        Named { value: &'a mut u8, unused: bool },
        Tuple(&'a u8, bool),
    }

    #[derive(Educe)]
    #[educe(Into(&'a mut u8, into))]
    enum Mutable<'a> {
        Tuple(&'a mut u8),
    }

    #[derive(Educe)]
    #[educe(Into(&'a &'b u8))]
    enum Nested<'a, 'b> {
        Named { value: &'a &'b u8 },
    }

    let mut number = 7;
    assert_eq!(
        7,
        *<&u8>::from(Shared::Named {
            value: &mut number, unused: false
        })
    );
    assert_eq!(7, *<&u8>::from(Shared::Tuple(&number, true)));
    *Into::<&mut u8>::into(Mutable::Tuple(&mut number)) = 9;
    let reference = &number;
    assert_eq!(
        9,
        **<&&u8>::from(Nested::Named {
            value: &reference
        })
    );
}

#[test]
fn source_self_bounds_and_qualified_method() {
    trait Convert<T> {
        fn convert(value: T) -> u16;
    }
    trait Source<S> {}

    #[derive(Educe)]
    #[educe(Into(u16))]
    enum Enum<T: Source<Self>>
    where
        Self: Convert<T>, {
        Tuple(#[educe(Into(u16, method(<Self as Convert<T>>::convert)))] T),
    }

    impl<T: Into<u16> + Source<Self>> Convert<T> for Enum<T> {
        fn convert(value: T) -> u16 {
            value.into()
        }
    }
    impl Source<Enum<u8>> for u8 {}

    assert_eq!(7, u16::from(Enum::Tuple(7u8)));
}
