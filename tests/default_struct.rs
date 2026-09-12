#![cfg(feature = "Default")]
#![no_std]
#![allow(clippy::default_constructed_unit_structs)]

extern crate alloc;

use alloc::string::String;

use assert_eq_float::assert_eq_float;
use educe::Educe;

#[test]
fn generic_container_name() {
    #[derive(Educe)]
    #[educe(Default)]
    struct Struct<Option>(Option);

    assert_eq!(0, Struct::<u8>::default().0);
}

#[test]
fn empty() {
    #[derive(Educe)]
    #[educe(Default)]
    struct Struct {}

    #[derive(Educe)]
    #[educe(Default)]
    struct Tuple();

    assert!(matches!(Struct::default(), Struct {}));
    assert!(matches!(Tuple::default(), Tuple()));
}

#[test]
fn basic() {
    #[derive(Educe)]
    #[educe(Default)]
    struct Unit;

    #[derive(Educe)]
    #[educe(Default)]
    struct Struct {
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Default)]
    struct Tuple(u8);

    assert!(matches!(Unit::default(), Unit));

    assert_eq!(0, Struct::default().f1);
    assert_eq!(0, Tuple::default().0);
}

#[test]
fn type_expression_1() {
    #[derive(Educe)]
    #[educe(Default(expression = Unit))]
    struct Unit;

    #[derive(Educe)]
    #[educe(Default(expression = Struct { f1: 1 }))]
    struct Struct {
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Default(expression = Tuple(1)))]
    struct Tuple(u8);

    assert!(matches!(Unit::default(), Unit));

    assert_eq!(1, Struct::default().f1);
    assert_eq!(1, Tuple::default().0);
}

#[test]
fn type_expression_2() {
    #[derive(Educe)]
    #[educe(Default(expression(Unit)))]
    struct Unit;

    #[derive(Educe)]
    #[educe(Default(expression(Struct { f1: 1 })))]
    struct Struct {
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Default(expression(Tuple(1))))]
    struct Tuple(u8);

    assert!(matches!(Unit::default(), Unit));

    assert_eq!(1, Struct::default().f1);
    assert_eq!(1, Tuple::default().0);
}

#[test]
fn field_expression_1() {
    #[derive(Educe)]
    #[educe(Default)]
    struct Struct {
        #[educe(Default = 1)]
        f1: u8,
        #[educe(Default = 11111111111111111111111111111)]
        f2: i128,
        #[educe(Default = 1.1)]
        f3: f64,
        #[educe(Default = true)]
        f4: bool,
        #[educe(Default = "Hi")]
        f5: &'static str,
        #[educe(Default = "Hello")]
        f6: String,
        #[educe(Default = 'M')]
        f7: char,
    }

    #[derive(Educe)]
    #[educe(Default)]
    struct Tuple(
        #[educe(Default = 1)] u8,
        #[educe(Default = 11111111111111111111111111111)] i128,
        #[educe(Default = 1.1)] f64,
        #[educe(Default = true)] bool,
        #[educe(Default = "Hi")] &'static str,
        #[educe(Default = "Hello")] String,
        #[educe(Default = 'M')] char,
    );

    let s = Struct::default();
    let t = Tuple::default();

    assert_eq!(1, s.f1);
    assert_eq!(11111111111111111111111111111, s.f2);
    assert_eq_float!(1.1, s.f3);
    assert!(s.f4);
    assert_eq!("Hi", s.f5);
    assert_eq!("Hello", s.f6);
    assert_eq!('M', s.f7);

    assert_eq!(1, t.0);
    assert_eq!(11111111111111111111111111111, t.1);
    assert_eq_float!(1.1, t.2);
    assert!(t.3);
    assert_eq!("Hi", t.4);
    assert_eq!("Hello", t.5);
    assert_eq!('M', t.6);
}

