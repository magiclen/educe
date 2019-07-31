#![no_std]

#[macro_use]
extern crate educe;

#[test]
fn basic() {
    #[derive(Educe)]
    #[educe(Clone)]
    struct Unit;

    #[derive(Educe)]
    #[educe(Clone)]
    struct Struct {
        f1: u8
    }

    #[derive(Educe)]
    #[educe(Clone)]
    struct Tuple(u8);
}