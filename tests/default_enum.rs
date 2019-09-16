#![cfg(feature = "Default")]
#![no_std]

extern crate alloc;

#[macro_use]
extern crate educe;

use alloc::string::String;

#[test]
#[allow(irrefutable_let_patterns, dead_code)]
fn basic() {
    #[derive(Educe)]
    #[educe(Default)]
    enum Enum {
        Unit,
    }

    assert!(if let Enum::Unit = Enum::default() {
        true
    } else {
        false
    });

    #[derive(Educe)]
    #[educe(Default)]
    enum Enum2 {
        #[educe(Default)]
        Unit,
    }

    assert!(if let Enum2::Unit = Enum2::default() {
        true
    } else {
        false
    });

    #[derive(Educe)]
    #[educe(Default)]
    enum Enum3 {
        Unit,
        #[educe(Default)]
        Unit2,
    }

    assert!(if let Enum3::Unit2 = Enum3::default() {
        true
    } else {
        false
    });

    #[derive(Educe)]
    #[educe(Default)]
    enum Enum4 {
        Unit,
        #[educe(Default)]
        Struct {
            f1: u8,
        },
        Tuple(u8),
    }

    assert!(if let Enum4::Struct {
        f1: 0,
    } = Enum4::default()
    {
        true
    } else {
        false
    });
}

#[test]
#[allow(dead_code)]
fn type_default_1() {
    #[derive(Educe)]
    #[educe(Default(expression = "Enum::Struct { f1: 1 }"))]
    enum Enum {
        Unit,
        Struct {
            f1: u8,
        },
        Tuple(u8),
    }

    let e = Enum::default();

    if let Enum::Struct {
        f1,
    } = e
    {
        assert_eq!(1, f1);
    } else {
        panic!();
    }
}

#[test]
#[allow(dead_code)]
fn type_default_2() {
    #[derive(Educe)]
    #[educe(Default(expression("Enum::Struct { f1: 1 }")))]
    enum Enum {
        Unit,
        Struct {
            f1: u8,
        },
        Tuple(u8),
    }

    let e = Enum::default();

    if let Enum::Struct {
        f1,
    } = e
    {
        assert_eq!(1, f1);
    } else {
        panic!();
    }
}

#[test]
#[allow(irrefutable_let_patterns, dead_code)]
fn field_default_1() {
    #[derive(Educe)]
    #[educe(Default)]
    enum Enum1 {
        Struct {
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
        },
    }

    #[derive(Educe)]
    #[educe(Default)]
    enum Enum2 {
        Unit,
        #[educe(Default)]
        Tuple(
            #[educe(Default = 1)] u8,
            #[educe(Default = 11111111111111111111111111111)] i128,
            #[educe(Default = 1.1)] f64,
            #[educe(Default = true)] bool,
            #[educe(Default = "Hi")] &'static str,
            #[educe(Default = "Hello")] String,
            #[educe(Default = 'M')] char,
        ),
    }

    let s = Enum1::default();
    let t = Enum2::default();

    if let Enum1::Struct {
        f1,
        f2,
        f3,
        f4,
        f5,
        f6,
        f7,
    } = s
    {
        assert_eq!(1, f1);
        assert_eq!(11111111111111111111111111111, f2);
        assert_eq!(1.1, f3);
        assert_eq!(true, f4);
        assert_eq!("Hi", f5);
        assert_eq!("Hello", f6);
        assert_eq!('M', f7);
    } else {
        panic!();
    }

    if let Enum2::Tuple(f1, f2, f3, f4, f5, f6, f7) = t {
        assert_eq!(1, f1);
        assert_eq!(11111111111111111111111111111, f2);
        assert_eq!(1.1, f3);
        assert_eq!(true, f4);
        assert_eq!("Hi", f5);
        assert_eq!("Hello", f6);
        assert_eq!('M', f7);
    } else {
        panic!();
    }
}

#[test]
#[allow(irrefutable_let_patterns, dead_code)]
fn field_default_2() {
    #[derive(Educe)]
    #[educe(Default)]
    enum Enum1 {
        Struct {
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
        },
    }

    #[derive(Educe)]
    #[educe(Default)]
    enum Enum2 {
        Unit,
        #[educe(Default)]
        Tuple(
            #[educe(Default = 1)] u8,
            #[educe(Default = 11111111111111111111111111111)] i128,
            #[educe(Default = 1.1)] f64,
            #[educe(Default = true)] bool,
            #[educe(Default = "Hi")] &'static str,
            #[educe(Default = "Hello")] String,
            #[educe(Default = 'M')] char,
        ),
    }

    let s = Enum1::default();
    let t = Enum2::default();

    if let Enum1::Struct {
        f1,
        f2,
        f3,
        f4,
        f5,
        f6,
        f7,
    } = s
    {
        assert_eq!(1, f1);
        assert_eq!(11111111111111111111111111111, f2);
        assert_eq!(1.1, f3);
        assert_eq!(true, f4);
        assert_eq!("Hi", f5);
        assert_eq!("Hello", f6);
        assert_eq!('M', f7);
    } else {
        panic!();
    }

    if let Enum2::Tuple(f1, f2, f3, f4, f5, f6, f7) = t {
        assert_eq!(1, f1);
        assert_eq!(11111111111111111111111111111, f2);
        assert_eq!(1.1, f3);
        assert_eq!(true, f4);
        assert_eq!("Hi", f5);
        assert_eq!("Hello", f6);
        assert_eq!('M', f7);
    } else {
        panic!();
    }
}

