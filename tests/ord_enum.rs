#![cfg(all(feature = "PartialEq", feature = "Eq", feature = "PartialOrd", feature = "Ord"))]

#![no_std]

#[macro_use]
extern crate educe;

use core::cmp::Ordering;

#[test]
fn basic_1() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    enum Enum {
        Unit,
        Unit2,
        Struct {
            f1: u8
        },
        Tuple(u8),
    }

    assert!(Enum::Unit == Enum::Unit);
    assert_eq!(Ordering::Less, Enum::Unit.cmp(&Enum::Unit2));

    assert_eq!(Ordering::Greater, Enum::Struct {
        f1: 2
    }.cmp(&Enum::Struct {
        f1: 1
    }));

    assert_eq!(Ordering::Less, Enum::Struct {
        f1: 1
    }.cmp(&Enum::Struct {
        f1: 2
    }));

    assert_eq!(Ordering::Greater, Enum::Tuple(2).cmp(&Enum::Tuple(1)));
    assert_eq!(Ordering::Less, Enum::Tuple(1).cmp(&Enum::Tuple(2)));
}

#[test]
fn basic_2() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    enum Enum {
        Struct {
            f1: u8,
            f2: u8,
        },
        Tuple(u8, u8),
    }

    assert_eq!(Ordering::Greater, Enum::Struct {
        f1: 2,
        f2: 1,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 2,
    }));

    assert_eq!(Ordering::Less, Enum::Struct {
        f1: 1,
        f2: 2,
    }.cmp(&Enum::Struct {
        f1: 2,
        f2: 1,
    }));

    assert_eq!(Ordering::Greater, Enum::Tuple(2, 1).cmp(&Enum::Tuple(1, 2)));
    assert_eq!(Ordering::Less, Enum::Tuple(1, 2).cmp(&Enum::Tuple(2, 1)));
}

#[test]
fn basic_3() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    enum Enum {
        A = 2,
        B = 1,
    }

    assert_eq!(Ordering::Greater, Enum::A.cmp(&Enum::B));
}

#[test]
fn ignore() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    enum Enum {
        Struct {
            #[educe(Ord(ignore))]
            f1: u8,
            f2: u8,
        },
        Tuple(
            #[educe(Ord(ignore))]
            u8,
            u8,
        ),
    }

    assert_eq!(Ordering::Greater, Enum::Struct {
        f1: 1,
        f2: 3,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 2,
    }));

    assert_eq!(Ordering::Less, Enum::Struct {
        f1: 1,
        f2: 2,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 3,
    }));

    assert_eq!(Ordering::Equal, Enum::Struct {
        f1: 2,
        f2: 2,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 2,
    }));

    assert_eq!(Ordering::Greater, Enum::Tuple(1, 3).cmp(&Enum::Tuple(1, 2)));
    assert_eq!(Ordering::Less, Enum::Tuple(1, 2).cmp(&Enum::Tuple(1, 3)));
    assert_eq!(Ordering::Equal, Enum::Tuple(2, 2).cmp(&Enum::Tuple(1, 2)));
}

#[test]
fn compare_without_trait_1() {
    fn cmp(a: &u8, b: &u8) -> Ordering {
        if a > b {
            Ordering::Less
        } else if a < b {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    enum Enum {
        Struct {
            f1: u8,
            #[educe(Ord(compare = "cmp"))]
            f2: u8,
        },
        Tuple(
            u8,
            #[educe(Ord(compare = "cmp"))]
            u8,
        ),
    }

    assert_eq!(Ordering::Less, Enum::Struct {
        f1: 1,
        f2: 3,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 2,
    }));

    assert_eq!(Ordering::Greater, Enum::Struct {
        f1: 1,
        f2: 2,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 3,
    }));

    assert_eq!(Ordering::Equal, Enum::Struct {
        f1: 1,
        f2: 2,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 2,
    }));

    assert_eq!(Ordering::Less, Enum::Tuple(1, 3).cmp(&Enum::Tuple(1, 2)));
    assert_eq!(Ordering::Greater, Enum::Tuple(1, 2).cmp(&Enum::Tuple(1, 3)));
    assert_eq!(Ordering::Equal, Enum::Tuple(1, 2).cmp(&Enum::Tuple(1, 2)));
}

