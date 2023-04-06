#![allow(clippy::eq_op, clippy::trivially_copy_pass_by_ref)]
#![cfg(feature = "PartialEq")]
#![no_std]

#[macro_use]
extern crate educe;

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

#[test]
#[allow(dead_code)]
fn ignore() {
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
fn compare_without_trait_1() {
    fn eq(a: &u8, b: &u8) -> bool {
        a != b
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    enum Enum {
        Struct {
            #[educe(PartialEq(method = "eq"))]
            f1: u8,
            f2: u8,
        },
        Tuple(#[educe(PartialEq(method = "eq"))] u8, u8),
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
fn compare_without_trait_2() {
    fn eq(a: &u8, b: &u8) -> bool {
        a != b
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    enum Enum {
        Struct {
            #[educe(PartialEq(method("eq")))]
            f1: u8,
            f2: u8,
        },
        Tuple(#[educe(PartialEq(method("eq")))] u8, u8),
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
fn compare_with_trait_1() {
    trait A {
        fn eq(&self, b: &Self) -> bool;
    }

    impl A for u8 {
        fn eq(&self, b: &u8) -> bool {
            self != b
        }
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    enum Enum {
        Struct {
            #[educe(PartialEq(trait = "A"))]
            f1: u8,
            f2: u8,
        },
        Tuple(#[educe(PartialEq(trait = "A"))] u8, u8),
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
fn compare_with_trait_2() {
    trait A {
        fn eq(&self, b: &Self) -> bool;
    }

    impl A for u8 {
        fn eq(&self, b: &u8) -> bool {
            self != b
        }
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    enum Enum {
        Struct {
            #[educe(PartialEq(trait("A")))]
            f1: u8,
            f2: u8,
        },
        Tuple(#[educe(PartialEq(trait("A")))] u8, u8),
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
fn compare_with_trait_3() {
    trait A {
        fn compare(&self, b: &Self) -> bool;
    }

    impl A for u8 {
        fn compare(&self, b: &u8) -> bool {
            self != b
        }
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    enum Enum {
        Struct {
            #[educe(PartialEq(trait = "A", method = "compare"))]
            f1: u8,
            f2: u8,
        },
        Tuple(#[educe(PartialEq(trait = "A", method = "compare"))] u8, u8),
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
fn compare_with_trait_4() {
    trait A {
        fn compare(&self, b: &Self) -> bool;
    }

    impl A for u8 {
        fn compare(&self, b: &u8) -> bool {
            self != b
        }
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    enum Enum {
        Struct {
            #[educe(PartialEq(trait("A"), method("compare")))]
            f1: u8,
            f2: u8,
        },
        Tuple(#[educe(PartialEq(trait("A"), method("compare")))] u8, u8),
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
    #[educe(PartialEq(bound))]
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
    #[educe(PartialEq(bound("T: core::cmp::PartialEq")))]
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
