#![cfg(feature = "PartialOrd")]
#![no_std]

use core::cmp::Ordering;

use educe::Educe;

#[test]
fn empty() {
    #[allow(dead_code)]
    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
    enum Enum {}

    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
    enum Enum2 {
        Struct {},
        Tuple(),
    }

    assert!(Enum2::Struct {} < Enum2::Tuple());
}

#[test]
fn basic_1() {
    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
    enum Enum {
        Unit,
        Unit2,
        Struct { f1: u8 },
        Tuple(u8),
    }

    assert!(Enum::Unit == Enum::Unit);
    assert!(Enum::Unit < Enum::Unit2);
    assert!(
        Enum::Unit2
            < Enum::Struct {
                f1: 0
            }
    );

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
    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
    enum Enum {
        Struct { f1: u8, f2: u8 },
        Tuple(u8, u8),
    }

    assert!(
        Enum::Struct {
            f1: 2, f2: 1
        } > Enum::Struct {
            f1: 1, f2: 2
        }
    );

    assert!(
        Enum::Struct {
            f1: 1, f2: 2
        } < Enum::Struct {
            f1: 2, f2: 1
        }
    );

    assert!(Enum::Tuple(2, 1) > Enum::Tuple(1, 2));
    assert!(Enum::Tuple(1, 2) < Enum::Tuple(2, 1));
}

#[test]
fn basic_3() {
    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
    enum Enum {
        A = 2,
        B = 1,
    }

    assert!(Enum::A > Enum::B);
}

#[rustversion::since(1.66)]
#[test]
fn basic_4() {
    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
    #[repr(u8)]
    enum Enum {
        Unit   = 4,
        Unit2  = 3,
        Struct { f1: u8 } = 2,
        Tuple(u8) = 1,
    }

    assert!(Enum::Unit == Enum::Unit);
    assert!(Enum::Unit > Enum::Unit2);
    assert!(
        Enum::Unit2
            > Enum::Struct {
                f1: 9
            }
    );

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

    assert!(
        Enum::Struct {
            f1: 1
        } > Enum::Tuple(2)
    );
}

#[test]
fn ignore_1() {
    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
    enum Enum {
        Struct {
            #[educe(PartialOrd = false)]
            f1: u8,
            f2: u8,
        },
        Tuple(#[educe(PartialOrd = false)] u8, u8),
    }

    assert_eq!(
        Some(Ordering::Greater),
        Enum::Struct {
            f1: 1, f2: 3
        }
        .partial_cmp(&Enum::Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(
        Some(Ordering::Less),
        Enum::Struct {
            f1: 1, f2: 2
        }
        .partial_cmp(&Enum::Struct {
            f1: 1, f2: 3
        })
    );

    assert_eq!(
        Some(Ordering::Equal),
        Enum::Struct {
            f1: 2, f2: 2
        }
        .partial_cmp(&Enum::Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(Some(Ordering::Greater), Enum::Tuple(1, 3).partial_cmp(&Enum::Tuple(1, 2)));
    assert_eq!(Some(Ordering::Less), Enum::Tuple(1, 2).partial_cmp(&Enum::Tuple(1, 3)));
    assert_eq!(Some(Ordering::Equal), Enum::Tuple(2, 2).partial_cmp(&Enum::Tuple(1, 2)));
}

#[test]
fn ignore_2() {
    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
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
            f1: 1, f2: 3
        }
        .partial_cmp(&Enum::Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(
        Some(Ordering::Less),
        Enum::Struct {
            f1: 1, f2: 2
        }
        .partial_cmp(&Enum::Struct {
            f1: 1, f2: 3
        })
    );

    assert_eq!(
        Some(Ordering::Equal),
        Enum::Struct {
            f1: 2, f2: 2
        }
        .partial_cmp(&Enum::Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(Some(Ordering::Greater), Enum::Tuple(1, 3).partial_cmp(&Enum::Tuple(1, 2)));
    assert_eq!(Some(Ordering::Less), Enum::Tuple(1, 2).partial_cmp(&Enum::Tuple(1, 3)));
    assert_eq!(Some(Ordering::Equal), Enum::Tuple(2, 2).partial_cmp(&Enum::Tuple(1, 2)));
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
            f1: 2, f2: 1
        } < Enum::Struct {
            f1: 1, f2: 2
        }
    );

    assert!(
        Enum::Struct {
            f1: 1, f2: 2
        } > Enum::Struct {
            f1: 2, f2: 1
        }
    );

    assert!(Enum::Tuple(2, 1) < Enum::Tuple(1, 2));
    assert!(Enum::Tuple(1, 2) > Enum::Tuple(2, 1));
}

#[test]
fn rank_2() {
    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
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
            f1: 2, f2: 1
        } < Enum::Struct {
            f1: 1, f2: 2
        }
    );

    assert!(
        Enum::Struct {
            f1: 1, f2: 2
        } > Enum::Struct {
            f1: 2, f2: 1
        }
    );

    assert!(Enum::Tuple(2, 1) < Enum::Tuple(1, 2));
    assert!(Enum::Tuple(1, 2) > Enum::Tuple(2, 1));
}

#[test]
fn rank_3() {
    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
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
            f1: 2, f2: 1
        } < Enum::Struct {
            f1: 1, f2: 2
        }
    );

    assert!(
        Enum::Struct {
            f1: 1, f2: 2
        } > Enum::Struct {
            f1: 2, f2: 1
        }
    );

    assert!(Enum::Tuple(2, 1) < Enum::Tuple(1, 2));
    assert!(Enum::Tuple(1, 2) > Enum::Tuple(2, 1));
}

#[test]
fn rank_4() {
    #[derive(PartialEq, Educe)]
    #[educe(PartialOrd)]
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
            f1: 2, f2: 1
        } < Enum::Struct {
            f1: 1, f2: 2
        }
    );

    assert!(
        Enum::Struct {
            f1: 1, f2: 2
        } > Enum::Struct {
            f1: 2, f2: 1
        }
    );

    assert!(Enum::Tuple(2, 1) < Enum::Tuple(1, 2));
    assert!(Enum::Tuple(1, 2) > Enum::Tuple(2, 1));
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
