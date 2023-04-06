#![allow(clippy::eq_op, clippy::trivially_copy_pass_by_ref)]
#![cfg(all(feature = "PartialEq", feature = "PartialOrd"))]
#![no_std]

#[macro_use]
extern crate educe;

use core::cmp::Ordering;

#[test]
fn basic_1() {
    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Unit;

    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Struct {
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Tuple(u8);

    assert!(Unit == Unit);

    assert!(
        Struct {
            f1: 2
        } > Struct {
            f1: 1
        }
    );

    assert!(
        Struct {
            f1: 1
        } < Struct {
            f1: 2
        }
    );

    assert!(Tuple(2) > Tuple(1));
    assert!(Tuple(1) < Tuple(2));
}

#[test]
fn basic_2() {
    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Struct {
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Tuple(u8, u8);

    assert!(
        Struct {
            f1: 2, f2: 1
        } > Struct {
            f1: 1, f2: 2
        }
    );

    assert!(
        Struct {
            f1: 1, f2: 2
        } < Struct {
            f1: 2, f2: 1
        }
    );

    assert!(Tuple(2, 1) > Tuple(1, 2));
    assert!(Tuple(1, 2) < Tuple(2, 1));
}

#[test]
#[allow(dead_code)]
fn ignore() {
    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Struct {
        #[educe(PartialOrd(ignore))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Tuple(#[educe(PartialOrd(ignore))] u8, u8);

    assert_eq!(
        Some(Ordering::Greater),
        Struct {
            f1: 1, f2: 3
        }
        .partial_cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(
        Some(Ordering::Less),
        Struct {
            f1: 1, f2: 2
        }
        .partial_cmp(&Struct {
            f1: 1, f2: 3
        })
    );

    assert_eq!(
        Some(Ordering::Equal),
        Struct {
            f1: 2, f2: 2
        }
        .partial_cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(Some(Ordering::Greater), Tuple(1, 3).partial_cmp(&Tuple(1, 2)));
    assert_eq!(Some(Ordering::Less), Tuple(1, 2).partial_cmp(&Tuple(1, 3)));
    assert_eq!(Some(Ordering::Equal), Tuple(2, 2).partial_cmp(&Tuple(1, 2)));
}

#[test]
fn compare_without_trait_1() {
    fn partial_cmp(a: &u8, b: &u8) -> Option<Ordering> {
        b.partial_cmp(a)
    }

    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Struct {
        f1: u8,
        #[educe(PartialOrd(method = "partial_cmp"))]
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Tuple(u8, #[educe(PartialOrd(method = "partial_cmp"))] u8);

    assert_eq!(
        Some(Ordering::Less),
        Struct {
            f1: 1, f2: 3
        }
        .partial_cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(
        Some(Ordering::Greater),
        Struct {
            f1: 1, f2: 2
        }
        .partial_cmp(&Struct {
            f1: 1, f2: 3
        })
    );

    assert_eq!(
        Some(Ordering::Equal),
        Struct {
            f1: 1, f2: 2
        }
        .partial_cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(Some(Ordering::Less), Tuple(1, 3).partial_cmp(&Tuple(1, 2)));
    assert_eq!(Some(Ordering::Greater), Tuple(1, 2).partial_cmp(&Tuple(1, 3)));
    assert_eq!(Some(Ordering::Equal), Tuple(1, 2).partial_cmp(&Tuple(1, 2)));
}

#[test]
fn compare_without_trait_2() {
    fn partial_cmp(a: &u8, b: &u8) -> Option<Ordering> {
        b.partial_cmp(a)
    }

    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Struct {
        f1: u8,
        #[educe(PartialOrd(method("partial_cmp")))]
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Tuple(u8, #[educe(PartialOrd(method("partial_cmp")))] u8);

    assert_eq!(
        Some(Ordering::Less),
        Struct {
            f1: 1, f2: 3
        }
        .partial_cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(
        Some(Ordering::Greater),
        Struct {
            f1: 1, f2: 2
        }
        .partial_cmp(&Struct {
            f1: 1, f2: 3
        })
    );

    assert_eq!(
        Some(Ordering::Equal),
        Struct {
            f1: 1, f2: 2
        }
        .partial_cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(Some(Ordering::Less), Tuple(1, 3).partial_cmp(&Tuple(1, 2)));
    assert_eq!(Some(Ordering::Greater), Tuple(1, 2).partial_cmp(&Tuple(1, 3)));
    assert_eq!(Some(Ordering::Equal), Tuple(1, 2).partial_cmp(&Tuple(1, 2)));
}

#[test]
fn compare_with_trait_1() {
    trait A {
        fn partial_cmp(&self, b: &Self) -> Option<Ordering>;
    }

    impl A for u8 {
        fn partial_cmp(&self, b: &u8) -> Option<Ordering> {
            #[allow(clippy::comparison_chain)]
            if self > b {
                Some(Ordering::Less)
            } else if self < b {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Equal)
            }
        }
    }

    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Struct {
        f1: u8,
        #[educe(PartialOrd(trait = "A"))]
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Tuple(u8, #[educe(PartialOrd(trait = "A"))] u8);

    assert_eq!(
        Some(Ordering::Less),
        Struct {
            f1: 1, f2: 3
        }
        .partial_cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(
        Some(Ordering::Greater),
        Struct {
            f1: 1, f2: 2
        }
        .partial_cmp(&Struct {
            f1: 1, f2: 3
        })
    );

    assert_eq!(
        Some(Ordering::Equal),
        Struct {
            f1: 1, f2: 2
        }
        .partial_cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(Some(Ordering::Less), Tuple(1, 3).partial_cmp(&Tuple(1, 2)));
    assert_eq!(Some(Ordering::Greater), Tuple(1, 2).partial_cmp(&Tuple(1, 3)));
    assert_eq!(Some(Ordering::Equal), Tuple(1, 2).partial_cmp(&Tuple(1, 2)));
}

#[test]
fn compare_with_trait_2() {
    trait A {
        fn partial_cmp(&self, b: &Self) -> Option<Ordering>;
    }

    impl A for u8 {
        fn partial_cmp(&self, b: &u8) -> Option<Ordering> {
            #[allow(clippy::comparison_chain)]
            if self > b {
                Some(Ordering::Less)
            } else if self < b {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Equal)
            }
        }
    }

    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Struct {
        f1: u8,
        #[educe(PartialOrd(trait("A")))]
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Tuple(u8, #[educe(PartialOrd(trait("A")))] u8);

    assert_eq!(
        Some(Ordering::Less),
        Struct {
            f1: 1, f2: 3
        }
        .partial_cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(
        Some(Ordering::Greater),
        Struct {
            f1: 1, f2: 2
        }
        .partial_cmp(&Struct {
            f1: 1, f2: 3
        })
    );

    assert_eq!(
        Some(Ordering::Equal),
        Struct {
            f1: 1, f2: 2
        }
        .partial_cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(Some(Ordering::Less), Tuple(1, 3).partial_cmp(&Tuple(1, 2)));
    assert_eq!(Some(Ordering::Greater), Tuple(1, 2).partial_cmp(&Tuple(1, 3)));
    assert_eq!(Some(Ordering::Equal), Tuple(1, 2).partial_cmp(&Tuple(1, 2)));
}

#[test]
fn compare_with_trait_3() {
    trait A {
        fn compare(&self, b: &Self) -> Option<Ordering>;
    }

    impl A for u8 {
        fn compare(&self, b: &u8) -> Option<Ordering> {
            b.partial_cmp(self)
        }
    }

    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Struct {
        f1: u8,
        #[educe(PartialOrd(trait = "A", method = "compare"))]
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Tuple(u8, #[educe(PartialOrd(trait = "A", method = "compare"))] u8);

    assert_eq!(
        Some(Ordering::Less),
        Struct {
            f1: 1, f2: 3
        }
        .partial_cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(
        Some(Ordering::Greater),
        Struct {
            f1: 1, f2: 2
        }
        .partial_cmp(&Struct {
            f1: 1, f2: 3
        })
    );

    assert_eq!(
        Some(Ordering::Equal),
        Struct {
            f1: 1, f2: 2
        }
        .partial_cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(Some(Ordering::Less), Tuple(1, 3).partial_cmp(&Tuple(1, 2)));
    assert_eq!(Some(Ordering::Greater), Tuple(1, 2).partial_cmp(&Tuple(1, 3)));
    assert_eq!(Some(Ordering::Equal), Tuple(1, 2).partial_cmp(&Tuple(1, 2)));
}

#[test]
fn compare_with_trait_4() {
    trait A {
        fn compare(&self, b: &Self) -> Option<Ordering>;
    }

    impl A for u8 {
        fn compare(&self, b: &u8) -> Option<Ordering> {
            b.partial_cmp(self)
        }
    }

    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Struct {
        f1: u8,
        #[educe(PartialOrd(trait("A"), method("compare")))]
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Tuple(u8, #[educe(PartialOrd(trait("A"), method("compare")))] u8);

    assert_eq!(
        Some(Ordering::Less),
        Struct {
            f1: 1, f2: 3
        }
        .partial_cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(
        Some(Ordering::Greater),
        Struct {
            f1: 1, f2: 2
        }
        .partial_cmp(&Struct {
            f1: 1, f2: 3
        })
    );

    assert_eq!(
        Some(Ordering::Equal),
        Struct {
            f1: 1, f2: 2
        }
        .partial_cmp(&Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(Some(Ordering::Less), Tuple(1, 3).partial_cmp(&Tuple(1, 2)));
    assert_eq!(Some(Ordering::Greater), Tuple(1, 2).partial_cmp(&Tuple(1, 3)));
    assert_eq!(Some(Ordering::Equal), Tuple(1, 2).partial_cmp(&Tuple(1, 2)));
}

#[test]
fn bound_1() {
    #[derive(Educe)]
    #[educe(PartialEq(bound), PartialOrd(bound))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(PartialEq(bound), PartialOrd(bound))]
    struct Tuple<T>(T);

    assert!(
        Struct {
            f1: 2
        } > Struct {
            f1: 1
        }
    );

    assert!(
        Struct {
            f1: 1
        } < Struct {
            f1: 2
        }
    );

    assert!(Tuple(2) > Tuple(1));
    assert!(Tuple(1) < Tuple(2));
}

#[test]
fn bound_2() {
    #[derive(Educe)]
    #[educe(PartialEq(bound), PartialOrd(bound = "T: core::cmp::PartialOrd"))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(PartialEq(bound), PartialOrd(bound = "T: core::cmp::PartialOrd"))]
    struct Tuple<T>(T);

    assert!(
        Struct {
            f1: 2
        } > Struct {
            f1: 1
        }
    );

    assert!(
        Struct {
            f1: 1
        } < Struct {
            f1: 2
        }
    );

    assert!(Tuple(2) > Tuple(1));
    assert!(Tuple(1) < Tuple(2));
}

#[test]
fn bound_3() {
    #[derive(Educe)]
    #[educe(PartialEq(bound), PartialOrd(bound("T: core::cmp::PartialOrd")))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(PartialEq(bound), PartialOrd(bound("T: core::cmp::PartialOrd")))]
    struct Tuple<T>(T);

    assert!(
        Struct {
            f1: 2
        } > Struct {
            f1: 1
        }
    );

    assert!(
        Struct {
            f1: 1
        } < Struct {
            f1: 2
        }
    );

    assert!(Tuple(2) > Tuple(1));
    assert!(Tuple(1) < Tuple(2));
}

#[test]
fn rank_1() {
    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Struct {
        #[educe(PartialOrd(rank = 1))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Tuple(#[educe(PartialOrd(rank = 1))] u8, u8);

    assert!(
        Struct {
            f1: 2, f2: 1
        } < Struct {
            f1: 1, f2: 2
        }
    );

    assert!(
        Struct {
            f1: 1, f2: 2
        } > Struct {
            f1: 2, f2: 1
        }
    );

    assert!(Tuple(2, 1) < Tuple(1, 2));
    assert!(Tuple(1, 2) > Tuple(2, 1));
}

#[test]
fn rank_2() {
    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Struct {
        #[educe(PartialOrd(rank(1)))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Tuple(#[educe(PartialOrd(rank(1)))] u8, u8);

    assert!(
        Struct {
            f1: 2, f2: 1
        } < Struct {
            f1: 1, f2: 2
        }
    );

    assert!(
        Struct {
            f1: 1, f2: 2
        } > Struct {
            f1: 2, f2: 1
        }
    );

    assert!(Tuple(2, 1) < Tuple(1, 2));
    assert!(Tuple(1, 2) > Tuple(2, 1));
}

#[test]
fn rank_3() {
    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Struct {
        #[educe(PartialOrd(rank = 1))]
        f1: u8,
        #[educe(PartialOrd(rank = 0))]
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Tuple(#[educe(PartialOrd(rank = 1))] u8, #[educe(PartialOrd(rank = 0))] u8);

    assert!(
        Struct {
            f1: 2, f2: 1
        } < Struct {
            f1: 1, f2: 2
        }
    );

    assert!(
        Struct {
            f1: 1, f2: 2
        } > Struct {
            f1: 2, f2: 1
        }
    );

    assert!(Tuple(2, 1) < Tuple(1, 2));
    assert!(Tuple(1, 2) > Tuple(2, 1));
}

#[test]
fn rank_4() {
    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Struct {
        #[educe(PartialOrd(rank(1)))]
        f1: u8,
        #[educe(PartialOrd(rank(0)))]
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    struct Tuple(#[educe(PartialOrd(rank(1)))] u8, #[educe(PartialOrd(rank(0)))] u8);

    assert!(
        Struct {
            f1: 2, f2: 1
        } < Struct {
            f1: 1, f2: 2
        }
    );

    assert!(
        Struct {
            f1: 1, f2: 2
        } > Struct {
            f1: 2, f2: 1
        }
    );

    assert!(Tuple(2, 1) < Tuple(1, 2));
    assert!(Tuple(1, 2) > Tuple(2, 1));
}
