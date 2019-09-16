#![cfg(all(feature = "PartialEq", feature = "PartialOrd"))]
#![no_std]

#[macro_use]
extern crate educe;

use core::cmp::Ordering;

#[test]
fn basic_1() {
    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    enum Enum {
        Unit,
        Unit2,
        Struct {
            f1: u8,
        },
        Tuple(u8),
    }

    assert!(Enum::Unit == Enum::Unit);
    assert!(Enum::Unit < Enum::Unit2);

    assert!(
        Enum::Struct {
            f1: 2
        } > Enum::Struct {
            f1: 1
        }
    );

    assert!(
        Enum::Struct {
            f1: 1
        } < Enum::Struct {
            f1: 2
        }
    );

    assert!(Enum::Tuple(2) > Enum::Tuple(1));
    assert!(Enum::Tuple(1) < Enum::Tuple(2));
}

#[test]
fn basic_2() {
    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    enum Enum {
        Struct {
            f1: u8,
            f2: u8,
        },
        Tuple(u8, u8),
    }

    assert!(
        Enum::Struct {
            f1: 2,
            f2: 1
        } > Enum::Struct {
            f1: 1,
            f2: 2
        }
    );

    assert!(
        Enum::Struct {
            f1: 1,
            f2: 2
        } < Enum::Struct {
            f1: 2,
            f2: 1
        }
    );

    assert!(Enum::Tuple(2, 1) > Enum::Tuple(1, 2));
    assert!(Enum::Tuple(1, 2) < Enum::Tuple(2, 1));
}

#[test]
fn basic_3() {
    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    enum Enum {
        A = 2,
        B = 1,
    }

    assert!(Enum::A > Enum::B);
}

