#![cfg(feature = "Default")]
#![no_std]
#![allow(clippy::default_constructed_unit_structs)]

use assert_eq_float::assert_eq_float;
use educe::Educe;

#[allow(dead_code)]
#[test]
fn basic() {
    #[derive(Educe)]
    #[educe(Default)]
    union Union {
        f1: u8,
    }

    assert_eq!(0, unsafe { Union::default().f1 });

    #[derive(Educe)]
    #[educe(Default)]
    union Union2 {
        #[educe(Default)]
        f1: u8,
    }

    assert_eq!(0, unsafe { Union2::default().f1 });

    #[derive(Educe)]
    #[educe(Default)]
    union Union3 {
        f1: u8,
        #[educe(Default)]
        f2: f64,
    }

    assert_eq_float!(0.0, unsafe { Union3::default().f2 });
}

#[allow(dead_code)]
#[test]
fn type_expression_1() {
    #[derive(Educe)]
    #[educe(Default(expression = Union { f1: 1 }))]
    union Union {
        f1: u8,
        f2: f64,
    }

    assert_eq!(1, unsafe { Union::default().f1 });
}

#[allow(dead_code)]
#[test]
fn type_default_2() {
    #[derive(Educe)]
    #[educe(Default(expression(Union { f1: 1 })))]
    union Union {
        f1: u8,
        f2: f64,
    }

    assert_eq!(1, unsafe { Union::default().f1 });
}

#[allow(dead_code)]
#[test]
fn field_expression_1() {
    #[derive(Educe)]
    #[educe(Default)]
    union Union {
        #[educe(Default = 1)]
        f1: u8,
        f2: i128,
        f3: f64,
        f4: bool,
        f5: &'static str,
        f6: char,
    }

    assert_eq!(1, unsafe { Union::default().f1 });

    #[derive(Educe)]
    #[educe(Default)]
    union Union2 {
        f1: u8,
        #[educe(Default = 11111111111111111111111111111)]
        f2: i128,
        f3: f64,
        f4: bool,
        f5: &'static str,
        f6: char,
    }

    assert_eq!(11111111111111111111111111111, unsafe { Union2::default().f2 });

    #[derive(Educe)]
    #[educe(Default)]
    union Union3 {
        f1: u8,
        f2: i128,
        #[educe(Default = 1.1)]
        f3: f64,
        f4: bool,
        f5: &'static str,
        f6: char,
    }

    assert_eq_float!(1.1, unsafe { Union3::default().f3 });

    #[derive(Educe)]
    #[educe(Default)]
    union Union4 {
        f1: u8,
        f2: i128,
        f3: f64,
        #[educe(Default = true)]
        f4: bool,
        f5: &'static str,
        f6: char,
    }

    assert!(unsafe { Union4::default().f4 });

    #[derive(Educe)]
    #[educe(Default)]
    union Union5 {
        f1: u8,
        f2: i128,
        f3: f64,
        f4: bool,
        #[educe(Default = "Hi")]
        f5: &'static str,
        f6: char,
    }

    assert_eq!("Hi", unsafe { Union5::default().f5 });

    #[derive(Educe)]
    #[educe(Default)]
    union Union6 {
        f1: u8,
        f2: i128,
        f3: f64,
        f4: bool,
        f5: &'static str,
        #[educe(Default = 'M')]
        f6: char,
    }

    assert_eq!('M', unsafe { Union6::default().f6 });
}

