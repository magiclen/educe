#![cfg(all(feature = "Eq", feature = "PartialEq"))]
#![no_std]

use educe::Educe;

#[allow(dead_code)]
#[test]
fn basic() {
    #[derive(Educe)]
    #[educe(PartialEq(unsafe), Eq)]
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
fn bound() {
    #[derive(Educe)]
    #[educe(PartialEq(unsafe), Eq)]
    union Union<T: Copy> {
        f1: T,
    }

    fn assert_eq_impl<T: Eq>() {}

    assert_eq_impl::<Union<u8>>();

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
