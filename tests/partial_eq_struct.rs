#![no_std]

#[macro_use]
extern crate educe;

#[test]
fn basic() {
    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Unit;

    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Struct {
        f1: u8
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Tuple(u8);

    assert!(Unit == Unit);

    assert!(Struct {
        f1: 1
    } == Struct {
        f1: 1
    });

    assert!(Struct {
        f1: 1
    } != Struct {
        f1: 2
    });

    assert!(Tuple(1) == Tuple(1));
    assert!(Tuple(1) != Tuple(2));
}

#[test]
#[allow(dead_code)]
fn ignore() {
    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Struct {
        #[educe(PartialEq(ignore))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Tuple(
        #[educe(PartialEq(ignore))]
        u8,
        u8,
    );

    assert!(Struct {
        f1: 1,
        f2: 2,
    } == Struct {
        f1: 1,
        f2: 2,
    });

    assert!(Struct {
        f1: 1,
        f2: 2,
    } == Struct {
        f1: 2,
        f2: 2,
    });

    assert!(Struct {
        f1: 1,
        f2: 2,
    } != Struct {
        f1: 2,
        f2: 3,
    });

    assert!(Tuple(1, 2) == Tuple(1, 2));
    assert!(Tuple(1, 2) == Tuple(2, 2));
    assert!(Tuple(1, 2) != Tuple(2, 3));
}

#[test]
fn compare_without_trait_1() {
    fn eq(a: &u8, b: &u8) -> bool {
        a != b
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Struct {
        #[educe(PartialEq(compare = "eq"))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Tuple(
        #[educe(PartialEq(compare = "eq"))]
        u8,
        u8,
    );

    assert!(Struct {
        f1: 1,
        f2: 2,
    } != Struct {
        f1: 1,
        f2: 2,
    });

    assert!(Struct {
        f1: 1,
        f2: 2,
    } == Struct {
        f1: 2,
        f2: 2,
    });

    assert!(Struct {
        f1: 1,
        f2: 2,
    } != Struct {
        f1: 2,
        f2: 3,
    });

    assert!(Tuple(1, 2) != Tuple(1, 2));
    assert!(Tuple(1, 2) == Tuple(2, 2));
    assert!(Tuple(1, 2) != Tuple(2, 3));
}

#[test]
fn compare_without_trait_2() {
    fn eq(a: &u8, b: &u8) -> bool {
        a != b
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Struct {
        #[educe(PartialEq(compare("eq")))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Tuple(
        #[educe(PartialEq(compare("eq")))]
        u8,
        u8,
    );

    assert!(Struct {
        f1: 1,
        f2: 2,
    } != Struct {
        f1: 1,
        f2: 2,
    });

    assert!(Struct {
        f1: 1,
        f2: 2,
    } == Struct {
        f1: 2,
        f2: 2,
    });

    assert!(Struct {
        f1: 1,
        f2: 2,
    } != Struct {
        f1: 2,
        f2: 3,
    });

    assert!(Tuple(1, 2) != Tuple(1, 2));
    assert!(Tuple(1, 2) == Tuple(2, 2));
    assert!(Tuple(1, 2) != Tuple(2, 3));
}

#[test]
fn compare_without_trait_3() {
    fn eq(a: &u8, b: &u8) -> bool {
        a != b
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Struct {
        #[educe(PartialEq(compare(method = "eq")))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Tuple(
        #[educe(PartialEq(compare(method = "eq")))]
        u8,
        u8,
    );

    assert!(Struct {
        f1: 1,
        f2: 2,
    } != Struct {
        f1: 1,
        f2: 2,
    });

    assert!(Struct {
        f1: 1,
        f2: 2,
    } == Struct {
        f1: 2,
        f2: 2,
    });

    assert!(Struct {
        f1: 1,
        f2: 2,
    } != Struct {
        f1: 2,
        f2: 3,
    });

    assert!(Tuple(1, 2) != Tuple(1, 2));
    assert!(Tuple(1, 2) == Tuple(2, 2));
    assert!(Tuple(1, 2) != Tuple(2, 3));
}

#[test]
fn compare_without_trait_4() {
    fn eq(a: &u8, b: &u8) -> bool {
        a != b
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Struct {
        #[educe(PartialEq(compare(method("eq"))))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Tuple(
        #[educe(PartialEq(compare(method("eq"))))]
        u8,
        u8,
    );

    assert!(Struct {
        f1: 1,
        f2: 2,
    } != Struct {
        f1: 1,
        f2: 2,
    });

    assert!(Struct {
        f1: 1,
        f2: 2,
    } == Struct {
        f1: 2,
        f2: 2,
    });

    assert!(Struct {
        f1: 1,
        f2: 2,
    } != Struct {
        f1: 2,
        f2: 3,
    });

    assert!(Tuple(1, 2) != Tuple(1, 2));
    assert!(Tuple(1, 2) == Tuple(2, 2));
    assert!(Tuple(1, 2) != Tuple(2, 3));
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
    struct Struct {
        #[educe(PartialEq(compare(trait = "A")))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Tuple(
        #[educe(PartialEq(compare(trait = "A")))]
        u8,
        u8,
    );

    assert!(Struct {
        f1: 1,
        f2: 2,
    } != Struct {
        f1: 1,
        f2: 2,
    });

    assert!(Struct {
        f1: 1,
        f2: 2,
    } == Struct {
        f1: 2,
        f2: 2,
    });

    assert!(Struct {
        f1: 1,
        f2: 2,
    } != Struct {
        f1: 2,
        f2: 3,
    });

    assert!(Tuple(1, 2) != Tuple(1, 2));
    assert!(Tuple(1, 2) == Tuple(2, 2));
    assert!(Tuple(1, 2) != Tuple(2, 3));
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
    struct Struct {
        #[educe(PartialEq(compare(trait ("A"))))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Tuple(
        #[educe(PartialEq(compare(trait ("A"))))]
        u8,
        u8,
    );

    assert!(Struct {
        f1: 1,
        f2: 2,
    } != Struct {
        f1: 1,
        f2: 2,
    });

    assert!(Struct {
        f1: 1,
        f2: 2,
    } == Struct {
        f1: 2,
        f2: 2,
    });

    assert!(Struct {
        f1: 1,
        f2: 2,
    } != Struct {
        f1: 2,
        f2: 3,
    });

    assert!(Tuple(1, 2) != Tuple(1, 2));
    assert!(Tuple(1, 2) == Tuple(2, 2));
    assert!(Tuple(1, 2) != Tuple(2, 3));
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
    struct Struct {
        #[educe(PartialEq(compare(trait = "A", method = "compare")))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Tuple(
        #[educe(PartialEq(compare(trait = "A", method = "compare")))]
        u8,
        u8,
    );

    assert!(Struct {
        f1: 1,
        f2: 2,
    } != Struct {
        f1: 1,
        f2: 2,
    });

    assert!(Struct {
        f1: 1,
        f2: 2,
    } == Struct {
        f1: 2,
        f2: 2,
    });

    assert!(Struct {
        f1: 1,
        f2: 2,
    } != Struct {
        f1: 2,
        f2: 3,
    });

    assert!(Tuple(1, 2) != Tuple(1, 2));
    assert!(Tuple(1, 2) == Tuple(2, 2));
    assert!(Tuple(1, 2) != Tuple(2, 3));
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
    struct Struct {
        #[educe(PartialEq(compare(trait ("A"), method("compare"))))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq)]
    struct Tuple(
        #[educe(PartialEq(compare(trait ("A"), method("compare"))))]
        u8,
        u8,
    );

    assert!(Struct {
        f1: 1,
        f2: 2,
    } != Struct {
        f1: 1,
        f2: 2,
    });

    assert!(Struct {
        f1: 1,
        f2: 2,
    } == Struct {
        f1: 2,
        f2: 2,
    });

    assert!(Struct {
        f1: 1,
        f2: 2,
    } != Struct {
        f1: 2,
        f2: 3,
    });

    assert!(Tuple(1, 2) != Tuple(1, 2));
    assert!(Tuple(1, 2) == Tuple(2, 2));
    assert!(Tuple(1, 2) != Tuple(2, 3));
}

#[test]
fn bound_1() {
    #[derive(Educe)]
    #[educe(PartialEq(bound))]
    struct Struct<T> {
        f1: T
    }

    #[derive(Educe)]
    #[educe(PartialEq(bound))]
    struct Tuple<T>(T);

    assert!(Struct {
        f1: 1
    } == Struct {
        f1: 1
    });

    assert!(Struct {
        f1: 1
    } != Struct {
        f1: 2
    });

    assert!(Tuple(1) == Tuple(1));
    assert!(Tuple(1) != Tuple(2));
}

#[test]
fn bound_2() {
    #[derive(Educe)]
    #[educe(PartialEq(bound = "T: core::cmp::PartialEq"))]
    struct Struct<T> {
        f1: T
    }

    #[derive(Educe)]
    #[educe(PartialEq(bound = "T: core::cmp::PartialEq"))]
    struct Tuple<T>(T);

    assert!(Struct {
        f1: 1
    } == Struct {
        f1: 1
    });

    assert!(Struct {
        f1: 1
    } != Struct {
        f1: 2
    });

    assert!(Tuple(1) == Tuple(1));
    assert!(Tuple(1) != Tuple(2));
}

#[test]
fn bound_3() {
    #[derive(Educe)]
    #[educe(PartialEq(bound("T: core::cmp::PartialEq")))]
    struct Struct<T> {
        f1: T
    }

    #[derive(Educe)]
    #[educe(PartialEq(bound("T: core::cmp::PartialEq")))]
    struct Tuple<T>(T);

    assert!(Struct {
        f1: 1
    } == Struct {
        f1: 1
    });

    assert!(Struct {
        f1: 1
    } != Struct {
        f1: 2
    });

    assert!(Tuple(1) == Tuple(1));
    assert!(Tuple(1) != Tuple(2));
}