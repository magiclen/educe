#![cfg(feature = "Default")]
#![no_std]

extern crate alloc;

#[macro_use]
extern crate educe;

use alloc::string::String;

#[test]
#[allow(irrefutable_let_patterns)]
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

    assert!(if let Unit = Unit::default() {
        true
    } else {
        false
    });

    assert_eq!(0, Struct::default().f1);
    assert_eq!(0, Tuple::default().0);
}

#[test]
#[allow(irrefutable_let_patterns)]
fn type_default_1() {
    #[derive(Educe)]
    #[educe(Default(expression = "Unit"))]
    struct Unit;

    #[derive(Educe)]
    #[educe(Default(expression = "Struct { f1: 1 }"))]
    struct Struct {
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Default(expression = "Tuple(1)"))]
    struct Tuple(u8);

    assert!(if let Unit = Unit::default() {
        true
    } else {
        false
    });

    assert_eq!(1, Struct::default().f1);
    assert_eq!(1, Tuple::default().0);
}

#[test]
#[allow(irrefutable_let_patterns)]
fn type_default_2() {
    #[derive(Educe)]
    #[educe(Default(expression("Unit")))]
    struct Unit;

    #[derive(Educe)]
    #[educe(Default(expression("Struct { f1: 1 }")))]
    struct Struct {
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Default(expression("Tuple(1)")))]
    struct Tuple(u8);

    assert!(if let Unit = Unit::default() {
        true
    } else {
        false
    });

    assert_eq!(1, Struct::default().f1);
    assert_eq!(1, Tuple::default().0);
}

#[test]
fn field_default_1() {
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
    assert_eq!(1.1, s.f3);
    assert_eq!(true, s.f4);
    assert_eq!("Hi", s.f5);
    assert_eq!("Hello", s.f6);
    assert_eq!('M', s.f7);

    assert_eq!(1, t.0);
    assert_eq!(11111111111111111111111111111, t.1);
    assert_eq!(1.1, t.2);
    assert_eq!(true, t.3);
    assert_eq!("Hi", t.4);
    assert_eq!("Hello", t.5);
    assert_eq!('M', t.6);
}

#[test]
fn field_default_2() {
    #[derive(Educe)]
    #[educe(Default)]
    struct Struct {
        #[educe(Default(1))]
        f1: u8,
        #[educe(Default(11111111111111111111111111111))]
        f2: i128,
        #[educe(Default(1.1))]
        f3: f64,
        #[educe(Default(true))]
        f4: bool,
        #[educe(Default("Hi"))]
        f5: &'static str,
        #[educe(Default("Hello"))]
        f6: String,
        #[educe(Default('M'))]
        f7: char,
    }

    #[derive(Educe)]
    #[educe(Default)]
    struct Tuple(
        #[educe(Default(1))] u8,
        #[educe(Default(11111111111111111111111111111))] i128,
        #[educe(Default(1.1))] f64,
        #[educe(Default(true))] bool,
        #[educe(Default("Hi"))] &'static str,
        #[educe(Default("Hello"))] String,
        #[educe(Default('M'))] char,
    );

    let s = Struct::default();
    let t = Tuple::default();

    assert_eq!(1, s.f1);
    assert_eq!(11111111111111111111111111111, s.f2);
    assert_eq!(1.1, s.f3);
    assert_eq!(true, s.f4);
    assert_eq!("Hi", s.f5);
    assert_eq!("Hello", s.f6);
    assert_eq!('M', s.f7);

    assert_eq!(1, t.0);
    assert_eq!(11111111111111111111111111111, t.1);
    assert_eq!(1.1, t.2);
    assert_eq!(true, t.3);
    assert_eq!("Hi", t.4);
    assert_eq!("Hello", t.5);
    assert_eq!('M', t.6);
}

