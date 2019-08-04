#![cfg(all(feature = "Copy", feature = "Clone"))]
#![no_std]

#[macro_use]
extern crate educe;

#[test]
fn basic() {
    #[derive(Educe)]
    #[educe(Copy, Clone)]
    union Union {
        f1: u8,
    }

    let u = Union { f1: 1 }.clone();

    assert_eq!(1, unsafe { u.f1 });
}
