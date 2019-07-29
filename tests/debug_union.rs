#![no_std]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate educe;

#[test]
#[allow(dead_code)]
fn name_1() {
    #[derive(Educe)]
    #[educe(Debug)]
    union Union {
        f1: u8
    }

    assert_eq!("Union([1])", format!("{:?}", Union {
        f1: 1
    }));
}

#[test]
#[allow(dead_code)]
fn name_2() {
    #[derive(Educe)]
    #[educe(Debug = "A")]
    union Union {
        f1: u8
    }

    assert_eq!("A([1])", format!("{:?}", Union {
        f1: 1
    }));
}

#[test]
#[allow(dead_code)]
fn name_3() {
    #[derive(Educe)]
    #[educe(Debug("A"))]
    union Union {
        f1: u8
    }

    assert_eq!("A([1])", format!("{:?}", Union {
        f1: 1
    }));
}

#[test]
#[allow(dead_code)]
fn name_4() {
    #[derive(Educe)]
    #[educe(Debug(name = "A"))]
    union Union {
        f1: u8
    }

    assert_eq!("A([1])", format!("{:?}", Union {
        f1: 1
    }));
}

#[test]
#[allow(dead_code)]
fn name_5() {
    #[derive(Educe)]
    #[educe(Debug(name("A")))]
    union Union {
        f1: u8
    }

    assert_eq!("A([1])", format!("{:?}", Union {
        f1: 1
    }));
}

#[test]
#[allow(dead_code)]
fn unnamed_1() {
    #[derive(Educe)]
    #[educe(Debug(name = false))]
    union Union {
        f1: u8
    }

    assert_eq!("[1]", format!("{:?}", Union {
        f1: 1
    }));
}

#[test]
#[allow(dead_code)]
fn unnamed_2() {
    #[derive(Educe)]
    #[educe(Debug(name(false)))]
    union Union {
        f1: u8
    }

    assert_eq!("[1]", format!("{:?}", Union {
        f1: 1
    }));
}

#[test]
#[allow(dead_code)]
fn unnamed_3() {
    #[derive(Educe)]
    #[educe(Debug = "")]
    union Union {
        f1: u8
    }

    assert_eq!("[1]", format!("{:?}", Union {
        f1: 1
    }));
}

#[test]
#[allow(dead_code)]
fn unnamed_4() {
    #[derive(Educe)]
    #[educe(Debug(""))]
    union Union {
        f1: u8
    }

    assert_eq!("[1]", format!("{:?}", Union {
        f1: 1
    }));
}

#[test]
#[allow(dead_code)]
fn unnamed_5() {
    #[derive(Educe)]
    #[educe(Debug(name = ""))]
    union Union {
        f1: u8
    }

    assert_eq!("[1]", format!("{:?}", Union {
        f1: 1
    }));
}

#[test]
#[allow(dead_code)]
fn unnamed_6() {
    #[derive(Educe)]
    #[educe(Debug(name("")))]
    union Union {
        f1: u8
    }

    assert_eq!("[1]", format!("{:?}", Union {
        f1: 1
    }));
}

#[test]
#[allow(dead_code)]
fn bound_1() {
    #[derive(Educe)]
    #[educe(Debug(bound))]
    union Union<T: Copy> {
        f1: T
    }

    assert_eq!("Union([1])", format!("{:?}", Union {
        f1: 1u8
    }));
}

#[test]
#[allow(dead_code)]
fn bound_2() {
    #[derive(Educe)]
    #[educe(Debug(bound = "T: core::fmt::Debug"))]
    union Union<T: Copy> {
        f1: T
    }

    assert_eq!("Union([1])", format!("{:?}", Union {
        f1: 1u8
    }));
}

#[test]
#[allow(dead_code)]
fn bound_3() {
    #[derive(Educe)]
    #[educe(Debug(bound("T: core::fmt::Debug")))]
    union Union<T: Copy> {
        f1: T
    }

    assert_eq!("Union([1])", format!("{:?}", Union {
        f1: 1u8
    }));
}