#[test]
fn compare_without_trait_2() {
    fn cmp(a: &u8, b: &u8) -> Ordering {
        if a > b {
            Ordering::Less
        } else if a < b {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    enum Enum {
        Struct {
            f1: u8,
            #[educe(Ord(compare("cmp")))]
            f2: u8,
        },
        Tuple(
            u8,
            #[educe(Ord(compare("cmp")))]
            u8,
        ),
    }

    assert_eq!(Ordering::Less, Enum::Struct {
        f1: 1,
        f2: 3,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 2,
    }));

    assert_eq!(Ordering::Greater, Enum::Struct {
        f1: 1,
        f2: 2,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 3,
    }));

    assert_eq!(Ordering::Equal, Enum::Struct {
        f1: 1,
        f2: 2,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 2,
    }));

    assert_eq!(Ordering::Less, Enum::Tuple(1, 3).cmp(&Enum::Tuple(1, 2)));
    assert_eq!(Ordering::Greater, Enum::Tuple(1, 2).cmp(&Enum::Tuple(1, 3)));
    assert_eq!(Ordering::Equal, Enum::Tuple(1, 2).cmp(&Enum::Tuple(1, 2)));
}

#[test]
fn compare_without_trait_3() {
    fn cmp(a: &u8, b: &u8) -> Ordering {
        if a > b {
            Ordering::Less
        } else if a < b {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    enum Enum {
        Struct {
            f1: u8,
            #[educe(Ord(compare(method = "cmp")))]
            f2: u8,
        },
        Tuple(
            u8,
            #[educe(Ord(compare(method = "cmp")))]
            u8,
        ),
    }

    assert_eq!(Ordering::Less, Enum::Struct {
        f1: 1,
        f2: 3,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 2,
    }));

    assert_eq!(Ordering::Greater, Enum::Struct {
        f1: 1,
        f2: 2,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 3,
    }));

    assert_eq!(Ordering::Equal, Enum::Struct {
        f1: 1,
        f2: 2,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 2,
    }));

    assert_eq!(Ordering::Less, Enum::Tuple(1, 3).cmp(&Enum::Tuple(1, 2)));
    assert_eq!(Ordering::Greater, Enum::Tuple(1, 2).cmp(&Enum::Tuple(1, 3)));
    assert_eq!(Ordering::Equal, Enum::Tuple(1, 2).cmp(&Enum::Tuple(1, 2)));
}

#[test]
fn compare_without_trait_4() {
    fn cmp(a: &u8, b: &u8) -> Ordering {
        if a > b {
            Ordering::Less
        } else if a < b {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    enum Enum {
        Struct {
            f1: u8,
            #[educe(Ord(compare(method("cmp"))))]
            f2: u8,
        },
        Tuple(
            u8,
            #[educe(Ord(compare(method("cmp"))))]
            u8,
        ),
    }

    assert_eq!(Ordering::Less, Enum::Struct {
        f1: 1,
        f2: 3,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 2,
    }));

    assert_eq!(Ordering::Greater, Enum::Struct {
        f1: 1,
        f2: 2,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 3,
    }));

    assert_eq!(Ordering::Equal, Enum::Struct {
        f1: 1,
        f2: 2,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 2,
    }));

    assert_eq!(Ordering::Less, Enum::Tuple(1, 3).cmp(&Enum::Tuple(1, 2)));
    assert_eq!(Ordering::Greater, Enum::Tuple(1, 2).cmp(&Enum::Tuple(1, 3)));
    assert_eq!(Ordering::Equal, Enum::Tuple(1, 2).cmp(&Enum::Tuple(1, 2)));
}

