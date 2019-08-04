#![cfg(all(feature = "PartialEq", feature = "Eq"))]
#![no_std]

#[macro_use]
extern crate educe;

#[test]
#[allow(dead_code)]
fn basic() {
    #[derive(Educe)]
    #[educe(Eq)]
    union Union {
        f1: u8,
    }

    impl core::cmp::PartialEq for Union {
        fn eq(&self, _other: &Union) -> bool {
            true
        }
    }
}

#[test]
#[allow(dead_code)]
fn bound_1() {
    #[derive(Educe)]
    #[educe(Eq(bound))]
    union Union<T: Copy> {
        f1: T,
    }

    impl<T: Copy> core::cmp::PartialEq for Union<T> {
        fn eq(&self, _other: &Union<T>) -> bool {
            true
        }
    }
}

#[test]
#[allow(dead_code)]
fn bound_2() {
    #[derive(Educe)]
    #[educe(Eq(bound = "T: core::cmp::Eq"))]
    union Union<T: Copy> {
        f1: T,
    }

    impl<T: Copy> core::cmp::PartialEq for Union<T> {
        fn eq(&self, _other: &Union<T>) -> bool {
            true
        }
    }
}

#[test]
#[allow(dead_code)]
fn bound_3() {
    #[derive(Educe)]
    #[educe(Eq(bound = "T: core::cmp::PartialEq"))]
    union Union<T: Copy> {
        f1: T,
    }

    impl<T: Copy> core::cmp::PartialEq for Union<T> {
        fn eq(&self, _other: &Union<T>) -> bool {
            true
        }
    }
}
