#![cfg(all(feature = "Eq", feature = "PartialEq"))]
#![no_std]

use core::marker::PhantomData;

use educe::Educe;

#[test]
fn empty() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Struct {}

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Tuple();

    assert!(Struct {} == Struct {});
    assert!(Tuple() == Tuple());
}

#[test]
fn basic() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Unit;

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Struct {
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Tuple(u8);

    assert!(Unit == Unit);

    assert!(
        Struct {
            f1: 1
        } == Struct {
            f1: 1
        }
    );

    assert!(
        Struct {
            f1: 1
        } != Struct {
            f1: 2
        }
    );

    assert!(Tuple(1) == Tuple(1));
    assert!(Tuple(1) != Tuple(2));
}

#[allow(dead_code)]
#[test]
fn ignore_1() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Struct {
        #[educe(Eq = false)]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Tuple(#[educe(Eq = false)] u8, u8);

    assert!(
        Struct {
            f1: 1, f2: 2
        } == Struct {
            f1: 1, f2: 2
        }
    );

    assert!(
        Struct {
            f1: 1, f2: 2
        } == Struct {
            f1: 2, f2: 2
        }
    );

    assert!(
        Struct {
            f1: 1, f2: 2
        } != Struct {
            f1: 2, f2: 3
        }
    );

    assert!(Tuple(1, 2) == Tuple(1, 2));
    assert!(Tuple(1, 2) == Tuple(2, 2));
    assert!(Tuple(1, 2) != Tuple(2, 3));
}

#[allow(dead_code)]
#[test]
fn ignore_2() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Struct {
        #[educe(Eq(ignore))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Tuple(#[educe(Eq(ignore))] u8, u8);

    assert!(
        Struct {
            f1: 1, f2: 2
        } == Struct {
            f1: 1, f2: 2
        }
    );

    assert!(
        Struct {
            f1: 1, f2: 2
        } == Struct {
            f1: 2, f2: 2
        }
    );

    assert!(
        Struct {
            f1: 1, f2: 2
        } != Struct {
            f1: 2, f2: 3
        }
    );

    assert!(Tuple(1, 2) == Tuple(1, 2));
    assert!(Tuple(1, 2) == Tuple(2, 2));
    assert!(Tuple(1, 2) != Tuple(2, 3));
}

#[test]
fn method_1() {
    fn eq(a: &u8, b: &u8) -> bool {
        a != b
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Struct {
        #[educe(Eq(method = eq))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Tuple(#[educe(Eq(method = eq))] u8, u8);

    assert!(
        Struct {
            f1: 1, f2: 2
        } != Struct {
            f1: 1, f2: 2
        }
    );

    assert!(
        Struct {
            f1: 1, f2: 2
        } == Struct {
            f1: 2, f2: 2
        }
    );

    assert!(
        Struct {
            f1: 1, f2: 2
        } != Struct {
            f1: 2, f2: 3
        }
    );

    assert!(Tuple(1, 2) != Tuple(1, 2));
    assert!(Tuple(1, 2) == Tuple(2, 2));
    assert!(Tuple(1, 2) != Tuple(2, 3));
}

#[test]
fn method_2() {
    fn eq(a: &u8, b: &u8) -> bool {
        a != b
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Struct {
        #[educe(Eq(method(eq)))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Tuple(#[educe(Eq(method(eq)))] u8, u8);

    assert!(
        Struct {
            f1: 1, f2: 2
        } != Struct {
            f1: 1, f2: 2
        }
    );

    assert!(
        Struct {
            f1: 1, f2: 2
        } == Struct {
            f1: 2, f2: 2
        }
    );

    assert!(
        Struct {
            f1: 1, f2: 2
        } != Struct {
            f1: 2, f2: 3
        }
    );

    assert!(Tuple(1, 2) != Tuple(1, 2));
    assert!(Tuple(1, 2) == Tuple(2, 2));
    assert!(Tuple(1, 2) != Tuple(2, 3));
}

#[test]
fn bound_1() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Tuple<T>(T);

    assert!(
        Struct {
            f1: 1
        } == Struct {
            f1: 1
        }
    );

    assert!(
        Struct {
            f1: 1
        } != Struct {
            f1: 2
        }
    );

    assert!(Tuple(1) == Tuple(1));
    assert!(Tuple(1) != Tuple(2));
}

#[test]
fn bound_2() {
    #[derive(Educe)]
    #[educe(PartialEq(bound = "T: core::cmp::PartialEq"), Eq)]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(PartialEq(bound = "T: core::cmp::PartialEq"), Eq)]
    struct Tuple<T>(T);

    assert!(
        Struct {
            f1: 1
        } == Struct {
            f1: 1
        }
    );

    assert!(
        Struct {
            f1: 1
        } != Struct {
            f1: 2
        }
    );

    assert!(Tuple(1) == Tuple(1));
    assert!(Tuple(1) != Tuple(2));
}

#[test]
fn bound_3() {
    #[derive(Educe)]
    #[educe(PartialEq(bound(T: core::cmp::PartialEq)), Eq)]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(PartialEq(bound(T: core::cmp::PartialEq)), Eq)]
    struct Tuple<T>(T);

    assert!(
        Struct {
            f1: 1
        } == Struct {
            f1: 1
        }
    );

    assert!(
        Struct {
            f1: 1
        } != Struct {
            f1: 2
        }
    );

    assert!(Tuple(1) == Tuple(1));
    assert!(Tuple(1) != Tuple(2));
}

#[test]
fn bound_4() {
    trait Suitable {}
    struct SuitableNotEq;
    impl Suitable for SuitableNotEq {}
    let phantom = PhantomData::<SuitableNotEq>;

    #[derive(Educe)]
    #[educe(Eq)]
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

#[allow(dead_code)]
#[test]
fn use_partial_eq_attr_ignore() {
    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Struct {
        #[educe(PartialEq(ignore))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Tuple(#[educe(PartialEq(ignore))] u8, u8);

    assert!(
        Struct {
            f1: 1, f2: 2
        } == Struct {
            f1: 1, f2: 2
        }
    );

    assert!(
        Struct {
            f1: 1, f2: 2
        } == Struct {
            f1: 2, f2: 2
        }
    );

    assert!(
        Struct {
            f1: 1, f2: 2
        } != Struct {
            f1: 2, f2: 3
        }
    );

    assert!(Tuple(1, 2) == Tuple(1, 2));
    assert!(Tuple(1, 2) == Tuple(2, 2));
    assert!(Tuple(1, 2) != Tuple(2, 3));
}
