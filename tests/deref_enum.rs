#![cfg(feature = "Deref")]
#![no_std]

use educe::Educe;

#[allow(dead_code)]
#[test]
fn basic() {
    #[derive(Educe)]
    #[educe(Deref)]
    enum Enum {
        Struct {
            f1: u8,
        },
        Struct2 {
            f1: u8,
            #[educe(Deref)]
            f2: u8,
        },
        Tuple(u8),
        Tuple2(u8, #[educe(Deref)] u8),
    }

    let s1 = Enum::Struct {
        f1: 1
    };

    let s2 = Enum::Struct2 {
        f1: 1, f2: 2
    };

    let t1 = Enum::Tuple(1);

    let t2 = Enum::Tuple2(1, 2);

    assert_eq!(1, *s1);
    assert_eq!(2, *s2);

    assert_eq!(1, *t1);
    assert_eq!(2, *t2);
}

#[test]
fn constant_name() {
    #[allow(non_upper_case_globals)]
    const value: u8 = 100;

    #[derive(Educe)]
    #[educe(Deref)]
    enum Enum {
        Named { value: u8 },
    }

    assert_eq!(7, *Enum::Named {
        value: 7
    });
    assert_eq!(100, value);
}
