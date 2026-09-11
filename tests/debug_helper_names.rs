#![cfg(feature = "Debug")]
#![no_std]
#![allow(non_camel_case_types, dead_code)]

#[macro_use]
extern crate alloc;

use core::fmt::{self, Formatter};

use educe::Educe;

#[test]
fn field_types() {
    struct r#Educe__DebugField;
    struct _Educe__DebugField;
    struct Educe__RawString;
    struct _Educe__RawString;

    fn render<T>(_: &T, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("field")
    }

    #[derive(Educe)]
    #[educe(Debug)]
    struct Tuple(
        #[educe(Debug(method = render))] Educe__DebugField,
        #[educe(Debug(method = render))] _Educe__DebugField,
    );

    #[derive(Educe)]
    #[educe(Debug(name = false))]
    struct Map {
        #[educe(Debug(method = render))]
        first:  Educe__RawString,
        #[educe(Debug(method = render))]
        second: _Educe__RawString,
    }

    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        #[educe(Debug(name = false))]
        Named {
            #[educe(Debug(method = render))]
            value: Educe__RawString,
        },
        Tuple(#[educe(Debug(method = render))] Educe__DebugField),
    }

    assert_eq!(
        "Tuple(field, field)",
        format!("{:?}", Tuple(Educe__DebugField, _Educe__DebugField))
    );
    assert_eq!(
        "{first: field, second: field}",
        format!("{:?}", Map {
            first: Educe__RawString, second: _Educe__RawString
        })
    );
    assert_eq!(
        "{value: field}",
        format!("{:?}", Enum::Named {
            value: Educe__RawString
        })
    );
    assert_eq!("Tuple(field)", format!("{:?}", Enum::Tuple(Educe__DebugField)));
}

#[allow(non_snake_case)]
#[test]
fn string_method_path() {
    mod Educe__DebugField {
        pub fn render(value: &u8, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{value}")
        }
    }

    #[derive(Educe)]
    #[educe(Debug)]
    struct Struct(#[educe(Debug(method = "Educe__DebugField::render"))] u8);

    assert_eq!("Struct(7)", format!("{:?}", Struct(7)));
}