#[test]
fn field_default_3() {
    #[derive(Educe)]
    #[educe(Default)]
    struct Struct {
        #[educe(Default(expression = "0 + 1"))]
        f1: u8,
        #[educe(Default(expression = "-11111111111111111111111111111 * -1"))]
        f2: i128,
        #[educe(Default(expression = "1.0 + 0.1"))]
        f3: f64,
        #[educe(Default(expression = "!false"))]
        f4: bool,
        #[educe(Default(expression = "\"Hi\""))]
        f5: &'static str,
        #[educe(Default(expression = "String::from(\"Hello\")"))]
        f6: String,
        #[educe(Default(expression = "'M'"))]
        f7: char,
    }

    #[derive(Educe)]
    #[educe(Default)]
    struct Tuple(
        #[educe(Default(expression = "0 + 1"))] u8,
        #[educe(Default(expression = "-11111111111111111111111111111 * -1"))] i128,
        #[educe(Default(expression = "1.0 + 0.1"))] f64,
        #[educe(Default(expression = "!false"))] bool,
        #[educe(Default(expression = "\"Hi\""))] &'static str,
        #[educe(Default(expression = "String::from(\"Hello\")"))] String,
        #[educe(Default(expression = "'M'"))] char,
    );

    let s = Struct::default();
    let t = Tuple::default();

    assert_eq!(1, s.f1);
    assert_eq!(11111111111111111111111111111, s.f2);
    assert_eq!(1.1, s.f3);
    assert_eq!(true, s.f4);
    assert_eq!("Hi", s.f5);
    assert_eq!("Hello", s.f6);
    assert_eq!('M', s.f7);

    assert_eq!(1, t.0);
    assert_eq!(11111111111111111111111111111, t.1);
    assert_eq!(1.1, t.2);
    assert_eq!(true, t.3);
    assert_eq!("Hi", t.4);
    assert_eq!("Hello", t.5);
    assert_eq!('M', t.6);
}

#[test]
fn field_default_4() {
    #[derive(Educe)]
    #[educe(Default)]
    struct Struct {
        #[educe(Default(expression("0 + 1")))]
        f1: u8,
        #[educe(Default(expression("-11111111111111111111111111111 * -1")))]
        f2: i128,
        #[educe(Default(expression("1.0 + 0.1")))]
        f3: f64,
        #[educe(Default(expression("!false")))]
        f4: bool,
        #[educe(Default(expression("\"Hi\"")))]
        f5: &'static str,
        #[educe(Default(expression("String::from(\"Hello\")")))]
        f6: String,
        #[educe(Default(expression("'M'")))]
        f7: char,
    }

    #[derive(Educe)]
    #[educe(Default)]
    struct Tuple(
        #[educe(Default(expression("0 + 1")))] u8,
        #[educe(Default(expression("-11111111111111111111111111111 * -1")))] i128,
        #[educe(Default(expression("1.0 + 0.1")))] f64,
        #[educe(Default(expression("!false")))] bool,
        #[educe(Default(expression("\"Hi\"")))] &'static str,
        #[educe(Default(expression("String::from(\"Hello\")")))] String,
        #[educe(Default(expression("'M'")))] char,
    );

    let s = Struct::default();
    let t = Tuple::default();

    assert_eq!(1, s.f1);
    assert_eq!(11111111111111111111111111111, s.f2);
    assert_eq!(1.1, s.f3);
    assert_eq!(true, s.f4);
    assert_eq!("Hi", s.f5);
    assert_eq!("Hello", s.f6);
    assert_eq!('M', s.f7);

    assert_eq!(1, t.0);
    assert_eq!(11111111111111111111111111111, t.1);
    assert_eq!(1.1, t.2);
    assert_eq!(true, t.3);
    assert_eq!("Hi", t.4);
    assert_eq!("Hello", t.5);
    assert_eq!('M', t.6);
}

#[test]
fn bound_1() {
    #[derive(Educe)]
    #[educe(Default(bound))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(Default(bound))]
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
    #[educe(Default(bound("T: core::default::Default")))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(Default(bound("T: core::default::Default")))]
    struct Tuple<T>(T);

    assert_eq!(0, Struct::default().f1);
    assert_eq!(0, Tuple::default().0);
}

#[test]
#[allow(irrefutable_let_patterns)]
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

    assert!(if let Unit = Unit::new() { true } else { false });

    assert_eq!(0, Struct::new().f1);
    assert_eq!(0, Tuple::new().0);
}
