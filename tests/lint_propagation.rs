#![cfg(any(feature = "Debug", feature = "Clone"))]
#![no_std]
#![deny(deprecated, clippy::used_underscore_binding, clippy::ptr_arg)]
// The types in these tests only exist to exercise the derived impls, and `#[automatically_derived]` impls do not count as uses for dead-code analysis.
#![allow(dead_code)]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;

use educe::Educe;

#[cfg(feature = "Debug")]
#[test]
fn automatically_derived() {
    // The generated impl is marked `#[automatically_derived]`, so lints like `clippy::used_underscore_binding` do not fire on the generated code.
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        Struct { f1: u8 },
    }

    let _ = Enum::Struct {
        f1: 1
    };
}

#[test]
fn lint_attribute_propagation() {
    #[deprecated]
    struct Deprecated;

    #[cfg(feature = "Debug")]
    #[allow(deprecated)]
    #[deprecated]
    fn format_deprecated(_: &Deprecated, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Deprecated")
    }

    #[cfg(feature = "Clone")]
    #[allow(deprecated)]
    #[deprecated]
    fn clone_deprecated(_: &Deprecated) -> Deprecated {
        Deprecated
    }

    #[allow(deprecated)]
    #[derive(Educe)]
    #[cfg_attr(feature = "Debug", educe(Debug))]
    #[cfg_attr(feature = "Clone", educe(Clone))]
    struct Struct {
        #[cfg_attr(feature = "Debug", educe(Debug(method = format_deprecated)))]
        #[cfg_attr(feature = "Clone", educe(Clone(method = clone_deprecated)))]
        field: Deprecated,
    }
}

#[cfg(feature = "Debug")]
#[test]
fn debug_method_marker_does_not_trigger_ptr_arg() {
    use core::fmt::{self, Formatter};

    // Writing a method that takes `&Vec<T>` is the user's own choice and legitimately trips `clippy::ptr_arg`, unrelated to Educe; it is allowed here so the test isolates the marker item Educe generates.
    #[allow(clippy::ptr_arg)]
    fn fmt_vec(v: &Vec<u8>, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{v:?}")
    }

    // The marker item Educe emits next to the `Debug` impl mirrors this field's type, so its `&Vec<u8>` parameter is a regression guard: `clippy::ptr_arg` does not currently fire on code coming from an external proc-macro, but the marker also carries its own `#[allow(clippy::all)]` in case that exemption ever narrows.
    #[derive(Educe)]
    #[educe(Debug)]
    struct Struct {
        #[educe(Debug(method = fmt_vec))]
        f1: Vec<u8>,
    }

    let _ = Struct {
        f1: vec![1, 2, 3]
    };
}

#[cfg(feature = "Clone")]
#[test]
fn clone_method_marker_does_not_trigger_ptr_arg() {
    // Writing a method that takes `&Vec<T>` is the user's own choice and legitimately trips `clippy::ptr_arg`, unrelated to Educe; it is allowed here so the test isolates the marker item Educe generates.
    #[allow(clippy::ptr_arg)]
    fn clone_vec(v: &Vec<u8>) -> Vec<u8> {
        v.clone()
    }

    // The marker item Educe emits next to the `Clone` impl mirrors this field's type, so its `&Vec<u8>` parameter is a regression guard: `clippy::ptr_arg` does not currently fire on code coming from an external proc-macro, but the marker also carries its own `#[allow(clippy::all)]` in case that exemption ever narrows.
    #[derive(Educe)]
    #[educe(Clone)]
    struct Struct {
        #[educe(Clone(method = clone_vec))]
        f1: Vec<u8>,
    }

    let _ = Struct {
        f1: vec![1, 2, 3]
    }
    .clone();
}

#[cfg(feature = "Eq")]
#[test]
fn eq_helper_lint_propagation() {
    #[derive(PartialEq, Eq)]
    #[deprecated]
    struct Deprecated;

    #[allow(deprecated)]
    #[derive(PartialEq, Educe)]
    #[educe(Eq)]
    struct Struct(Deprecated);

    fn assert_eq_impl<T: Eq>() {}
    assert_eq_impl::<Struct>();
}
