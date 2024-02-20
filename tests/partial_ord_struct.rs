#![cfg(feature = "PartialOrd")]
#![no_std]

use core::{cmp::Ordering, marker::PhantomData};

use educe::Educe;

#[test]
fn empty() {
    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
    struct Struct {}

    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
    struct Tuple();

    assert_eq!(Some(Ordering::Equal), Struct {}.partial_cmp(&Struct {}));
    assert_eq!(Some(Ordering::Equal), Tuple {}.partial_cmp(&Tuple {}));
}

#[test]
fn basic_1() {
    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
    struct Unit;

    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
    struct Struct {
        f1: u8,
    }

    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
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
    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
    struct Struct {
        f1: u8,
        f2: u8,
    }

    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
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
fn ignore_1() {
    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
    struct Struct {
        #[educe(PartialOrd = false)]
        f1: u8,
        f2: u8,
    }

    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
    struct Tuple(#[educe(PartialOrd = false)] u8, u8);

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
fn ignore_2() {
    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
    struct Struct {
        #[educe(PartialOrd(ignore))]
        f1: u8,
        f2: u8,
    }

    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
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
fn method_1() {
    fn partial_cmp(a: &u8, b: &u8) -> Option<Ordering> {
        b.partial_cmp(a)
    }

    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
    struct Struct {
        f1: u8,
        #[educe(PartialOrd(method = partial_cmp))]
        f2: u8,
    }

    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
    struct Tuple(u8, #[educe(PartialOrd(method = partial_cmp))] u8);

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
fn method_2() {
    fn partial_cmp(a: &u8, b: &u8) -> Option<Ordering> {
        b.partial_cmp(a)
    }

    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
    struct Struct {
        f1: u8,
        #[educe(PartialOrd(method(partial_cmp)))]
        f2: u8,
    }

    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
    struct Tuple(u8, #[educe(PartialOrd(method(partial_cmp)))] u8);

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
fn rank_1() {
    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
    struct Struct {
        #[educe(PartialOrd(rank = 1))]
        f1: u8,
        f2: u8,
    }

    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
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
    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
    struct Struct {
        #[educe(PartialOrd(rank(1)))]
        f1: u8,
        f2: u8,
    }

    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
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
    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
    struct Struct {
        #[educe(PartialOrd(rank = 1))]
        f1: u8,
        #[educe(PartialOrd(rank = 0))]
        f2: u8,
    }

    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
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
    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
    struct Struct {
        #[educe(PartialOrd(rank(1)))]
        f1: u8,
        #[educe(PartialOrd(rank(0)))]
        f2: u8,
    }

    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
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

#[test]
fn bound_1() {
    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
    struct Struct<T> {
        f1: T,
    }

    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
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
    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd(bound = "T: core::cmp::PartialOrd"))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd(bound = "T: core::cmp::PartialOrd"))]
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
    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd(bound(T: core::cmp::PartialOrd)))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd(bound(T: core::cmp::PartialOrd)))]
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
fn bound_4() {
    trait Suitable {}
    struct SuitableNotEq;
    impl Suitable for SuitableNotEq {}
    let phantom = PhantomData::<SuitableNotEq>;

    #[derive(Educe)]
    #[educe(PartialOrd)]
    struct Struct<T, U> {
        f1: T,
        // PhantomData is Eq (all PhantomData are equal to all others)
        f2: PhantomData<U>,
    }

    impl<T: PartialEq, U: Suitable> PartialEq for Struct<T, U> {
        fn eq(&self, other: &Struct<T, U>) -> bool {
            self.f1.eq(&other.f1)
        }
    }

    #[derive(Educe)]
    #[educe(Eq)]
    struct Tuple<T, U>(T, PhantomData<U>);

    impl<T: PartialEq, U: Suitable> PartialEq for Tuple<T, U> {
        fn eq(&self, other: &Tuple<T, U>) -> bool {
            self.0.eq(&other.0)
        }
    }
    assert!(
        Struct {
            f1: 1, f2: phantom
        } == Struct {
            f1: 1, f2: phantom
        }
    );

    assert!(
        Struct {
            f1: 1, f2: phantom
        } != Struct {
            f1: 2, f2: phantom
        }
    );

    assert!(Tuple(1, phantom) == Tuple(1, phantom));
    assert!(Tuple(1, phantom) != Tuple(2, phantom));
}