#[allow(dead_code)]
#[test]
fn field_expression_2() {
    #[derive(Educe)]
    #[educe(Default)]
    union Union {
        #[educe(Default(expression = 1))]
        f1: u8,
        f2: i128,
        f3: f64,
        f4: bool,
        f5: &'static str,
        f6: char,
    }

    assert_eq!(1, unsafe { Union::default().f1 });

    #[derive(Educe)]
    #[educe(Default)]
    union Union2 {
        f1: u8,
        #[educe(Default(expression = 11111111111111111111111111111))]
        f2: i128,
        f3: f64,
        f4: bool,
        f5: &'static str,
        f6: char,
    }

    assert_eq!(11111111111111111111111111111, unsafe { Union2::default().f2 });

    #[derive(Educe)]
    #[educe(Default)]
    union Union3 {
        f1: u8,
        f2: i128,
        #[educe(Default(expression = 1.1))]
        f3: f64,
        f4: bool,
        f5: &'static str,
        f6: char,
    }

    assert_eq_float!(1.1, unsafe { Union3::default().f3 });

    #[derive(Educe)]
    #[educe(Default)]
    union Union4 {
        f1: u8,
        f2: i128,
        f3: f64,
        #[educe(Default(expression = true))]
        f4: bool,
        f5: &'static str,
        f6: char,
    }

    assert!(unsafe { Union4::default().f4 });

    #[derive(Educe)]
    #[educe(Default)]
    union Union5 {
        f1: u8,
        f2: i128,
        f3: f64,
        f4: bool,
        #[educe(Default(expression = "Hi"))]
        f5: &'static str,
        f6: char,
    }

    assert_eq!("Hi", unsafe { Union5::default().f5 });

    #[derive(Educe)]
    #[educe(Default)]
    union Union6 {
        f1: u8,
        f2: i128,
        f3: f64,
        f4: bool,
        f5: &'static str,
        #[educe(Default(expression = 'M'))]
        f6: char,
    }

    assert_eq!('M', unsafe { Union6::default().f6 });
}

#[allow(dead_code)]
#[test]
fn field_expression_3() {
    #[derive(Educe)]
    #[educe(Default)]
    union Union {
        #[educe(Default(expression(1)))]
        f1: u8,
        f2: i128,
        f3: f64,
        f4: bool,
        f5: &'static str,
        f6: char,
    }

    assert_eq!(1, unsafe { Union::default().f1 });

    #[derive(Educe)]
    #[educe(Default)]
    union Union2 {
        f1: u8,
        #[educe(Default(expression(11111111111111111111111111111)))]
        f2: i128,
        f3: f64,
        f4: bool,
        f5: &'static str,
        f6: char,
    }

    assert_eq!(11111111111111111111111111111, unsafe { Union2::default().f2 });

    #[derive(Educe)]
    #[educe(Default)]
    union Union3 {
        f1: u8,
        f2: i128,
        #[educe(Default(expression(1.1)))]
        f3: f64,
        f4: bool,
        f5: &'static str,
        f6: char,
    }

    assert_eq_float!(1.1, unsafe { Union3::default().f3 });

    #[derive(Educe)]
    #[educe(Default)]
    union Union4 {
        f1: u8,
        f2: i128,
        f3: f64,
        #[educe(Default(expression(true)))]
        f4: bool,
        f5: &'static str,
        f6: char,
    }

    assert!(unsafe { Union4::default().f4 });

    #[derive(Educe)]
    #[educe(Default)]
    union Union5 {
        f1: u8,
        f2: i128,
        f3: f64,
        f4: bool,
        #[educe(Default(expression("Hi")))]
        f5: &'static str,
        f6: char,
    }

    assert_eq!("Hi", unsafe { Union5::default().f5 });

    #[derive(Educe)]
    #[educe(Default)]
    union Union6 {
        f1: u8,
        f2: i128,
        f3: f64,
        f4: bool,
        f5: &'static str,
        #[educe(Default(expression('M')))]
        f6: char,
    }

    assert_eq!('M', unsafe { Union6::default().f6 });
}

