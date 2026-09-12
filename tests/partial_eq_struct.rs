#![cfg(feature = "PartialEq")]
#![no_std]

use educe::Educe;

#[test]
fn empty() {
    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Struct {}

    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Tuple();

    assert!(Struct {} == Struct {});
    assert!(Tuple() == Tuple());
}

#[test]
fn basic() {
    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Unit;

    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Struct {
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Tuple(u8);

    assert!(Unit == Unit);

    assert!(
        Struct {
            f1: 1
        } == Struct {
            f1: 1
        }
    );

    assert!(
        Struct {
            f1: 1
        } != Struct {
            f1: 2
        }
    );

    assert!(Tuple(1) == Tuple(1));
    assert!(Tuple(1) != Tuple(2));
}

#[allow(dead_code)]
#[test]
fn ignore_1() {
    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Struct {
        #[educe(PartialEq = false)]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Tuple(#[educe(PartialEq = false)] u8, u8);

    assert!(
        Struct {
            f1: 1, f2: 2
        } == Struct {
            f1: 1, f2: 2
        }
    );

    assert!(
        Struct {
            f1: 1, f2: 2
        } == Struct {
            f1: 2, f2: 2
        }
    );

    assert!(
        Struct {
            f1: 1, f2: 2
        } != Struct {
            f1: 2, f2: 3
        }
    );

    assert!(Tuple(1, 2) == Tuple(1, 2));
    assert!(Tuple(1, 2) == Tuple(2, 2));
    assert!(Tuple(1, 2) != Tuple(2, 3));
}

#[allow(dead_code)]
#[test]
fn ignore_2() {
    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Struct {
        #[educe(PartialEq(ignore))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Tuple(#[educe(PartialEq(ignore))] u8, u8);

    assert!(
        Struct {
            f1: 1, f2: 2
        } == Struct {
            f1: 1, f2: 2
        }
    );

    assert!(
        Struct {
            f1: 1, f2: 2
        } == Struct {
            f1: 2, f2: 2
        }
    );

    assert!(
        Struct {
            f1: 1, f2: 2
        } != Struct {
            f1: 2, f2: 3
        }
    );

    assert!(Tuple(1, 2) == Tuple(1, 2));
    assert!(Tuple(1, 2) == Tuple(2, 2));
    assert!(Tuple(1, 2) != Tuple(2, 3));
}

#[test]
fn method_1() {
    fn eq(a: &u8, b: &u8) -> bool {
        a != b
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Struct {
        #[educe(PartialEq(method = eq))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Tuple(#[educe(PartialEq(method = eq))] u8, u8);

    assert!(
        Struct {
            f1: 1, f2: 2
        } != Struct {
            f1: 1, f2: 2
        }
    );

    assert!(
        Struct {
            f1: 1, f2: 2
        } == Struct {
            f1: 2, f2: 2
        }
    );

    assert!(
        Struct {
            f1: 1, f2: 2
        } != Struct {
            f1: 2, f2: 3
        }
    );

    assert!(Tuple(1, 2) != Tuple(1, 2));
    assert!(Tuple(1, 2) == Tuple(2, 2));
    assert!(Tuple(1, 2) != Tuple(2, 3));
}

#[test]
fn method_2() {
    fn eq(a: &u8, b: &u8) -> bool {
        a != b
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Struct {
        #[educe(PartialEq(method(eq)))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Tuple(#[educe(PartialEq(method(eq)))] u8, u8);

    assert!(
        Struct {
            f1: 1, f2: 2
        } != Struct {
            f1: 1, f2: 2
        }
    );

    assert!(
        Struct {
            f1: 1, f2: 2
        } == Struct {
            f1: 2, f2: 2
        }
    );

    assert!(
        Struct {
            f1: 1, f2: 2
        } != Struct {
            f1: 2, f2: 3
        }
    );

    assert!(Tuple(1, 2) != Tuple(1, 2));
    assert!(Tuple(1, 2) == Tuple(2, 2));
    assert!(Tuple(1, 2) != Tuple(2, 3));
}

#[test]
fn bound_1() {
    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Tuple<T>(T);

    assert!(
        Struct {
            f1: 1
        } == Struct {
            f1: 1
        }
    );

    assert!(
        Struct {
            f1: 1
        } != Struct {
            f1: 2
        }
    );

    assert!(Tuple(1) == Tuple(1));
    assert!(Tuple(1) != Tuple(2));
}

#[test]
fn bound_2() {
    #[derive(Educe)]
    #[educe(PartialEq(bound = "T: core::cmp::PartialEq"))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(PartialEq(bound = "T: core::cmp::PartialEq"))]
    struct Tuple<T>(T);

    assert!(
        Struct {
            f1: 1
        } == Struct {
            f1: 1
        }
    );

    assert!(
        Struct {
            f1: 1
        } != Struct {
            f1: 2
        }
    );

    assert!(Tuple(1) == Tuple(1));
    assert!(Tuple(1) != Tuple(2));
}

#[test]
fn bound_3() {
    #[derive(Educe)]
    #[educe(PartialEq(bound(T: core::cmp::PartialEq)))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(PartialEq(bound(T: core::cmp::PartialEq)))]
    struct Tuple<T>(T);

    assert!(
        Struct {
            f1: 1
        } == Struct {
            f1: 1
        }
    );

    assert!(
        Struct {
            f1: 1
        } != Struct {
            f1: 2
        }
    );

    assert!(Tuple(1) == Tuple(1));
    assert!(Tuple(1) != Tuple(2));
}

#[test]
fn explicit_bounds() {
    // `bound(*)` adds a predicate for every type parameter, and `bound(false)` adds none, so the impl relies on what the type itself declares.
    #[derive(Educe)]
    #[educe(PartialEq(bound(*)))]
    struct All<T>(T, u8);

    #[derive(Educe)]
    #[educe(PartialEq(bound(false)))]
    struct Disabled<T: PartialEq>(T);

    assert!(All(1u8, 2u8) == All(1u8, 2u8));
    assert!(Disabled(1u8) != Disabled(2u8));
}