#[test]
fn ignore() {
    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    enum Enum {
        Struct {
            #[educe(PartialOrd(ignore))]
            f1: u8,
            f2: u8,
        },
        Tuple(#[educe(PartialOrd(ignore))] u8, u8),
    }

    assert_eq!(
        Some(Ordering::Greater),
        Enum::Struct {
            f1: 1,
            f2: 3
        }
        .partial_cmp(&Enum::Struct {
            f1: 1,
            f2: 2
        })
    );

    assert_eq!(
        Some(Ordering::Less),
        Enum::Struct {
            f1: 1,
            f2: 2
        }
        .partial_cmp(&Enum::Struct {
            f1: 1,
            f2: 3
        })
    );

    assert_eq!(
        Some(Ordering::Equal),
        Enum::Struct {
            f1: 2,
            f2: 2
        }
        .partial_cmp(&Enum::Struct {
            f1: 1,
            f2: 2
        })
    );

    assert_eq!(Some(Ordering::Greater), Enum::Tuple(1, 3).partial_cmp(&Enum::Tuple(1, 2)));
    assert_eq!(Some(Ordering::Less), Enum::Tuple(1, 2).partial_cmp(&Enum::Tuple(1, 3)));
    assert_eq!(Some(Ordering::Equal), Enum::Tuple(2, 2).partial_cmp(&Enum::Tuple(1, 2)));
}

#[test]
fn compare_without_trait_1() {
    fn partial_cmp(a: &u8, b: &u8) -> Option<Ordering> {
        if a > b {
            Some(Ordering::Less)
        } else if a < b {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }

    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    enum Enum {
        Struct {
            f1: u8,
            #[educe(PartialOrd(method = "partial_cmp"))]
            f2: u8,
        },
        Tuple(u8, #[educe(PartialOrd(method = "partial_cmp"))] u8),
    }

    assert_eq!(
        Some(Ordering::Less),
        Enum::Struct {
            f1: 1,
            f2: 3
        }
        .partial_cmp(&Enum::Struct {
            f1: 1,
            f2: 2
        })
    );

    assert_eq!(
        Some(Ordering::Greater),
        Enum::Struct {
            f1: 1,
            f2: 2
        }
        .partial_cmp(&Enum::Struct {
            f1: 1,
            f2: 3
        })
    );

    assert_eq!(
        Some(Ordering::Equal),
        Enum::Struct {
            f1: 1,
            f2: 2
        }
        .partial_cmp(&Enum::Struct {
            f1: 1,
            f2: 2
        })
    );

    assert_eq!(Some(Ordering::Less), Enum::Tuple(1, 3).partial_cmp(&Enum::Tuple(1, 2)));
    assert_eq!(Some(Ordering::Greater), Enum::Tuple(1, 2).partial_cmp(&Enum::Tuple(1, 3)));
    assert_eq!(Some(Ordering::Equal), Enum::Tuple(1, 2).partial_cmp(&Enum::Tuple(1, 2)));
}

#[test]
fn compare_without_trait_2() {
    fn partial_cmp(a: &u8, b: &u8) -> Option<Ordering> {
        if a > b {
            Some(Ordering::Less)
        } else if a < b {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }

    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    enum Enum {
        Struct {
            f1: u8,
            #[educe(PartialOrd(method("partial_cmp")))]
            f2: u8,
        },
        Tuple(u8, #[educe(PartialOrd(method("partial_cmp")))] u8),
    }

    assert_eq!(
        Some(Ordering::Less),
        Enum::Struct {
            f1: 1,
            f2: 3
        }
        .partial_cmp(&Enum::Struct {
            f1: 1,
            f2: 2
        })
    );

    assert_eq!(
        Some(Ordering::Greater),
        Enum::Struct {
            f1: 1,
            f2: 2
        }
        .partial_cmp(&Enum::Struct {
            f1: 1,
            f2: 3
        })
    );

    assert_eq!(
        Some(Ordering::Equal),
        Enum::Struct {
            f1: 1,
            f2: 2
        }
        .partial_cmp(&Enum::Struct {
            f1: 1,
            f2: 2
        })
    );

    assert_eq!(Some(Ordering::Less), Enum::Tuple(1, 3).partial_cmp(&Enum::Tuple(1, 2)));
    assert_eq!(Some(Ordering::Greater), Enum::Tuple(1, 2).partial_cmp(&Enum::Tuple(1, 3)));
    assert_eq!(Some(Ordering::Equal), Enum::Tuple(1, 2).partial_cmp(&Enum::Tuple(1, 2)));
}

#[test]
fn compare_with_trait_1() {
    trait A {
        fn partial_cmp(&self, b: &Self) -> Option<Ordering>;
    }

    impl A for u8 {
        fn partial_cmp(&self, b: &u8) -> Option<Ordering> {
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
    enum Enum {
        Struct {
            f1: u8,
            #[educe(PartialOrd(trait = "A"))]
            f2: u8,
        },
        Tuple(u8, #[educe(PartialOrd(trait = "A"))] u8),
    }

    assert_eq!(
        Some(Ordering::Less),
        Enum::Struct {
            f1: 1,
            f2: 3
        }
        .partial_cmp(&Enum::Struct {
            f1: 1,
            f2: 2
        })
    );

    assert_eq!(
        Some(Ordering::Greater),
        Enum::Struct {
            f1: 1,
            f2: 2
        }
        .partial_cmp(&Enum::Struct {
            f1: 1,
            f2: 3
        })
    );

    assert_eq!(
        Some(Ordering::Equal),
        Enum::Struct {
            f1: 1,
            f2: 2
        }
        .partial_cmp(&Enum::Struct {
            f1: 1,
            f2: 2
        })
    );

    assert_eq!(Some(Ordering::Less), Enum::Tuple(1, 3).partial_cmp(&Enum::Tuple(1, 2)));
    assert_eq!(Some(Ordering::Greater), Enum::Tuple(1, 2).partial_cmp(&Enum::Tuple(1, 3)));
    assert_eq!(Some(Ordering::Equal), Enum::Tuple(1, 2).partial_cmp(&Enum::Tuple(1, 2)));
}

#[test]
fn compare_with_trait_2() {
    trait A {
        fn partial_cmp(&self, b: &Self) -> Option<Ordering>;
    }

    impl A for u8 {
        fn partial_cmp(&self, b: &u8) -> Option<Ordering> {
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
    enum Enum {
        Struct {
            f1: u8,
            #[educe(PartialOrd(trait("A")))]
            f2: u8,
        },
        Tuple(u8, #[educe(PartialOrd(trait("A")))] u8),
    }

    assert_eq!(
        Some(Ordering::Less),
        Enum::Struct {
            f1: 1,
            f2: 3
        }
        .partial_cmp(&Enum::Struct {
            f1: 1,
            f2: 2
        })
    );

    assert_eq!(
        Some(Ordering::Greater),
        Enum::Struct {
            f1: 1,
            f2: 2
        }
        .partial_cmp(&Enum::Struct {
            f1: 1,
            f2: 3
        })
    );

    assert_eq!(
        Some(Ordering::Equal),
        Enum::Struct {
            f1: 1,
            f2: 2
        }
        .partial_cmp(&Enum::Struct {
            f1: 1,
            f2: 2
        })
    );

    assert_eq!(Some(Ordering::Less), Enum::Tuple(1, 3).partial_cmp(&Enum::Tuple(1, 2)));
    assert_eq!(Some(Ordering::Greater), Enum::Tuple(1, 2).partial_cmp(&Enum::Tuple(1, 3)));
    assert_eq!(Some(Ordering::Equal), Enum::Tuple(1, 2).partial_cmp(&Enum::Tuple(1, 2)));
}

#[test]
fn compare_with_trait_3() {
    trait A {
        fn compare(&self, b: &Self) -> Option<Ordering>;
    }

    impl A for u8 {
        fn compare(&self, b: &u8) -> Option<Ordering> {
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
    enum Enum {
        Struct {
            f1: u8,
            #[educe(PartialOrd(trait = "A", method = "compare"))]
            f2: u8,
        },
        Tuple(u8, #[educe(PartialOrd(trait = "A", method = "compare"))] u8),
    }

    assert_eq!(
        Some(Ordering::Less),
        Enum::Struct {
            f1: 1,
            f2: 3
        }
        .partial_cmp(&Enum::Struct {
            f1: 1,
            f2: 2
        })
    );

    assert_eq!(
        Some(Ordering::Greater),
        Enum::Struct {
            f1: 1,
            f2: 2
        }
        .partial_cmp(&Enum::Struct {
            f1: 1,
            f2: 3
        })
    );

    assert_eq!(
        Some(Ordering::Equal),
        Enum::Struct {
            f1: 1,
            f2: 2
        }
        .partial_cmp(&Enum::Struct {
            f1: 1,
            f2: 2
        })
    );

    assert_eq!(Some(Ordering::Less), Enum::Tuple(1, 3).partial_cmp(&Enum::Tuple(1, 2)));
    assert_eq!(Some(Ordering::Greater), Enum::Tuple(1, 2).partial_cmp(&Enum::Tuple(1, 3)));
    assert_eq!(Some(Ordering::Equal), Enum::Tuple(1, 2).partial_cmp(&Enum::Tuple(1, 2)));
}

#[test]
fn compare_with_trait_4() {
    trait A {
        fn compare(&self, b: &Self) -> Option<Ordering>;
    }

    impl A for u8 {
        fn compare(&self, b: &u8) -> Option<Ordering> {
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
    enum Enum {
        Struct {
            f1: u8,
            #[educe(PartialOrd(trait("A"), method("compare")))]
            f2: u8,
        },
        Tuple(u8, #[educe(PartialOrd(trait("A"), method("compare")))] u8),
    }

    assert_eq!(
        Some(Ordering::Less),
        Enum::Struct {
            f1: 1,
            f2: 3
        }
        .partial_cmp(&Enum::Struct {
            f1: 1,
            f2: 2
        })
    );

    assert_eq!(
        Some(Ordering::Greater),
        Enum::Struct {
            f1: 1,
            f2: 2
        }
        .partial_cmp(&Enum::Struct {
            f1: 1,
            f2: 3
        })
    );

    assert_eq!(
        Some(Ordering::Equal),
        Enum::Struct {
            f1: 1,
            f2: 2
        }
        .partial_cmp(&Enum::Struct {
            f1: 1,
            f2: 2
        })
    );

    assert_eq!(Some(Ordering::Less), Enum::Tuple(1, 3).partial_cmp(&Enum::Tuple(1, 2)));
    assert_eq!(Some(Ordering::Greater), Enum::Tuple(1, 2).partial_cmp(&Enum::Tuple(1, 3)));
    assert_eq!(Some(Ordering::Equal), Enum::Tuple(1, 2).partial_cmp(&Enum::Tuple(1, 2)));
}

#[test]
fn bound_1() {
    #[derive(Educe)]
    #[educe(PartialEq(bound), PartialOrd(bound))]
    enum Enum<T> {
        Struct {
            f1: T,
        },
        Tuple(T),
    }

    assert!(
        Enum::Struct {
            f1: 2
        } > Enum::Struct {
            f1: 1
        }
    );

    assert!(
        Enum::Struct {
            f1: 1
        } < Enum::Struct {
            f1: 2
        }
    );

    assert!(Enum::Tuple(2) > Enum::Tuple(1));
    assert!(Enum::Tuple(1) < Enum::Tuple(2));
}

#[test]
fn bound_2() {
    #[derive(Educe)]
    #[educe(PartialEq(bound), PartialOrd(bound = "T: core::cmp::PartialOrd"))]
    enum Enum<T> {
        Struct {
            f1: T,
        },
        Tuple(T),
    }

    assert!(
        Enum::Struct {
            f1: 2
        } > Enum::Struct {
            f1: 1
        }
    );

    assert!(
        Enum::Struct {
            f1: 1
        } < Enum::Struct {
            f1: 2
        }
    );

    assert!(Enum::Tuple(2) > Enum::Tuple(1));
    assert!(Enum::Tuple(1) < Enum::Tuple(2));
}

#[test]
fn bound_3() {
    #[derive(Educe)]
    #[educe(PartialEq(bound), PartialOrd(bound("T: core::cmp::PartialOrd")))]
    enum Enum<T> {
        Struct {
            f1: T,
        },
        Tuple(T),
    }

    assert!(
        Enum::Struct {
            f1: 2
        } > Enum::Struct {
            f1: 1
        }
    );

    assert!(
        Enum::Struct {
            f1: 1
        } < Enum::Struct {
            f1: 2
        }
    );

    assert!(Enum::Tuple(2) > Enum::Tuple(1));
    assert!(Enum::Tuple(1) < Enum::Tuple(2));
}

#[test]
fn field_rank_1() {
    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    enum Enum {
        Struct {
            #[educe(PartialOrd(rank = 1))]
            f1: u8,
            f2: u8,
        },
        Tuple(#[educe(PartialOrd(rank = 1))] u8, u8),
    }

    assert!(
        Enum::Struct {
            f1: 2,
            f2: 1
        } < Enum::Struct {
            f1: 1,
            f2: 2
        }
    );

    assert!(
        Enum::Struct {
            f1: 1,
            f2: 2
        } > Enum::Struct {
            f1: 2,
            f2: 1
        }
    );

    assert!(Enum::Tuple(2, 1) < Enum::Tuple(1, 2));
    assert!(Enum::Tuple(1, 2) > Enum::Tuple(2, 1));
}

#[test]
fn field_rank_2() {
    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    enum Enum {
        Struct {
            #[educe(PartialOrd(rank(1)))]
            f1: u8,
            f2: u8,
        },
        Tuple(#[educe(PartialOrd(rank(1)))] u8, u8),
    }

    assert!(
        Enum::Struct {
            f1: 2,
            f2: 1
        } < Enum::Struct {
            f1: 1,
            f2: 2
        }
    );

    assert!(
        Enum::Struct {
            f1: 1,
            f2: 2
        } > Enum::Struct {
            f1: 2,
            f2: 1
        }
    );

    assert!(Enum::Tuple(2, 1) < Enum::Tuple(1, 2));
    assert!(Enum::Tuple(1, 2) > Enum::Tuple(2, 1));
}

#[test]
fn field_rank_3() {
    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    enum Enum {
        Struct {
            #[educe(PartialOrd(rank = 1))]
            f1: u8,
            #[educe(PartialOrd(rank = 0))]
            f2: u8,
        },
        Tuple(#[educe(PartialOrd(rank = 1))] u8, #[educe(PartialOrd(rank = 0))] u8),
    }

    assert!(
        Enum::Struct {
            f1: 2,
            f2: 1
        } < Enum::Struct {
            f1: 1,
            f2: 2
        }
    );

    assert!(
        Enum::Struct {
            f1: 1,
            f2: 2
        } > Enum::Struct {
            f1: 2,
            f2: 1
        }
    );

    assert!(Enum::Tuple(2, 1) < Enum::Tuple(1, 2));
    assert!(Enum::Tuple(1, 2) > Enum::Tuple(2, 1));
}

#[test]
fn field_rank_4() {
    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    enum Enum {
        Struct {
            #[educe(PartialOrd(rank(1)))]
            f1: u8,
            #[educe(PartialOrd(rank(0)))]
            f2: u8,
        },
        Tuple(#[educe(PartialOrd(rank(1)))] u8, #[educe(PartialOrd(rank(0)))] u8),
    }

    assert!(
        Enum::Struct {
            f1: 2,
            f2: 1
        } < Enum::Struct {
            f1: 1,
            f2: 2
        }
    );

    assert!(
        Enum::Struct {
            f1: 1,
            f2: 2
        } > Enum::Struct {
            f1: 2,
            f2: 1
        }
    );

    assert!(Enum::Tuple(2, 1) < Enum::Tuple(1, 2));
    assert!(Enum::Tuple(1, 2) > Enum::Tuple(2, 1));
}

#[test]
fn variant_rank_1() {
    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    enum Enum {
        #[educe(PartialOrd(rank = 2))]
        Two,
        #[educe(PartialOrd(rank = 1))]
        One,
    }

    assert!(Enum::Two > Enum::One);
}

#[test]
fn variant_rank_2() {
    #[derive(Educe)]
    #[educe(PartialEq, PartialOrd)]
    enum Enum {
        #[educe(PartialOrd(rank(2)))]
        Two,
        #[educe(PartialOrd(rank(1)))]
        One,
    }

    assert!(Enum::Two > Enum::One);
}