#[test]
fn compare_with_trait_1() {
    trait A {
        fn cmp(&self, b: &Self) -> Ordering;
    }

    impl A for u8 {
        fn cmp(&self, b: &u8) -> Ordering {
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
    enum Enum {
        Struct {
            f1: u8,
            #[educe(Ord(compare(trait = "A")))]
            f2: u8,
        },
        Tuple(
            u8,
            #[educe(Ord(compare(trait = "A")))]
            u8,
        ),
    }

    assert_eq!(Ordering::Less, Enum::Struct {
        f1: 1,
        f2: 3,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 2,
    }));

    assert_eq!(Ordering::Greater, Enum::Struct {
        f1: 1,
        f2: 2,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 3,
    }));

    assert_eq!(Ordering::Equal, Enum::Struct {
        f1: 1,
        f2: 2,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 2,
    }));

    assert_eq!(Ordering::Less, Enum::Tuple(1, 3).cmp(&Enum::Tuple(1, 2)));
    assert_eq!(Ordering::Greater, Enum::Tuple(1, 2).cmp(&Enum::Tuple(1, 3)));
    assert_eq!(Ordering::Equal, Enum::Tuple(1, 2).cmp(&Enum::Tuple(1, 2)));
}

#[test]
fn compare_with_trait_2() {
    trait A {
        fn cmp(&self, b: &Self) -> Ordering;
    }

    impl A for u8 {
        fn cmp(&self, b: &u8) -> Ordering {
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
    enum Enum {
        Struct {
            f1: u8,
            #[educe(Ord(compare(trait ("A"))))]
            f2: u8,
        },
        Tuple(
            u8,
            #[educe(Ord(compare(trait ("A"))))]
            u8,
        ),
    }

    assert_eq!(Ordering::Less, Enum::Struct {
        f1: 1,
        f2: 3,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 2,
    }));

    assert_eq!(Ordering::Greater, Enum::Struct {
        f1: 1,
        f2: 2,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 3,
    }));

    assert_eq!(Ordering::Equal, Enum::Struct {
        f1: 1,
        f2: 2,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 2,
    }));

    assert_eq!(Ordering::Less, Enum::Tuple(1, 3).cmp(&Enum::Tuple(1, 2)));
    assert_eq!(Ordering::Greater, Enum::Tuple(1, 2).cmp(&Enum::Tuple(1, 3)));
    assert_eq!(Ordering::Equal, Enum::Tuple(1, 2).cmp(&Enum::Tuple(1, 2)));
}

#[test]
fn compare_with_trait_3() {
    trait A {
        fn compare(&self, b: &Self) -> Ordering;
    }

    impl A for u8 {
        fn compare(&self, b: &u8) -> Ordering {
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
    enum Enum {
        Struct {
            f1: u8,
            #[educe(Ord(compare(trait = "A", method = "compare")))]
            f2: u8,
        },
        Tuple(
            u8,
            #[educe(Ord(compare(trait = "A", method = "compare")))]
            u8,
        ),
    }

    assert_eq!(Ordering::Less, Enum::Struct {
        f1: 1,
        f2: 3,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 2,
    }));

    assert_eq!(Ordering::Greater, Enum::Struct {
        f1: 1,
        f2: 2,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 3,
    }));

    assert_eq!(Ordering::Equal, Enum::Struct {
        f1: 1,
        f2: 2,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 2,
    }));

    assert_eq!(Ordering::Less, Enum::Tuple(1, 3).cmp(&Enum::Tuple(1, 2)));
    assert_eq!(Ordering::Greater, Enum::Tuple(1, 2).cmp(&Enum::Tuple(1, 3)));
    assert_eq!(Ordering::Equal, Enum::Tuple(1, 2).cmp(&Enum::Tuple(1, 2)));
}

