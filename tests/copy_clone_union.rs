#![cfg(all(feature = "Copy", feature = "Clone"))]
#![no_std]
#![allow(clippy::clone_on_copy)]

use educe::Educe;

#[test]
fn basic() {
    #[derive(Educe)]
    #[educe(Copy, Clone)]
    union Union {
        f1: u8,
    }

    let u = Union {
        f1: 1
    }
    .clone();

    assert_eq!(1, unsafe { u.f1 });
}

#[test]
fn bound() {
    #[derive(Educe)]
    #[educe(Copy, Clone)]
    union Union<T: Copy> {
        f1: T,
    }

    let u = Union {
        f1: 1
    }
    .clone();

    assert_eq!(1, unsafe { u.f1 });
}
