#![cfg(feature = "PartialEq")]
#![no_std]

use educe::Educe;

#[allow(dead_code)]
#[test]
fn basic() {
    #[derive(Educe)]
    #[educe(PartialEq(unsafe))]
    union Union {
        f1: u8,
    }

    assert!(
        Union {
            f1: 1
        } == Union {
            f1: 1
        }
    );

    assert!(
        Union {
            f1: 1
        } != Union {
            f1: 2
        }
    );
}

#[allow(dead_code)]
#[test]
fn generic() {
    #[derive(Educe)]
    #[educe(PartialEq(unsafe))]
    union Union<T: Copy> {
        f1: T,
    }

    assert!(
        Union {
            f1: 1
        } == Union {
            f1: 1
        }
    );

    assert!(
        Union {
            f1: 1
        } != Union {
            f1: 2
        }
    );
}

#[allow(dead_code)]
#[test]
fn bound() {
    #[derive(Educe)]
    #[educe(PartialEq(unsafe, bound(T: Copy + core::cmp::PartialEq)))]
    union Union<T: Copy> {
        f1: T,
    }

    fn assert_partial_eq_impl<T: PartialEq>() {}

    assert_partial_eq_impl::<Union<u8>>();

    assert!(
        Union {
            f1: 1
        } == Union {
            f1: 1
        }
    );
}

#[allow(dead_code)]
#[test]
fn bound_all() {
    #[derive(Educe)]
    #[educe(PartialEq(unsafe, bound(*)))]
    union Union<T: Copy> {
        f1: T,
    }

    assert!(
        Union {
            f1: 1u8
        } == Union {
            f1: 1u8
        }
    );
}
