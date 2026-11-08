#![cfg(all(feature = "Copy", feature = "Clone"))]
#![no_std]
#![allow(clippy::clone_on_copy)]

use core::marker::PhantomData;

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

#[test]
fn generic_union() {
    // A generic union stays bitwise-copied, and its parameters receive a `Copy` bound.
    #[derive(Educe)]
    #[educe(Copy, Clone)]
    union Union<T: Copy> {
        f1: T,
    }

    let u = Union {
        f1: 1u8
    };

    let v = u.clone();

    assert_eq!(1, unsafe { v.f1 });
}

#[test]
fn explicit_copy_bounds() {
    #[derive(Educe)]
    #[educe(Copy(bound('a: 'static)), Clone)]
    union Borrowed<'a> {
        value: &'a str,
    }

    #[derive(Educe)]
    #[educe(Copy(bound(*)), Clone)]
    union Marker<T> {
        marker: PhantomData<T>,
    }

    let value = Borrowed {
        value: "value"
    };
    let marker = Marker {
        marker: PhantomData::<u8>
    };

    assert_eq!("value", unsafe { value.clone().value });
    assert_eq!(PhantomData::<u8>, unsafe { marker.clone().marker });
}
