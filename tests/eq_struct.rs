#![cfg(all(feature = "Eq", feature = "PartialEq"))]
#![no_std]

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
        #[educe(PartialEq = false)]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Tuple(#[educe(PartialEq = false)] u8, u8);

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

#[test]
fn method_1() {
    fn eq(a: &u8, b: &u8) -> bool {
        a != b
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Struct {
        #[educe(PartialEq(method = eq))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Tuple(#[educe(PartialEq(method = eq))] u8, u8);

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
        #[educe(PartialEq(method(eq)))]
        f1: u8,
        f2: u8,
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Tuple(#[educe(PartialEq(method(eq)))] u8, u8);

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

#[test]
fn bound_4() {
    trait Suitable: PartialEq {}

    impl Suitable for u8 {}

    // Explicit bounds can be set on `PartialEq` and `Eq` separately.
    #[derive(Educe)]
    #[educe(PartialEq(bound(T: Suitable)), Eq(bound(T: Suitable)))]
    struct Struct<T> {
        f1: T,
    }

    fn assert_eq_impl<T: Eq>(_v: &T) {}

    assert_eq_impl(&Struct {
        f1: 1
    });
}

#[test]
fn bound_inheritance() {
    trait Suitable: PartialEq {}

    impl Suitable for u8 {}

    // With an automatic bound, `Eq` inherits the custom predicates of the `PartialEq` impl.
    #[derive(Educe)]
    #[educe(PartialEq(bound(T: Suitable)), Eq)]
    struct Struct<T> {
        f1: T,
    }

    fn assert_eq_impl<T: Eq>(_v: &T) {}

    assert_eq_impl(&Struct {
        f1: 1
    });
}

#[test]
#[allow(dead_code)]
fn ignored_generic_bound() {
    struct NotEq;

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Struct<T> {
        value:   u8,
        #[educe(PartialEq(ignore))]
        ignored: T,
    }

    fn equal<T: Eq>(a: T, b: T) -> bool {
        a == b
    }

    assert!(equal(
        Struct {
            value: 1, ignored: NotEq
        },
        Struct {
            value: 1, ignored: NotEq
        }
    ));
}

#[test]
fn custom_comparison_without_eq_bound() {
    fn equal<T>(_: &T, _: &T) -> bool {
        true
    }

    #[derive(Educe)]
    #[educe(PartialEq, Eq)]
    struct Value<T>(#[educe(PartialEq(method = equal))] T);

    fn assert_eq_impl<T: Eq>(value: T) {
        assert!(value == value);
    }

    assert_eq_impl(Value(1.0f64));
}

#[test]
fn explicit_bounds_keep_manual_equality() {
    #[derive(Educe)]
    #[educe(Eq(bound(false)))]
    struct Disabled(f64);

    impl PartialEq for Disabled {
        fn eq(&self, other: &Self) -> bool {
            self.0.to_bits() == other.0.to_bits()
        }
    }

    #[derive(Educe)]
    #[educe(Eq(bound(*)))]
    struct All<T>(T, f64);

    impl<T: PartialEq> PartialEq for All<T> {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0 && self.1.to_bits() == other.1.to_bits()
        }
    }

    #[derive(Educe)]
    #[educe(Eq(bound(T: Eq)))]
    struct Custom<T>(T, f64);

    impl<T: PartialEq> PartialEq for Custom<T> {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0 && self.1.to_bits() == other.1.to_bits()
        }
    }

    fn assert_eq_impl<T: Eq>(left: T, right: T) {
        assert!(left == right);
    }

    assert_eq_impl(Disabled(f64::NAN), Disabled(f64::NAN));
    assert_eq_impl(All(1u8, f64::NAN), All(1u8, f64::NAN));
    assert_eq_impl(Custom(1u8, f64::NAN), Custom(1u8, f64::NAN));
}
