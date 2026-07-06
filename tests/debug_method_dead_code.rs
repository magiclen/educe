#![cfg(feature = "Debug")]
#![no_std]
// Unlike the other Debug tests, this crate denies dead code so it works as a regression guard. A custom formatting method is referenced only inside the generated `Debug` impl, whose body dead-code analysis skips because the impl is `#[automatically_derived]` and `Debug` carries `#[rustc_trivial_field_reads]`. Without the marker item Educe emits, the method would be reported as never used. The derived types and their fields are `pub` so they stay reachable and are not themselves reported, keeping the check focused on the methods.
#![deny(dead_code)]

#[macro_use]
extern crate alloc;

use core::fmt::{self, Formatter};

use educe::Educe;

fn fmt(_s: &u8, f: &mut Formatter<'_>) -> fmt::Result {
    f.write_str("Hi")
}

// A generic method is exercised as well so the marker item has to reproduce the generics and where clause of the impl in order to compile.
fn fmt_generic<T>(_s: &T, f: &mut Formatter<'_>) -> fmt::Result {
    f.write_str("Hi")
}

#[derive(Educe)]
#[educe(Debug)]
pub struct Struct {
    #[educe(Debug(method = fmt))]
    pub f1: u8,
}

#[derive(Educe)]
#[educe(Debug)]
pub struct Generic<T> {
    #[educe(Debug(method = fmt_generic))]
    pub f1: T,
}

#[derive(Educe)]
#[educe(Debug)]
pub enum Enum<T> {
    Struct {
        #[educe(Debug(method = fmt_generic))]
        f1: T,
    },
    Tuple(#[educe(Debug(method = fmt_generic))] T),
}

#[test]
fn method_is_marked_used() {
    assert_eq!(
        "Struct { f1: Hi }",
        format!("{:?}", Struct {
            f1: 1
        })
    );
    assert_eq!(
        "Generic { f1: Hi }",
        format!("{:?}", Generic {
            f1: 1u8
        })
    );
    assert_eq!(
        "Struct { f1: Hi }",
        format!("{:?}", Enum::Struct {
            f1: 1u8
        })
    );
    assert_eq!("Tuple(Hi)", format!("{:?}", Enum::Tuple(1u8)));
}