#[test]
fn field_expression_2() {
    #[derive(Educe)]
    #[educe(Default)]
    struct Struct {
        #[educe(Default(expression = 1))]
        f1: u8,
        #[educe(Default(expression = 11111111111111111111111111111))]
        f2: i128,
        #[educe(Default(expression = 1.1))]
        f3: f64,
        #[educe(Default(expression = true))]
        f4: bool,
        #[educe(Default(expression = "Hi"))]
        f5: &'static str,
        #[educe(Default(expression = "Hello"))]
        f6: String,
        #[educe(Default(expression = 'M'))]
        f7: char,
    }

    #[derive(Educe)]
    #[educe(Default)]
    struct Tuple(
        #[educe(Default(expression = 1))] u8,
        #[educe(Default(expression = 11111111111111111111111111111))] i128,
        #[educe(Default(expression = 1.1))] f64,
        #[educe(Default(expression = true))] bool,
        #[educe(Default(expression = "Hi"))] &'static str,
        #[educe(Default(expression = "Hello"))] String,
        #[educe(Default(expression = 'M'))] char,
    );

    let s = Struct::default();
    let t = Tuple::default();

    assert_eq!(1, s.f1);
    assert_eq!(11111111111111111111111111111, s.f2);
    assert_eq_float!(1.1, s.f3);
    assert!(s.f4);
    assert_eq!("Hi", s.f5);
    assert_eq!("Hello", s.f6);
    assert_eq!('M', s.f7);

    assert_eq!(1, t.0);
    assert_eq!(11111111111111111111111111111, t.1);
    assert_eq_float!(1.1, t.2);
    assert!(t.3);
    assert_eq!("Hi", t.4);
    assert_eq!("Hello", t.5);
    assert_eq!('M', t.6);
}

#[test]
fn field_expression_3() {
    #[derive(Educe)]
    #[educe(Default)]
    struct Struct {
        #[educe(Default(expression(1)))]
        f1: u8,
        #[educe(Default(expression(11111111111111111111111111111)))]
        f2: i128,
        #[educe(Default(expression(1.1)))]
        f3: f64,
        #[educe(Default(expression(true)))]
        f4: bool,
        #[educe(Default(expression("Hi")))]
        f5: &'static str,
        #[educe(Default(expression("Hello")))]
        f6: String,
        #[educe(Default(expression('M')))]
        f7: char,
    }

    #[derive(Educe)]
    #[educe(Default)]
    struct Tuple(
        #[educe(Default(expression(1)))] u8,
        #[educe(Default(expression(11111111111111111111111111111)))] i128,
        #[educe(Default(expression(1.1)))] f64,
        #[educe(Default(expression(true)))] bool,
        #[educe(Default(expression("Hi")))] &'static str,
        #[educe(Default(expression("Hello")))] String,
        #[educe(Default(expression('M')))] char,
    );

    let s = Struct::default();
    let t = Tuple::default();

    assert_eq!(1, s.f1);
    assert_eq!(11111111111111111111111111111, s.f2);
    assert_eq_float!(1.1, s.f3);
    assert!(s.f4);
    assert_eq!("Hi", s.f5);
    assert_eq!("Hello", s.f6);
    assert_eq!('M', s.f7);

    assert_eq!(1, t.0);
    assert_eq!(11111111111111111111111111111, t.1);
    assert_eq_float!(1.1, t.2);
    assert!(t.3);
    assert_eq!("Hi", t.4);
    assert_eq!("Hello", t.5);
    assert_eq!('M', t.6);
}

#[allow(clippy::identity_op, clippy::nonminimal_bool, clippy::useless_conversion)]
#[test]
fn field_expression_4() {
    #[derive(Educe)]
    #[educe(Default)]
    struct Struct {
        #[educe(Default(expression = 0 + 1))]
        f1: u8,
        #[educe(Default(expression = -11111111111111111111111111111 * -1))]
        f2: i128,
        #[educe(Default(expression = 1.0 + 0.1))]
        f3: f64,
        #[educe(Default(expression = !false))]
        f4: bool,
        #[educe(Default(expression = "Hi".into()))]
        f5: &'static str,
        #[educe(Default(expression = String::from("Hello")))]
        f6: String,
        #[educe(Default(expression = 'M'.into()))]
        f7: char,
    }

    #[derive(Educe)]
    #[educe(Default)]
    struct Tuple(
        #[educe(Default(expression = 0 + 1))] u8,
        #[educe(Default(expression = -11111111111111111111111111111 * -1))] i128,
        #[educe(Default(expression = 1.0 + 0.1))] f64,
        #[educe(Default(expression = !false))] bool,
        #[educe(Default(expression = "Hi".into()))] &'static str,
        #[educe(Default(expression = String::from("Hello")))] String,
        #[educe(Default(expression = 'M'.into()))] char,
    );

    let s = Struct::default();
    let t = Tuple::default();

    assert_eq!(1, s.f1);
    assert_eq!(11111111111111111111111111111, s.f2);
    assert_eq_float!(1.1, s.f3);
    assert!(s.f4);
    assert_eq!("Hi", s.f5);
    assert_eq!("Hello", s.f6);
    assert_eq!('M', s.f7);

    assert_eq!(1, t.0);
    assert_eq!(11111111111111111111111111111, t.1);
    assert_eq_float!(1.1, t.2);
    assert!(t.3);
    assert_eq!("Hi", t.4);
    assert_eq!("Hello", t.5);
    assert_eq!('M', t.6);
}

