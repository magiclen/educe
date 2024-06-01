#![cfg(feature = "PartialEq")]
#![no_std]

use educe::Educe;

#[test]
fn empty() {
    #[allow(dead_code)]
    #[derive(Educe)]
    #[educe(PartialEq)]
    enum Enum {}

    #[derive(Educe)]
    #[educe(PartialEq)]
    enum Enum2 {
        Struct {},
        Tuple(),
    }

    assert!(Enum2::Struct {} == Enum2::Struct {});
    assert!(Enum2::Tuple() == Enum2::Tuple());
}

#[test]
fn basic() {
    #[derive(Educe)]
    #[educe(PartialEq)]
    enum Enum {
        Unit,
        Unit2,
        Struct { f1: u8 },
        Tuple(u8),
    }

    assert!(Enum::Unit == Enum::Unit);
    assert!(Enum::Unit != Enum::Unit2);

    assert!(
        Enum::Struct {
            f1: 1
        } == Enum::Struct {
            f1: 1
        }
    );

    assert!(
        Enum::Struct {
            f1: 1
        } != Enum::Struct {
            f1: 2
        }
    );

    assert!(Enum::Tuple(1) == Enum::Tuple(1));
    assert!(Enum::Tuple(1) != Enum::Tuple(2));
}

#[allow(dead_code)]
#[test]
fn ignore_1() {
    #[derive(Educe)]
    #[educe(PartialEq)]
    enum Enum {
        Struct {
            #[educe(PartialEq = false)]
            f1: u8,
            f2: u8,
        },
        Tuple(#[educe(PartialEq = false)] u8, u8),
    }

    assert!(
        Enum::Struct {
            f1: 1, f2: 2
        } == Enum::Struct {
            f1: 1, f2: 2
        }
    );

    assert!(
        Enum::Struct {
            f1: 1, f2: 2
        } == Enum::Struct {
            f1: 2, f2: 2
        }
    );

    assert!(
        Enum::Struct {
            f1: 1, f2: 2
        } != Enum::Struct {
            f1: 2, f2: 3
        }
    );

    assert!(Enum::Tuple(1, 2) == Enum::Tuple(1, 2));
    assert!(Enum::Tuple(1, 2) == Enum::Tuple(2, 2));
    assert!(Enum::Tuple(1, 2) != Enum::Tuple(2, 3));
}

#[allow(dead_code)]
#[test]
fn ignore_2() {
    #[derive(Educe)]
    #[educe(PartialEq)]
    enum Enum {
        Struct {
            #[educe(PartialEq(ignore))]
            f1: u8,
            f2: u8,
        },
        Tuple(#[educe(PartialEq(ignore))] u8, u8),
    }

    assert!(
        Enum::Struct {
            f1: 1, f2: 2
        } == Enum::Struct {
            f1: 1, f2: 2
        }
    );

    assert!(
        Enum::Struct {
            f1: 1, f2: 2
        } == Enum::Struct {
            f1: 2, f2: 2
        }
    );

    assert!(
        Enum::Struct {
            f1: 1, f2: 2
        } != Enum::Struct {
            f1: 2, f2: 3
        }
    );

    assert!(Enum::Tuple(1, 2) == Enum::Tuple(1, 2));
    assert!(Enum::Tuple(1, 2) == Enum::Tuple(2, 2));
    assert!(Enum::Tuple(1, 2) != Enum::Tuple(2, 3));
}

#[test]
fn method_1() {
    fn eq(a: &u8, b: &u8) -> bool {
        a != b
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    enum Enum {
        Struct {
            #[educe(PartialEq(method = eq))]
            f1: u8,
            f2: u8,
        },
        Tuple(#[educe(PartialEq(method = eq))] u8, u8),
    }

    assert!(
        Enum::Struct {
            f1: 1, f2: 2
        } != Enum::Struct {
            f1: 1, f2: 2
        }
    );

    assert!(
        Enum::Struct {
            f1: 1, f2: 2
        } == Enum::Struct {
            f1: 2, f2: 2
        }
    );

    assert!(
        Enum::Struct {
            f1: 1, f2: 2
        } != Enum::Struct {
            f1: 2, f2: 3
        }
    );

    assert!(Enum::Tuple(1, 2) != Enum::Tuple(1, 2));
    assert!(Enum::Tuple(1, 2) == Enum::Tuple(2, 2));
    assert!(Enum::Tuple(1, 2) != Enum::Tuple(2, 3));
}

#[test]
fn method_2() {
    fn eq(a: &u8, b: &u8) -> bool {
        a != b
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    enum Enum {
        Struct {
            #[educe(PartialEq(method(eq)))]
            f1: u8,
            f2: u8,
        },
        Tuple(#[educe(PartialEq(method(eq)))] u8, u8),
    }

    assert!(
        Enum::Struct {
            f1: 1, f2: 2
        } != Enum::Struct {
            f1: 1, f2: 2
        }
    );

    assert!(
        Enum::Struct {
            f1: 1, f2: 2
        } == Enum::Struct {
            f1: 2, f2: 2
        }
    );

    assert!(
        Enum::Struct {
            f1: 1, f2: 2
        } != Enum::Struct {
            f1: 2, f2: 3
        }
    );

    assert!(Enum::Tuple(1, 2) != Enum::Tuple(1, 2));
    assert!(Enum::Tuple(1, 2) == Enum::Tuple(2, 2));
    assert!(Enum::Tuple(1, 2) != Enum::Tuple(2, 3));
}

#[test]
fn bound_1() {
    #[derive(Educe)]
    #[educe(PartialEq)]
    enum Enum<T> {
        Unit,
        Struct { f1: T },
        Tuple(T),
    }

    assert!(Enum::<u8>::Unit == Enum::<u8>::Unit);

    assert!(
        Enum::Struct {
            f1: 1
        } == Enum::Struct {
            f1: 1
        }
    );

    assert!(
        Enum::Struct {
            f1: 1
        } != Enum::Struct {
            f1: 2
        }
    );

    assert!(Enum::Tuple(1) == Enum::Tuple(1));
    assert!(Enum::Tuple(1) != Enum::Tuple(2));
}

#[test]
fn bound_2() {
    #[derive(Educe)]
    #[educe(PartialEq(bound = "T: core::cmp::PartialEq"))]
    enum Enum<T> {
        Unit,
        Struct { f1: T },
        Tuple(T),
    }

    assert!(Enum::<u8>::Unit == Enum::<u8>::Unit);

    assert!(
        Enum::Struct {
            f1: 1
        } == Enum::Struct {
            f1: 1
        }
    );

    assert!(
        Enum::Struct {
            f1: 1
        } != Enum::Struct {
            f1: 2
        }
    );

    assert!(Enum::Tuple(1) == Enum::Tuple(1));
    assert!(Enum::Tuple(1) != Enum::Tuple(2));
}

#[test]
fn bound_3() {
    #[derive(Educe)]
    #[educe(PartialEq(bound(T: core::cmp::PartialEq)))]
    enum Enum<T> {
        Unit,
        Struct { f1: T },
        Tuple(T),
    }

    assert!(Enum::<u8>::Unit == Enum::<u8>::Unit);

    assert!(
        Enum::Struct {
            f1: 1
        } == Enum::Struct {
            f1: 1
        }
    );

    assert!(
        Enum::Struct {
            f1: 1
        } != Enum::Struct {
            f1: 2
        }
    );

    assert!(Enum::Tuple(1) == Enum::Tuple(1));
    assert!(Enum::Tuple(1) != Enum::Tuple(2));
}
