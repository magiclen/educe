#![cfg(feature = "Debug")]
#![no_std]
#![deny(clippy::used_underscore_binding)]
// The types in these tests only exist to exercise the derived impls, and `#[automatically_derived]` impls do not count as uses for dead-code analysis.
#![allow(dead_code)]

use educe::Educe;

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
    #[allow(dead_code)]
    #[deprecated]
    struct Deprecated;

    // The `#[allow(deprecated)]` on the type is copied onto the generated impl, so using the deprecated field type inside it does not fail under `#![deny(deprecated)]`.
    #[allow(dead_code)]
    #[allow(deprecated)]
    #[derive(Educe)]
    #[educe(Debug)]
    struct Struct {
        #[educe(Debug(ignore))]
        f1: Deprecated,
    }
}
