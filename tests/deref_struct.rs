#![cfg(feature = "Deref")]
#![no_std]

#[macro_use]
extern crate educe;

#[test]
#[allow(dead_code)]
fn basic() {
    #[derive(Educe)]
    #[educe(Deref)]
    struct Struct {
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Deref)]
    struct Struct2 {
        f1: u8,
        #[educe(Deref)]
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(Deref)]
    struct Tuple(u8);

    #[derive(Educe)]
    #[educe(Deref)]
    struct Tuple2(u8, #[educe(Deref)] u8);

    let s1 = Struct {
        f1: 1
    };

    let s2 = Struct2 {
        f1: 1, f2: 2
    };

    let t1 = Tuple(1);
    let t2 = Tuple2(1, 2);

    assert_eq!(1, *s1);
    assert_eq!(2, *s2);

    assert_eq!(1, *t1);
    assert_eq!(2, *t2);
}