#[test]
#[allow(irrefutable_let_patterns, dead_code)]
fn field_default_3() {
    #[derive(Educe)]
    #[educe(Default)]
    enum Enum1 {
        Struct {
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
        },
    }

    #[derive(Educe)]
    #[educe(Default)]
    enum Enum2 {
        Unit,
        #[educe(Default)]
        Tuple(
            #[educe(Default(expression = "0 + 1"))] u8,
            #[educe(Default(expression = "-11111111111111111111111111111 * -1"))] i128,
            #[educe(Default(expression = "1.0 + 0.1"))] f64,
            #[educe(Default(expression = "!false"))] bool,
            #[educe(Default(expression = "\"Hi\""))] &'static str,
            #[educe(Default(expression = "String::from(\"Hello\")"))] String,
            #[educe(Default(expression = "'M'"))] char,
        ),
    }

    let s = Enum1::default();
    let t = Enum2::default();

    if let Enum1::Struct {
        f1,
        f2,
        f3,
        f4,
        f5,
        f6,
        f7,
    } = s
    {
        assert_eq!(1, f1);
        assert_eq!(11111111111111111111111111111, f2);
        assert_eq!(1.1, f3);
        assert_eq!(true, f4);
        assert_eq!("Hi", f5);
        assert_eq!("Hello", f6);
        assert_eq!('M', f7);
    } else {
        panic!();
    }

    if let Enum2::Tuple(f1, f2, f3, f4, f5, f6, f7) = t {
        assert_eq!(1, f1);
        assert_eq!(11111111111111111111111111111, f2);
        assert_eq!(1.1, f3);
        assert_eq!(true, f4);
        assert_eq!("Hi", f5);
        assert_eq!("Hello", f6);
        assert_eq!('M', f7);
    } else {
        panic!();
    }
}

#[test]
#[allow(irrefutable_let_patterns, dead_code)]
fn field_default_4() {
    #[derive(Educe)]
    #[educe(Default)]
    enum Enum1 {
        Struct {
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
        },
    }

    #[derive(Educe)]
    #[educe(Default)]
    enum Enum2 {
        Unit,
        #[educe(Default)]
        Tuple(
            #[educe(Default(expression("0 + 1")))] u8,
            #[educe(Default(expression("-11111111111111111111111111111 * -1")))] i128,
            #[educe(Default(expression("1.0 + 0.1")))] f64,
            #[educe(Default(expression("!false")))] bool,
            #[educe(Default(expression("\"Hi\"")))] &'static str,
            #[educe(Default(expression("String::from(\"Hello\")")))] String,
            #[educe(Default(expression("'M'")))] char,
        ),
    }

    let s = Enum1::default();
    let t = Enum2::default();

    if let Enum1::Struct {
        f1,
        f2,
        f3,
        f4,
        f5,
        f6,
        f7,
    } = s
    {
        assert_eq!(1, f1);
        assert_eq!(11111111111111111111111111111, f2);
        assert_eq!(1.1, f3);
        assert_eq!(true, f4);
        assert_eq!("Hi", f5);
        assert_eq!("Hello", f6);
        assert_eq!('M', f7);
    } else {
        panic!();
    }

    if let Enum2::Tuple(f1, f2, f3, f4, f5, f6, f7) = t {
        assert_eq!(1, f1);
        assert_eq!(11111111111111111111111111111, f2);
        assert_eq!(1.1, f3);
        assert_eq!(true, f4);
        assert_eq!("Hi", f5);
        assert_eq!("Hello", f6);
        assert_eq!('M', f7);
    } else {
        panic!();
    }
}

#[test]
#[allow(dead_code)]
fn bound_1() {
    #[derive(Educe)]
    #[educe(Default(bound))]
    enum Enum<T> {
        Unit,
        #[educe(Default)]
        Struct {
            f1: T,
        },
        Tuple(T),
    }

    assert!(if let Enum::Struct {
        f1: 0,
    } = Enum::default()
    {
        true
    } else {
        false
    });
}

#[test]
#[allow(dead_code)]
fn bound_2() {
    #[derive(Educe)]
    #[educe(Default(bound = "T: core::default::Default"))]
    enum Enum<T> {
        Unit,
        #[educe(Default)]
        Struct {
            f1: T,
        },
        Tuple(T),
    }

    assert!(if let Enum::Struct {
        f1: 0,
    } = Enum::default()
    {
        true
    } else {
        false
    });
}

#[test]
#[allow(dead_code)]
fn bound_3() {
    #[derive(Educe)]
    #[educe(Default(bound("T: core::default::Default")))]
    enum Enum<T> {
        Unit,
        #[educe(Default)]
        Struct {
            f1: T,
        },
        Tuple(T),
    }

    assert!(if let Enum::Struct {
        f1: 0,
    } = Enum::default()
    {
        true
    } else {
        false
    });
}

#[test]
#[allow(irrefutable_let_patterns, dead_code)]
fn new() {
    #[derive(Educe)]
    #[educe(Default(new))]
    enum Enum {
        Unit,
        #[educe(Default)]
        Struct {
            f1: u8,
        },
        Tuple(u8),
    }

    assert!(if let Enum::Struct {
        f1: 0,
    } = Enum::new()
    {
        true
    } else {
        false
    });
}