#[allow(dead_code, clippy::identity_op, clippy::nonminimal_bool, clippy::useless_conversion)]
#[test]
fn field_expression_4() {
    #[derive(Educe)]
    #[educe(Default)]
    union Union {
        #[educe(Default(expression = 0 + 1))]
        f1: u8,
        f2: i128,
        f3: f64,
        f4: bool,
        f5: &'static str,
        f6: char,
    }

    assert_eq!(1, unsafe { Union::default().f1 });

    #[derive(Educe)]
    #[educe(Default)]
    union Union2 {
        f1: u8,
        #[educe(Default(expression = -11111111111111111111111111111 * -1))]
        f2: i128,
        f3: f64,
        f4: bool,
        f5: &'static str,
        f6: char,
    }

    assert_eq!(11111111111111111111111111111, unsafe { Union2::default().f2 });

    #[derive(Educe)]
    #[educe(Default)]
    union Union3 {
        f1: u8,
        f2: i128,
        #[educe(Default(expression = 1.0 + 0.1))]
        f3: f64,
        f4: bool,
        f5: &'static str,
        f6: char,
    }

    assert_eq_float!(1.1, unsafe { Union3::default().f3 });

    #[derive(Educe)]
    #[educe(Default)]
    union Union4 {
        f1: u8,
        f2: i128,
        f3: f64,
        #[educe(Default(expression = !false))]
        f4: bool,
        f5: &'static str,
        f6: char,
    }

    assert!(unsafe { Union4::default().f4 });

    #[derive(Educe)]
    #[educe(Default)]
    union Union5 {
        f1: u8,
        f2: i128,
        f3: f64,
        f4: bool,
        #[educe(Default(expression = "Hi".into()))]
        f5: &'static str,
        f6: char,
    }

    assert_eq!("Hi", unsafe { Union5::default().f5 });

    #[derive(Educe)]
    #[educe(Default)]
    union Union6 {
        f1: u8,
        f2: i128,
        f3: f64,
        f4: bool,
        f5: &'static str,
        #[educe(Default(expression = 'M'.into()))]
        f6: char,
    }

    assert_eq!('M', unsafe { Union6::default().f6 });
}

#[allow(dead_code)]
#[test]
fn bound_1() {
    #[derive(Educe)]
    #[educe(Default)]
    union Union<T: Copy> {
        f1: T,
    }

    assert_eq!(0, unsafe { Union::default().f1 });

    #[derive(Educe)]
    #[educe(Default)]
    union Union2<T: Copy, K: Copy> {
        f1: T,
        #[educe(Default)]
        f2: K,
    }

    assert_eq_float!(0.0, unsafe { Union2::<u8, f64>::default().f2 });
}

#[allow(dead_code)]
#[test]
fn bound_2() {
    #[derive(Educe)]
    #[educe(Default(bound = "T: core::default::Default"))]
    union Union<T: Copy> {
        f1: T,
    }

    assert_eq!(0, unsafe { Union::default().f1 });

    #[derive(Educe)]
    #[educe(Default(bound = "K: core::default::Default"))]
    union Union2<T: Copy, K: Copy> {
        f1: T,
        #[educe(Default)]
        f2: K,
    }

    assert_eq_float!(0.0, unsafe { Union2::<u8, f64>::default().f2 });
}

#[allow(dead_code)]
#[test]
fn bound_3() {
    #[derive(Educe)]
    #[educe(Default(bound(T: core::default::Default)))]
    union Union<T: Copy> {
        f1: T,
    }

    assert_eq!(0, unsafe { Union::default().f1 });

    #[derive(Educe)]
    #[educe(Default(bound(K: core::default::Default)))]
    union Union2<T: Copy, K: Copy> {
        f1: T,
        #[educe(Default)]
        f2: K,
    }

    assert_eq_float!(0.0, unsafe { Union2::<u8, f64>::default().f2 });
}

#[allow(dead_code)]
#[test]
fn new() {
    #[derive(Educe)]
    #[educe(Default(new))]
    union Union {
        f1: u8,
    }

    assert_eq!(0, unsafe { Union::new().f1 });

    #[derive(Educe)]
    #[educe(Default(new))]
    union Union2 {
        f1: u8,
        #[educe(Default)]
        f2: f64,
    }

    assert_eq_float!(0.0, unsafe { Union2::new().f2 });
}
