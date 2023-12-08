#![cfg(feature = "Debug")]
#![no_std]

#[macro_use]
extern crate alloc;

use educe::Educe;

#[allow(dead_code)]
#[test]
fn name_1() {
    #[derive(Educe)]
    #[educe(Debug(unsafe))]
    union Union {
        f1: u8,
    }

    assert_eq!(
        "Union([1])",
        format!("{:?}", Union {
            f1: 1
        })
    );
}

#[allow(dead_code)]
#[test]
fn name_2() {
    #[derive(Educe)]
    #[educe(Debug(unsafe, name = A))]
    union Union {
        f1: u8,
    }

    assert_eq!(
        "A([1])",
        format!("{:?}", Union {
            f1: 1
        })
    );
}

#[allow(dead_code)]
#[test]
fn name_3() {
    #[derive(Educe)]
    #[educe(Debug(unsafe, name(A)))]
    union Union {
        f1: u8,
    }

    assert_eq!(
        "A([1])",
        format!("{:?}", Union {
            f1: 1
        })
    );
}

#[allow(dead_code)]
#[test]
fn unnamed_1() {
    #[derive(Educe)]
    #[educe(Debug(unsafe, name = false))]
    union Union {
        f1: u8,
    }

    assert_eq!(
        "[1]",
        format!("{:?}", Union {
            f1: 1
        })
    );
}

#[allow(dead_code)]
#[test]
fn unnamed_2() {
    #[derive(Educe)]
    #[educe(Debug(unsafe, name(false)))]
    union Union {
        f1: u8,
    }

    assert_eq!(
        "[1]",
        format!("{:?}", Union {
            f1: 1
        })
    );
}

#[allow(dead_code)]
#[test]
fn unnamed_3() {
    #[derive(Educe)]
    #[educe(Debug(unsafe, name = ""))]
    union Union {
        f1: u8,
    }

    assert_eq!(
        "[1]",
        format!("{:?}", Union {
            f1: 1
        })
    );
}

#[allow(dead_code)]
#[test]
fn unnamed_4() {
    #[derive(Educe)]
    #[educe(Debug(unsafe, name("")))]
    union Union {
        f1: u8,
    }

    assert_eq!(
        "[1]",
        format!("{:?}", Union {
            f1: 1
        })
    );
}

#[allow(dead_code)]
#[test]
fn bound() {
    #[derive(Educe)]
    #[educe(Debug(unsafe))]
    union Union<T: Copy> {
        f1: T,
    }

    assert_eq!(
        "Union([1])",
        format!("{:?}", Union {
            f1: 1u8
        })
    );
}
