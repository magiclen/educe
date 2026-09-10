#![cfg(feature = "Clone")]
#![no_std]
// Unlike the other Clone tests, this crate denies dead code so it works as a regression guard. A custom clone method is referenced only inside the generated `Clone` impl, whose body dead-code analysis skips because the impl is `#[automatically_derived]` and `Clone` carries `#[rustc_trivial_field_reads]`. Without the marker item Educe emits, the method would be reported as never used. The derived types and their fields are `pub` so they stay reachable and are not themselves reported, keeping the check focused on the methods.
#![deny(dead_code)]

use educe::Educe;

fn clone(v: &u8) -> u8 {
    *v
}

// A generic method is exercised as well. Because the type parameter also appears in a plain field, the derive adds a `T: Clone` bound, which the marker item has to reproduce in order to compile.
fn clone_generic<T: Clone>(v: &T) -> T {
    v.clone()
}

#[derive(Educe)]
#[educe(Clone)]
pub struct Struct {
    #[educe(Clone(method = clone))]
    pub f1: u8,
}

#[derive(Educe)]
#[educe(Clone)]
pub struct Generic<T> {
    pub normal:  T,
    #[educe(Clone(method = clone_generic))]
    pub special: T,
}

#[derive(Educe)]
#[educe(Clone)]
pub enum Enum<T> {
    Struct {
        normal:  T,
        #[educe(Clone(method = clone_generic))]
        special: T,
    },
    Tuple(T, #[educe(Clone(method = clone_generic))] T),
}

#[test]
fn method_is_marked_used() {
    let s = Struct {
        f1: 1
    }
    .clone();
    assert_eq!(1, s.f1);

    let g = Generic {
        normal: 1u8, special: 2
    }
    .clone();
    assert_eq!(1, g.normal);
    assert_eq!(2, g.special);

    if let Enum::Struct {
        normal,
        special,
    } = (Enum::Struct {
        normal: 1u8, special: 2
    })
    .clone()
    {
        assert_eq!(1, normal);
        assert_eq!(2, special);
    } else {
        panic!();
    }

    if let Enum::Tuple(a, b) = Enum::Tuple(1u8, 2).clone() {
        assert_eq!(1, a);
        assert_eq!(2, b);
    } else {
        panic!();
    }
}

#[derive(Educe)]
#[educe(Clone(bound(T: Clone)))]
pub struct Associated<T> {
    #[educe(Clone(method = Self::copy_value))]
    pub value: T,
}

impl<T: Clone> Associated<T> {
    fn copy_value(value: &T) -> T {
        value.clone()
    }
}

#[derive(Educe)]
#[educe(Clone(bound(T: Clone)))]
pub enum AssociatedEnum<T> {
    Tuple(#[educe(Clone(method = Self::copy_value))] T),
}

impl<T: Clone> AssociatedEnum<T> {
    fn copy_value(value: &T) -> T {
        value.clone()
    }
}

#[test]
fn associated_method_with_bound() {
    let mut value = Associated {
        value: 1
    };
    value.clone_from(&Associated {
        value: 7
    });
    assert_eq!(7, value.clone().value);

    let mut value = AssociatedEnum::Tuple(1);
    value.clone_from(&AssociatedEnum::Tuple(7));
    let AssociatedEnum::Tuple(value) = value.clone();
    assert_eq!(7, value);
}
