#![allow(clippy::eq_op, clippy::trivially_copy_pass_by_ref)]
#![cfg(all(feature = "PartialEq", feature = "Eq", feature = "PartialOrd", feature = "Ord"))]
#![no_std]

#[macro_use]
extern crate educe;

use core::cmp::Ordering;

#[test]
fn basic_1() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Unit;

    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Struct {
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Tuple(u8);

    assert!(Unit == Unit);

    assert_eq!(
        Ordering::Greater,
        Struct {
            f1: 2
        }
        .cmp(&Struct {
            f1: 1
        })
    );

    assert_eq!(
        Ordering::Less,
        Struct {
            f1: 1
        }
        .cmp(&Struct {
            f1: 2
        })
    );

    assert_eq!(Ordering::Greater, Tuple(2).cmp(&Tuple(1)));
    assert_eq!(Ordering::Less, Tuple(1).cmp(&Tuple(2)));
}

#[test]
fn basic_2() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Struct {
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Tuple(u8, u8);

    assert_eq!(
        Ordering::Greater,
        Struct {
            f1: 2, f2: 1
        }
        .cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(
        Ordering::Less,
        Struct {
            f1: 1, f2: 2
        }
        .cmp(&Struct {
            f1: 2, f2: 1
        })
    );

    assert_eq!(Ordering::Greater, Tuple(2, 1).cmp(&Tuple(1, 2)));
    assert_eq!(Ordering::Less, Tuple(1, 2).cmp(&Tuple(2, 1)));
}

#[test]
#[allow(dead_code)]
fn ignore() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Struct {
        #[educe(Ord(ignore))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Tuple(#[educe(Ord(ignore))] u8, u8);

    assert_eq!(
        Ordering::Greater,
        Struct {
            f1: 1, f2: 3
        }
        .cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(
        Ordering::Less,
        Struct {
            f1: 1, f2: 2
        }
        .cmp(&Struct {
            f1: 1, f2: 3
        })
    );

    assert_eq!(
        Ordering::Equal,
        Struct {
            f1: 2, f2: 2
        }
        .cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(Ordering::Greater, Tuple(1, 3).cmp(&Tuple(1, 2)));
    assert_eq!(Ordering::Less, Tuple(1, 2).cmp(&Tuple(1, 3)));
    assert_eq!(Ordering::Equal, Tuple(2, 2).cmp(&Tuple(1, 2)));
}

#[test]
fn compare_without_trait_1() {
    fn cmp(a: &u8, b: &u8) -> Ordering {
        b.cmp(a)
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Struct {
        f1: u8,
        #[educe(Ord(method = "cmp"))]
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Tuple(u8, #[educe(Ord(method = "cmp"))] u8);

    assert_eq!(
        Ordering::Less,
        Struct {
            f1: 1, f2: 3
        }
        .cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(
        Ordering::Greater,
        Struct {
            f1: 1, f2: 2
        }
        .cmp(&Struct {
            f1: 1, f2: 3
        })
    );

    assert_eq!(
        Ordering::Equal,
        Struct {
            f1: 1, f2: 2
        }
        .cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(Ordering::Less, Tuple(1, 3).cmp(&Tuple(1, 2)));
    assert_eq!(Ordering::Greater, Tuple(1, 2).cmp(&Tuple(1, 3)));
    assert_eq!(Ordering::Equal, Tuple(1, 2).cmp(&Tuple(1, 2)));
}

#[test]
fn compare_without_trait_2() {
    fn cmp(a: &u8, b: &u8) -> Ordering {
        b.cmp(a)
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Struct {
        f1: u8,
        #[educe(Ord(method("cmp")))]
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Tuple(u8, #[educe(Ord(method("cmp")))] u8);

    assert_eq!(
        Ordering::Less,
        Struct {
            f1: 1, f2: 3
        }
        .cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(
        Ordering::Greater,
        Struct {
            f1: 1, f2: 2
        }
        .cmp(&Struct {
            f1: 1, f2: 3
        })
    );

    assert_eq!(
        Ordering::Equal,
        Struct {
            f1: 1, f2: 2
        }
        .cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(Ordering::Less, Tuple(1, 3).cmp(&Tuple(1, 2)));
    assert_eq!(Ordering::Greater, Tuple(1, 2).cmp(&Tuple(1, 3)));
    assert_eq!(Ordering::Equal, Tuple(1, 2).cmp(&Tuple(1, 2)));
}

#[test]
fn compare_with_trait_1() {
    trait A {
        fn cmp(&self, b: &Self) -> Ordering;
    }

    impl A for u8 {
        fn cmp(&self, b: &u8) -> Ordering {
            #[allow(clippy::comparison_chain)]
            if self > b {
                Ordering::Less
            } else if self < b {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Struct {
        f1: u8,
        #[educe(Ord(trait = "A"))]
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Tuple(u8, #[educe(Ord(trait = "A"))] u8);

    assert_eq!(
        Ordering::Less,
        Struct {
            f1: 1, f2: 3
        }
        .cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(
        Ordering::Greater,
        Struct {
            f1: 1, f2: 2
        }
        .cmp(&Struct {
            f1: 1, f2: 3
        })
    );

    assert_eq!(
        Ordering::Equal,
        Struct {
            f1: 1, f2: 2
        }
        .cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(Ordering::Less, Tuple(1, 3).cmp(&Tuple(1, 2)));
    assert_eq!(Ordering::Greater, Tuple(1, 2).cmp(&Tuple(1, 3)));
    assert_eq!(Ordering::Equal, Tuple(1, 2).cmp(&Tuple(1, 2)));
}

#[test]
fn compare_with_trait_2() {
    trait A {
        fn cmp(&self, b: &Self) -> Ordering;
    }

    impl A for u8 {
        fn cmp(&self, b: &u8) -> Ordering {
            #[allow(clippy::comparison_chain)]
            if self > b {
                Ordering::Less
            } else if self < b {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Struct {
        f1: u8,
        #[educe(Ord(trait("A")))]
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Tuple(u8, #[educe(Ord(trait("A")))] u8);

    assert_eq!(
        Ordering::Less,
        Struct {
            f1: 1, f2: 3
        }
        .cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(
        Ordering::Greater,
        Struct {
            f1: 1, f2: 2
        }
        .cmp(&Struct {
            f1: 1, f2: 3
        })
    );

    assert_eq!(
        Ordering::Equal,
        Struct {
            f1: 1, f2: 2
        }
        .cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(Ordering::Less, Tuple(1, 3).cmp(&Tuple(1, 2)));
    assert_eq!(Ordering::Greater, Tuple(1, 2).cmp(&Tuple(1, 3)));
    assert_eq!(Ordering::Equal, Tuple(1, 2).cmp(&Tuple(1, 2)));
}

#[test]
fn compare_with_trait_3() {
    trait A {
        fn compare(&self, b: &Self) -> Ordering;
    }

    impl A for u8 {
        fn compare(&self, b: &u8) -> Ordering {
            b.cmp(self)
        }
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Struct {
        f1: u8,
        #[educe(Ord(trait = "A", method = "compare"))]
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Tuple(u8, #[educe(Ord(trait = "A", method = "compare"))] u8);

    assert_eq!(
        Ordering::Less,
        Struct {
            f1: 1, f2: 3
        }
        .cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(
        Ordering::Greater,
        Struct {
            f1: 1, f2: 2
        }
        .cmp(&Struct {
            f1: 1, f2: 3
        })
    );

    assert_eq!(
        Ordering::Equal,
        Struct {
            f1: 1, f2: 2
        }
        .cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(Ordering::Less, Tuple(1, 3).cmp(&Tuple(1, 2)));
    assert_eq!(Ordering::Greater, Tuple(1, 2).cmp(&Tuple(1, 3)));
    assert_eq!(Ordering::Equal, Tuple(1, 2).cmp(&Tuple(1, 2)));
}

#[test]
fn compare_with_trait_4() {
    trait A {
        fn compare(&self, b: &Self) -> Ordering;
    }

    impl A for u8 {
        fn compare(&self, b: &u8) -> Ordering {
            b.cmp(self)
        }
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Struct {
        f1: u8,
        #[educe(Ord(trait("A"), method("compare")))]
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Tuple(u8, #[educe(Ord(trait("A"), method("compare")))] u8);

    assert_eq!(
        Ordering::Less,
        Struct {
            f1: 1, f2: 3
        }
        .cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(
        Ordering::Greater,
        Struct {
            f1: 1, f2: 2
        }
        .cmp(&Struct {
            f1: 1, f2: 3
        })
    );

    assert_eq!(
        Ordering::Equal,
        Struct {
            f1: 1, f2: 2
        }
        .cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(Ordering::Less, Tuple(1, 3).cmp(&Tuple(1, 2)));
    assert_eq!(Ordering::Greater, Tuple(1, 2).cmp(&Tuple(1, 3)));
    assert_eq!(Ordering::Equal, Tuple(1, 2).cmp(&Tuple(1, 2)));
}

#[test]
fn bound_1() {
    #[derive(Educe)]
    #[educe(PartialEq(bound), Eq(bound), PartialOrd(bound), Ord(bound))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(PartialEq(bound), Eq(bound), PartialOrd(bound), Ord(bound))]
    struct Tuple<T>(T);

    assert_eq!(
        Ordering::Greater,
        Struct {
            f1: 2
        }
        .cmp(&Struct {
            f1: 1
        })
    );

    assert_eq!(
        Ordering::Less,
        Struct {
            f1: 1
        }
        .cmp(&Struct {
            f1: 2
        })
    );

    assert_eq!(Ordering::Greater, Tuple(2).cmp(&Tuple(1)));
    assert_eq!(Ordering::Less, Tuple(1).cmp(&Tuple(2)));
}

#[test]
fn bound_2() {
    #[derive(Educe)]
    #[educe(PartialEq(bound), Eq(bound), PartialOrd(bound), Ord(bound = "T: core::cmp::Ord"))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(PartialEq(bound), Eq(bound), PartialOrd(bound), Ord(bound = "T: core::cmp::Ord"))]
    struct Tuple<T>(T);

    assert_eq!(
        Ordering::Greater,
        Struct {
            f1: 2
        }
        .cmp(&Struct {
            f1: 1
        })
    );

    assert_eq!(
        Ordering::Less,
        Struct {
            f1: 1
        }
        .cmp(&Struct {
            f1: 2
        })
    );

    assert_eq!(Ordering::Greater, Tuple(2).cmp(&Tuple(1)));
    assert_eq!(Ordering::Less, Tuple(1).cmp(&Tuple(2)));
}

#[test]
fn bound_3() {
    #[derive(Educe)]
    #[educe(PartialEq(bound), Eq(bound), PartialOrd(bound), Ord(bound("T: core::cmp::Ord")))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(PartialEq(bound), Eq(bound), PartialOrd(bound), Ord(bound("T: core::cmp::Ord")))]
    struct Tuple<T>(T);

    assert_eq!(
        Ordering::Greater,
        Struct {
            f1: 2
        }
        .cmp(&Struct {
            f1: 1
        })
    );

    assert_eq!(
        Ordering::Less,
        Struct {
            f1: 1
        }
        .cmp(&Struct {
            f1: 2
        })
    );

    assert_eq!(Ordering::Greater, Tuple(2).cmp(&Tuple(1)));
    assert_eq!(Ordering::Less, Tuple(1).cmp(&Tuple(2)));
}

#[test]
fn rank_1() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Struct {
        #[educe(Ord(rank = 1))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Tuple(#[educe(Ord(rank = 1))] u8, u8);

    assert_eq!(
        Ordering::Less,
        Struct {
            f1: 2, f2: 1
        }
        .cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(
        Ordering::Greater,
        Struct {
            f1: 1, f2: 2
        }
        .cmp(&Struct {
            f1: 2, f2: 1
        })
    );

    assert_eq!(Ordering::Less, Tuple(2, 1).cmp(&Tuple(1, 2)));
    assert_eq!(Ordering::Greater, Tuple(1, 2).cmp(&Tuple(2, 1)));
}

#[test]
fn rank_2() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Struct {
        #[educe(Ord(rank(1)))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Tuple(#[educe(Ord(rank(1)))] u8, u8);

    assert_eq!(
        Ordering::Less,
        Struct {
            f1: 2, f2: 1
        }
        .cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(
        Ordering::Greater,
        Struct {
            f1: 1, f2: 2
        }
        .cmp(&Struct {
            f1: 2, f2: 1
        })
    );

    assert_eq!(Ordering::Less, Tuple(2, 1).cmp(&Tuple(1, 2)));
    assert_eq!(Ordering::Greater, Tuple(1, 2).cmp(&Tuple(2, 1)));
}

#[test]
fn rank_3() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Struct {
        #[educe(Ord(rank = 1))]
        f1: u8,
        #[educe(Ord(rank = 0))]
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Tuple(#[educe(Ord(rank = 1))] u8, #[educe(Ord(rank = 0))] u8);

    assert_eq!(
        Ordering::Less,
        Struct {
            f1: 2, f2: 1
        }
        .cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(
        Ordering::Greater,
        Struct {
            f1: 1, f2: 2
        }
        .cmp(&Struct {
            f1: 2, f2: 1
        })
    );

    assert_eq!(Ordering::Less, Tuple(2, 1).cmp(&Tuple(1, 2)));
    assert_eq!(Ordering::Greater, Tuple(1, 2).cmp(&Tuple(2, 1)));
}

#[test]
fn rank_4() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Struct {
        #[educe(Ord(rank(1)))]
        f1: u8,
        #[educe(Ord(rank(0)))]
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    struct Tuple(#[educe(Ord(rank = 1))] u8, #[educe(Ord(rank = 0))] u8);

    assert_eq!(
        Ordering::Less,
        Struct {
            f1: 2, f2: 1
        }
        .cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(
        Ordering::Greater,
        Struct {
            f1: 1, f2: 2
        }
        .cmp(&Struct {
            f1: 2, f2: 1
        })
    );

    assert_eq!(Ordering::Less, Tuple(2, 1).cmp(&Tuple(1, 2)));
    assert_eq!(Ordering::Greater, Tuple(1, 2).cmp(&Tuple(2, 1)));
}
