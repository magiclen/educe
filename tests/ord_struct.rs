#![cfg(all(feature = "Ord", feature = "PartialOrd"))]
#![no_std]

use core::{cmp::Ordering, marker::PhantomData};

use educe::Educe;

#[test]
fn empty() {
    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    struct Struct {}

    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    struct Tuple();

    assert_eq!(Ordering::Equal, Struct {}.cmp(&Struct {}));
    assert_eq!(Ordering::Equal, Tuple {}.cmp(&Tuple {}));
}

#[test]
fn basic_1() {
    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    struct Unit;

    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    struct Struct {
        f1: u8,
    }

    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
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
    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    struct Struct {
        f1: u8,
        f2: u8,
    }

    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
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
    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    struct Struct {
        #[educe(Ord = false)]
        f1: u8,
        f2: u8,
    }

    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    struct Tuple(#[educe(Ord = false)] u8, u8);

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
fn ignore_2() {
    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    struct Struct {
        #[educe(Ord(ignore))]
        f1: u8,
        f2: u8,
    }

    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
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
fn method_1() {
    fn cmp(a: &u8, b: &u8) -> Ordering {
        b.cmp(a)
    }

    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    struct Struct {
        f1: u8,
        #[educe(Ord(method = cmp))]
        f2: u8,
    }

    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    struct Tuple(u8, #[educe(Ord(method = cmp))] u8);

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
fn method_2() {
    fn cmp(a: &u8, b: &u8) -> Ordering {
        b.cmp(a)
    }

    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    struct Struct {
        f1: u8,
        #[educe(Ord(method(cmp)))]
        f2: u8,
    }

    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    struct Tuple(u8, #[educe(Ord(method(cmp)))] u8);

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
fn rank_1() {
    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    struct Struct {
        #[educe(Ord(rank = 1))]
        f1: u8,
        f2: u8,
    }

    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    struct Tuple(#[educe(Ord(rank = 1))] u8, u8);

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
    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    struct Struct {
        #[educe(Ord(rank(1)))]
        f1: u8,
        f2: u8,
    }

    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    struct Tuple(#[educe(Ord(rank(1)))] u8, u8);

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
    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    struct Struct {
        #[educe(Ord(rank = 1))]
        f1: u8,
        #[educe(Ord(rank = 0))]
        f2: u8,
    }

    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    struct Tuple(#[educe(Ord(rank = 1))] u8, #[educe(Ord(rank = 0))] u8);

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
    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    struct Struct {
        #[educe(Ord(rank(1)))]
        f1: u8,
        #[educe(Ord(rank(0)))]
        f2: u8,
    }

    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    struct Tuple(#[educe(Ord(rank(1)))] u8, #[educe(Ord(rank(0)))] u8);

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
    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    struct Struct<T> {
        f1: T,
    }

    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
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
    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord(bound = "T: core::cmp::Ord"))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord(bound = "T: core::cmp::Ord"))]
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
    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord(bound(T: core::cmp::Ord)))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord(bound(T: core::cmp::Ord)))]
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
    #[educe(Ord)]
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

    impl<T: PartialEq, U: Suitable> Eq for Struct<T, U> {}

    impl<T: PartialOrd, U: Suitable> PartialOrd for Struct<T, U> {
        fn partial_cmp(&self, other: &Struct<T, U>) -> Option<Ordering> {
            self.f1.partial_cmp(&other.f1)
        }
    }

    #[derive(Educe)]
    #[educe(Ord)]
    struct Tuple<T, U>(T, PhantomData<U>);

    impl<T: PartialEq, U: Suitable> PartialEq for Tuple<T, U> {
        fn eq(&self, other: &Tuple<T, U>) -> bool {
            self.0.eq(&other.0)
        }
    }

    impl<T: PartialEq, U: Suitable> Eq for Tuple<T, U> {}

    impl<T: PartialOrd, U: Suitable> PartialOrd for Tuple<T, U> {
        fn partial_cmp(&self, other: &Tuple<T, U>) -> Option<Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }

    assert_eq!(
        Ord::cmp(
            &Struct {
                f1: 1, f2: phantom
            },
            &Struct {
                f1: 1, f2: phantom
            }
        ),
        Ordering::Equal
    );

    assert_eq!(
        Ord::cmp(
            &Struct {
                f1: 1, f2: phantom
            },
            &Struct {
                f1: 2, f2: phantom
            }
        ),
        Ordering::Less
    );

    assert_eq!(Ord::cmp(&Tuple(1, phantom), &Tuple(1, phantom)), Ordering::Equal);
    assert_eq!(Ord::cmp(&Tuple(1, phantom), &Tuple(2, phantom)), Ordering::Less);
}

#[test]
fn use_partial_ord_attr_ignore() {
    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    struct Struct {
        #[educe(PartialOrd(ignore))]
        f1: u8,
        f2: u8,
    }

    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    struct Tuple(#[educe(PartialOrd(ignore))] u8, u8);

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
