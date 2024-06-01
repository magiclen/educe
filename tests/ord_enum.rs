#![cfg(all(feature = "Ord", feature = "PartialOrd"))]
#![no_std]

use core::cmp::Ordering;

use educe::Educe;

#[test]
fn empty() {
    #[allow(dead_code)]
    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    enum Enum {}

    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    enum Enum2 {
        Struct {},
        Tuple(),
    }

    assert!(Enum2::Struct {} < Enum2::Tuple());
}

#[test]
fn basic_1() {
    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
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
    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
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
    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    enum Enum {
        A = 2,
        B = 1,
    }

    assert!(Enum::A > Enum::B);
}

#[rustversion::since(1.66)]
#[test]
fn basic_4() {
    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
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
    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    enum Enum {
        Struct {
            #[educe(Ord = false)]
            f1: u8,
            f2: u8,
        },
        Tuple(#[educe(Ord = false)] u8, u8),
    }

    assert_eq!(
        Ordering::Greater,
        Enum::Struct {
            f1: 1, f2: 3
        }
        .cmp(&Enum::Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(
        Ordering::Less,
        Enum::Struct {
            f1: 1, f2: 2
        }
        .cmp(&Enum::Struct {
            f1: 1, f2: 3
        })
    );

    assert_eq!(
        Ordering::Equal,
        Enum::Struct {
            f1: 2, f2: 2
        }
        .cmp(&Enum::Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(Ordering::Greater, Enum::Tuple(1, 3).cmp(&Enum::Tuple(1, 2)));
    assert_eq!(Ordering::Less, Enum::Tuple(1, 2).cmp(&Enum::Tuple(1, 3)));
    assert_eq!(Ordering::Equal, Enum::Tuple(2, 2).cmp(&Enum::Tuple(1, 2)));
}

#[test]
fn ignore_2() {
    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    enum Enum {
        Struct {
            #[educe(Ord(ignore))]
            f1: u8,
            f2: u8,
        },
        Tuple(#[educe(Ord(ignore))] u8, u8),
    }

    assert_eq!(
        Ordering::Greater,
        Enum::Struct {
            f1: 1, f2: 3
        }
        .cmp(&Enum::Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(
        Ordering::Less,
        Enum::Struct {
            f1: 1, f2: 2
        }
        .cmp(&Enum::Struct {
            f1: 1, f2: 3
        })
    );

    assert_eq!(
        Ordering::Equal,
        Enum::Struct {
            f1: 2, f2: 2
        }
        .cmp(&Enum::Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(Ordering::Greater, Enum::Tuple(1, 3).cmp(&Enum::Tuple(1, 2)));
    assert_eq!(Ordering::Less, Enum::Tuple(1, 2).cmp(&Enum::Tuple(1, 3)));
    assert_eq!(Ordering::Equal, Enum::Tuple(2, 2).cmp(&Enum::Tuple(1, 2)));
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
    enum Enum {
        Struct {
            #[educe(Ord(rank = 1))]
            f1: u8,
            f2: u8,
        },
        Tuple(#[educe(Ord(rank = 1))] u8, u8),
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
    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    enum Enum {
        Struct {
            #[educe(Ord(rank(1)))]
            f1: u8,
            f2: u8,
        },
        Tuple(#[educe(Ord(rank(1)))] u8, u8),
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
    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    enum Enum {
        Struct {
            #[educe(Ord(rank = 1))]
            f1: u8,
            #[educe(Ord(rank = 0))]
            f2: u8,
        },
        Tuple(#[educe(Ord(rank = 1))] u8, #[educe(Ord(rank = 0))] u8),
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
    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    enum Enum {
        Struct {
            #[educe(Ord(rank(1)))]
            f1: u8,
            #[educe(Ord(rank(0)))]
            f2: u8,
        },
        Tuple(#[educe(Ord(rank(1)))] u8, #[educe(Ord(rank(0)))] u8),
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
fn use_partial_ord_attr_ignore() {
    #[derive(PartialEq, Eq, Educe)]
    #[educe(PartialOrd, Ord)]
    enum Enum {
        Struct {
            #[educe(PartialOrd(ignore))]
            f1: u8,
            f2: u8,
        },
        Tuple(#[educe(PartialOrd(ignore))] u8, u8),
    }

    assert_eq!(
        Ordering::Greater,
        Enum::Struct {
            f1: 1, f2: 3
        }
        .cmp(&Enum::Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(
        Ordering::Less,
        Enum::Struct {
            f1: 1, f2: 2
        }
        .cmp(&Enum::Struct {
            f1: 1, f2: 3
        })
    );

    assert_eq!(
        Ordering::Equal,
        Enum::Struct {
            f1: 2, f2: 2
        }
        .cmp(&Enum::Struct {
            f1: 1, f2: 2
        })
    );

    assert_eq!(Ordering::Greater, Enum::Tuple(1, 3).cmp(&Enum::Tuple(1, 2)));
    assert_eq!(Ordering::Less, Enum::Tuple(1, 2).cmp(&Enum::Tuple(1, 3)));
    assert_eq!(Ordering::Equal, Enum::Tuple(2, 2).cmp(&Enum::Tuple(1, 2)));
}