#[test]
fn compare_with_trait_4() {
    trait A {
        fn compare(&self, b: &Self) -> Ordering;
    }

    impl A for u8 {
        fn compare(&self, b: &u8) -> Ordering {
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
    enum Enum {
        Struct {
            f1: u8,
            #[educe(Ord(compare(trait ("A"), method("compare"))))]
            f2: u8,
        },
        Tuple(
            u8,
            #[educe(Ord(compare(trait ("A"), method("compare"))))]
            u8,
        ),
    }

    assert_eq!(Ordering::Less, Enum::Struct {
        f1: 1,
        f2: 3,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 2,
    }));

    assert_eq!(Ordering::Greater, Enum::Struct {
        f1: 1,
        f2: 2,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 3,
    }));

    assert_eq!(Ordering::Equal, Enum::Struct {
        f1: 1,
        f2: 2,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 2,
    }));

    assert_eq!(Ordering::Less, Enum::Tuple(1, 3).cmp(&Enum::Tuple(1, 2)));
    assert_eq!(Ordering::Greater, Enum::Tuple(1, 2).cmp(&Enum::Tuple(1, 3)));
    assert_eq!(Ordering::Equal, Enum::Tuple(1, 2).cmp(&Enum::Tuple(1, 2)));
}

#[test]
fn bound_1() {
    #[derive(Educe)]
    #[educe(PartialEq(bound), Eq(bound), PartialOrd(bound), Ord(bound))]
    enum Enum<T> {
        Struct {
            f1: T
        },
        Tuple(T),
    }

    assert_eq!(Ordering::Greater, Enum::Struct {
        f1: 2
    }.cmp(&Enum::Struct {
        f1: 1
    }));

    assert_eq!(Ordering::Less, Enum::Struct {
        f1: 1
    }.cmp(&Enum::Struct {
        f1: 2
    }));

    assert_eq!(Ordering::Greater, Enum::Tuple(2).cmp(&Enum::Tuple(1)));
    assert_eq!(Ordering::Less, Enum::Tuple(1).cmp(&Enum::Tuple(2)));
}

#[test]
fn bound_2() {
    #[derive(Educe)]
    #[educe(PartialEq(bound), Eq(bound), PartialOrd(bound), Ord(bound = "T: core::cmp::Ord"))]
    enum Enum<T> {
        Struct {
            f1: T
        },
        Tuple(T),
    }

    assert_eq!(Ordering::Greater, Enum::Struct {
        f1: 2
    }.cmp(&Enum::Struct {
        f1: 1
    }));

    assert_eq!(Ordering::Less, Enum::Struct {
        f1: 1
    }.cmp(&Enum::Struct {
        f1: 2
    }));

    assert_eq!(Ordering::Greater, Enum::Tuple(2).cmp(&Enum::Tuple(1)));
    assert_eq!(Ordering::Less, Enum::Tuple(1).cmp(&Enum::Tuple(2)));
}

#[test]
fn bound_3() {
    #[derive(Educe)]
    #[educe(PartialEq(bound), Eq(bound), PartialOrd(bound), Ord(bound("T: core::cmp::Ord")))]
    enum Enum<T> {
        Struct {
            f1: T
        },
        Tuple(T),
    }

    assert_eq!(Ordering::Greater, Enum::Struct {
        f1: 2
    }.cmp(&Enum::Struct {
        f1: 1
    }));

    assert_eq!(Ordering::Less, Enum::Struct {
        f1: 1
    }.cmp(&Enum::Struct {
        f1: 2
    }));

    assert_eq!(Ordering::Greater, Enum::Tuple(2).cmp(&Enum::Tuple(1)));
    assert_eq!(Ordering::Less, Enum::Tuple(1).cmp(&Enum::Tuple(2)));
}

