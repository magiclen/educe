#![cfg(all(feature = "Eq", feature = "PartialEq"))]
#![no_std]

use educe::Educe;

#[test]
fn empty() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Struct {}

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Tuple();

    assert!(Struct {} == Struct {});
    assert!(Tuple() == Tuple());
}

#[test]
fn basic() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Unit;

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Struct {
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
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
    #[educe(PartialEq, Eq)]
    struct Struct {
        #[educe(Eq = false)]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Tuple(#[educe(Eq = false)] u8, u8);

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
    #[educe(PartialEq, Eq)]
    struct Struct {
        #[educe(Eq(ignore))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Tuple(#[educe(Eq(ignore))] u8, u8);

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
    #[educe(PartialEq, Eq)]
    struct Struct {
        #[educe(Eq(method = eq))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Tuple(#[educe(Eq(method = eq))] u8, u8);

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
    #[educe(PartialEq, Eq)]
    struct Struct {
        #[educe(Eq(method(eq)))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Tuple(#[educe(Eq(method(eq)))] u8, u8);

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
    #[educe(PartialEq, Eq)]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
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
    #[educe(PartialEq(bound = "T: core::cmp::PartialEq"), Eq)]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(PartialEq(bound = "T: core::cmp::PartialEq"), Eq)]
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
    #[educe(PartialEq(bound(T: core::cmp::PartialEq)), Eq)]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(PartialEq(bound(T: core::cmp::PartialEq)), Eq)]
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

#[allow(dead_code)]
#[test]
fn use_partial_eq_attr_ignore() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Struct {
        #[educe(PartialEq(ignore))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
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