#[test]
fn bound_1() {
    #[derive(Educe)]
    #[educe(Default)]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(Default)]
    struct Tuple<T>(T);

    assert_eq!(0, Struct::default().f1);
    assert_eq!(0, Tuple::default().0);
}

#[test]
fn bound_2() {
    #[derive(Educe)]
    #[educe(Default(bound = "T: core::default::Default"))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(Default(bound = "T: core::default::Default"))]
    struct Tuple<T>(T);

    assert_eq!(0, Struct::default().f1);
    assert_eq!(0, Tuple::default().0);
}

#[test]
fn bound_3() {
    #[derive(Educe)]
    #[educe(Default(bound(T: core::default::Default)))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(Default(bound(T: core::default::Default)))]
    struct Tuple<T>(T);

    assert_eq!(0, Struct::default().f1);
    assert_eq!(0, Tuple::default().0);
}

#[test]
fn new() {
    #[derive(Educe)]
    #[educe(Default(new))]
    struct Unit;

    #[derive(Educe)]
    #[educe(Default(new))]
    struct Struct {
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Default(new))]
    struct Tuple(u8);

    assert!(matches!(Unit::new(), Unit));

    assert_eq!(0, Struct::new().f1);
    assert_eq!(0, Tuple::new().0);
}

#[test]
fn bound_4() {
    use alloc::{collections::BTreeMap, vec::Vec};

    struct NotDefault;

    // These std containers default to an empty value, so their type arguments never receive a `Default` bound.
    #[derive(Educe)]
    #[educe(Default)]
    struct Struct<T> {
        f1: Vec<T>,
        f2: Option<T>,
        f3: BTreeMap<T, T>,
    }

    let s: Struct<NotDefault> = Struct::default();

    assert!(s.f1.is_empty());
    assert!(s.f2.is_none());
    assert!(s.f3.is_empty());
}

#[test]
fn const_generic_bound() {
    #[derive(Educe)]
    #[educe(Default)]
    struct Array<const N: usize>([u8; N]);

    struct Buffer<const N: usize>([u8; N]);
    impl Default for Buffer<4> {
        fn default() -> Self {
            Self([0; 4])
        }
    }

    #[derive(Educe)]
    #[educe(Default)]
    struct Wrapper<const N: usize>(Buffer<N>);

    assert_eq!([0; 4], Array::<4>::default().0);
    assert_eq!([0; 4], Wrapper::<4>::default().0.0);
}

#[test]
fn explicit_bounds() {
    // `bound(*)` adds a predicate for every type parameter, and `bound(false)` adds none, so the impl relies on what the type itself declares.
    #[derive(Educe)]
    #[educe(Default(bound(*)))]
    struct All<T>(T, #[allow(dead_code)] u8);

    #[derive(Educe)]
    #[educe(Default(bound(false)))]
    struct Disabled<T: Default>(T);

    assert_eq!(0, All::<u8>::default().0);
    assert_eq!(0, Disabled::<u8>::default().0);
}

#[test]
fn qualified_primitive_literals() {
    extern crate std;

    #[derive(Educe)]
    #[educe(Default)]
    struct Struct {
        #[educe(Default = 7)]
        integer:     ::core::primitive::u64,
        #[educe(Default = 1.5)]
        float:       core::primitive::f32,
        #[educe(Default = 9)]
        std_integer: std::primitive::u64,
        #[educe(Default = 2.5)]
        std_float:   std::primitive::f32,
    }

    let value = Struct::default();
    assert_eq!(7, value.integer);
    assert_eq_float!(1.5, value.float);
    assert_eq!(9, value.std_integer);
    assert_eq_float!(2.5, value.std_float);
}