#[test]
fn rank_1() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    enum Enum {
        Struct {
            #[educe(Ord(rank = 1))]
            f1: u8,
            f2: u8,
        },
        Tuple(
            #[educe(Ord(rank = 1))]
            u8,
            u8,
        ),
    }

    assert_eq!(Ordering::Less, Enum::Struct {
        f1: 2,
        f2: 1,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 2,
    }));

    assert_eq!(Ordering::Greater, Enum::Struct {
        f1: 1,
        f2: 2,
    }.cmp(&Enum::Struct {
        f1: 2,
        f2: 1,
    }));

    assert_eq!(Ordering::Less, Enum::Tuple(2, 1).cmp(&Enum::Tuple(1, 2)));
    assert_eq!(Ordering::Greater, Enum::Tuple(1, 2).cmp(&Enum::Tuple(2, 1)));
}

#[test]
fn rank_2() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    enum Enum {
        Struct {
            #[educe(Ord(rank(1)))]
            f1: u8,
            f2: u8,
        },
        Tuple(
            #[educe(Ord(rank(1)))]
            u8,
            u8,
        ),
    }

    assert_eq!(Ordering::Less, Enum::Struct {
        f1: 2,
        f2: 1,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 2,
    }));

    assert_eq!(Ordering::Greater, Enum::Struct {
        f1: 1,
        f2: 2,
    }.cmp(&Enum::Struct {
        f1: 2,
        f2: 1,
    }));

    assert_eq!(Ordering::Less, Enum::Tuple(2, 1).cmp(&Enum::Tuple(1, 2)));
    assert_eq!(Ordering::Greater, Enum::Tuple(1, 2).cmp(&Enum::Tuple(2, 1)));
}


#[test]
fn rank_3() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    enum Enum {
        Struct {
            #[educe(Ord(rank = 1))]
            f1: u8,
            #[educe(Ord(rank = 0))]
            f2: u8,
        },
        Tuple(
            #[educe(Ord(rank = 1))]
            u8,
            #[educe(Ord(rank = 0))]
            u8,
        ),
    }

    assert_eq!(Ordering::Less, Enum::Struct {
        f1: 2,
        f2: 1,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 2,
    }));

    assert_eq!(Ordering::Greater, Enum::Struct {
        f1: 1,
        f2: 2,
    }.cmp(&Enum::Struct {
        f1: 2,
        f2: 1,
    }));

    assert_eq!(Ordering::Less, Enum::Tuple(2, 1).cmp(&Enum::Tuple(1, 2)));
    assert_eq!(Ordering::Greater, Enum::Tuple(1, 2).cmp(&Enum::Tuple(2, 1)));
}

#[test]
fn rank_4() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    enum Enum {
        Struct {
            #[educe(Ord(rank(1)))]
            f1: u8,
            #[educe(Ord(rank(0)))]
            f2: u8,
        },
        Tuple(
            #[educe(Ord(rank(1)))]
            u8,
            #[educe(Ord(rank(0)))]
            u8,
        ),
    }

    assert_eq!(Ordering::Less, Enum::Struct {
        f1: 2,
        f2: 1,
    }.cmp(&Enum::Struct {
        f1: 1,
        f2: 2,
    }));

    assert_eq!(Ordering::Greater, Enum::Struct {
        f1: 1,
        f2: 2,
    }.cmp(&Enum::Struct {
        f1: 2,
        f2: 1,
    }));

    assert_eq!(Ordering::Less, Enum::Tuple(2, 1).cmp(&Enum::Tuple(1, 2)));
    assert_eq!(Ordering::Greater, Enum::Tuple(1, 2).cmp(&Enum::Tuple(2, 1)));
}

#[test]
fn value_1() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    enum Enum {
        #[educe(Ord(value = 2))]
        Two,
        #[educe(Ord(value = 1))]
        One,
    }

    assert_eq!(Ordering::Greater, Enum::Two.cmp(&Enum::One));
}

#[test]
fn value_2() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq, PartialOrd, Ord)]
    enum Enum {
        #[educe(Ord(value(2)))]
        Two,
        #[educe(Ord(value(1)))]
        One,
    }

    assert_eq!(Ordering::Greater, Enum::Two.cmp(&Enum::One));
}