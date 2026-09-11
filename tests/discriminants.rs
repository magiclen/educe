#![cfg(any(feature = "Ord", feature = "PartialOrd"))]
#![no_std]

use core::cmp::Ordering;

use educe::Educe;

#[allow(non_camel_case_types, dead_code)]
#[test]
fn primitive_names() {
    type isize = u8;
    type i16 = u8;

    #[derive(PartialEq, Eq, Educe)]
    #[cfg_attr(feature = "Ord", educe(Ord))]
    #[cfg_attr(feature = "PartialOrd", educe(PartialOrd))]
    #[cfg_attr(not(feature = "PartialOrd"), derive(PartialOrd))]
    enum DefaultRepr {
        First = -2,
        Next,
    }

    #[derive(PartialEq, Eq, Educe)]
    #[cfg_attr(feature = "Ord", educe(Ord))]
    #[cfg_attr(feature = "PartialOrd", educe(PartialOrd))]
    #[cfg_attr(not(feature = "PartialOrd"), derive(PartialOrd))]
    #[repr(i16)]
    enum ExplicitRepr {
        First = -2,
        Next,
    }

    assert_eq!(Some(Ordering::Less), DefaultRepr::First.partial_cmp(&DefaultRepr::Next));
    assert_eq!(Some(Ordering::Less), ExplicitRepr::First.partial_cmp(&ExplicitRepr::Next));
    #[cfg(feature = "Ord")]
    {
        assert_eq!(Ordering::Less, DefaultRepr::First.cmp(&DefaultRepr::Next));
        assert_eq!(Ordering::Less, ExplicitRepr::First.cmp(&ExplicitRepr::Next));
    }
}

#[test]
fn expressions_and_alignment() {
    const BASE: isize = 4;

    #[derive(PartialEq, Eq, Educe)]
    #[cfg_attr(feature = "Ord", educe(Ord))]
    #[cfg_attr(feature = "PartialOrd", educe(PartialOrd))]
    #[cfg_attr(not(feature = "PartialOrd"), derive(PartialOrd))]
    #[repr(C, align(8))]
    enum Enum {
        First = -BASE,
        Next,
        Last  = 1 << 3,
    }

    assert_eq!(Some(Ordering::Less), Enum::First.partial_cmp(&Enum::Next));
    assert_eq!(Some(Ordering::Less), Enum::Next.partial_cmp(&Enum::Last));
    #[cfg(feature = "Ord")]
    {
        assert_eq!(Ordering::Less, Enum::First.cmp(&Enum::Next));
        assert_eq!(Ordering::Less, Enum::Next.cmp(&Enum::Last));
    }
}

#[test]
fn unsigned_range() {
    #[derive(PartialEq, Eq, Educe)]
    #[cfg_attr(feature = "Ord", educe(Ord))]
    #[cfg_attr(feature = "PartialOrd", educe(PartialOrd))]
    #[cfg_attr(not(feature = "PartialOrd"), derive(PartialOrd))]
    #[repr(u128)]
    enum Enum {
        High = 170141183460469231731687303715884105728,
        Next,
        Max  = u128::MAX,
    }

    assert_eq!(Some(Ordering::Less), Enum::High.partial_cmp(&Enum::Next));
    assert_eq!(Some(Ordering::Greater), Enum::Max.partial_cmp(&Enum::Next));
    #[cfg(feature = "Ord")]
    {
        assert_eq!(Ordering::Less, Enum::High.cmp(&Enum::Next));
        assert_eq!(Ordering::Greater, Enum::Max.cmp(&Enum::Next));
    }
}

#[test]
fn generic_fields() {
    const FIRST: i16 = -4;

    #[derive(PartialEq, Eq, Educe)]
    #[cfg_attr(feature = "Ord", educe(Ord))]
    #[cfg_attr(feature = "PartialOrd", educe(PartialOrd))]
    #[cfg_attr(not(feature = "PartialOrd"), derive(PartialOrd))]
    #[repr(i16, align(8))]
    enum Enum<T> {
        Tuple(T) = FIRST,
        Named { value: T },
    }

    assert_eq!(
        Some(Ordering::Less),
        Enum::Tuple(9u8).partial_cmp(&Enum::Named {
            value: 1
        })
    );
    assert_eq!(Some(Ordering::Less), Enum::Tuple(1u8).partial_cmp(&Enum::Tuple(2)));
    #[cfg(feature = "Ord")]
    {
        assert_eq!(
            Ordering::Less,
            Enum::Tuple(9u8).cmp(&Enum::Named {
                value: 1
            })
        );
        assert_eq!(Ordering::Less, Enum::Tuple(1u8).cmp(&Enum::Tuple(2)));
    }
}

#[test]
fn self_expression() {
    #[derive(PartialEq, Eq, Educe)]
    #[cfg_attr(feature = "Ord", educe(Ord))]
    #[cfg_attr(feature = "PartialOrd", educe(PartialOrd))]
    #[cfg_attr(not(feature = "PartialOrd"), derive(PartialOrd))]
    #[repr(i16)]
    enum Enum {
        First    = 1,
        Explicit = Self::First as i16 + 2,
        Next,
    }

    assert_eq!(Some(Ordering::Less), Enum::First.partial_cmp(&Enum::Explicit));
    assert_eq!(Some(Ordering::Less), Enum::Explicit.partial_cmp(&Enum::Next));
    #[cfg(feature = "Ord")]
    {
        assert_eq!(Ordering::Less, Enum::First.cmp(&Enum::Explicit));
        assert_eq!(Ordering::Less, Enum::Explicit.cmp(&Enum::Next));
    }
}
