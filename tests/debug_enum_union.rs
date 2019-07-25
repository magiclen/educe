#![no_std]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate educe;

#[test]
fn name_1() {
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        A
    };

    assert_eq!("A", format!("{:?}", Enum::A));
}

#[test]
fn name_2() {
    #[derive(Educe)]
    #[educe(Debug = "Hi")]
    enum Enum {
        A
    };

    assert_eq!("Hi::A", format!("{:?}", Enum::A));
}

#[test]
fn name_3() {
    #[derive(Educe)]
    #[educe(Debug("Hi"))]
    enum Enum {
        A
    };

    assert_eq!("Hi::A", format!("{:?}", Enum::A));
}

#[test]
fn name_4() {
    #[derive(Educe)]
    #[educe(Debug(name = "Hi"))]
    enum Enum {
        A
    };

    assert_eq!("Hi::A", format!("{:?}", Enum::A));
}

#[test]
fn name_5() {
    #[derive(Educe)]
    #[educe(Debug(name("Hi")))]
    enum Enum {
        A
    };

    assert_eq!("Hi::A", format!("{:?}", Enum::A));
}

#[test]
fn name_7() {
    #[derive(Educe)]
    #[educe(Debug(name = true))]
    enum Enum {
        A
    };

    assert_eq!("Enum::A", format!("{:?}", Enum::A));
}

#[test]
fn name_8() {
    #[derive(Educe)]
    #[educe(Debug(name(true)))]
    enum Enum {
        A
    };

    assert_eq!("Enum::A", format!("{:?}", Enum::A));
}

#[test]
fn unnamed_variant_1() {
    #[derive(Educe)]
    #[educe(Debug(name = true))]
    enum Enum {
        #[educe(Debug(name = false))]
        A
    };

    assert_eq!("Enum", format!("{:?}", Enum::A));
}

#[test]
fn unnamed_variant_2() {
    #[derive(Educe)]
    #[educe(Debug(name = true))]
    enum Enum {
        #[educe(Debug(name(false)))]
        A
    };

    assert_eq!("Enum", format!("{:?}", Enum::A));
}

#[test]
fn unnamed_variant_3() {
    #[derive(Educe)]
    #[educe(Debug(name = true))]
    enum Enum {
        #[educe(Debug(name = ""))]
        A
    };

    assert_eq!("Enum", format!("{:?}", Enum::A));
}

#[test]
fn unnamed_variant_4() {
    #[derive(Educe)]
    #[educe(Debug(name = true))]
    enum Enum {
        #[educe(Debug(name("")))]
        A
    };

    assert_eq!("Enum", format!("{:?}", Enum::A));
}

#[test]
fn unnamed_variant_5() {
    #[derive(Educe)]
    #[educe(Debug(name = true))]
    enum Enum {
        #[educe(Debug = "")]
        A
    };

    assert_eq!("Enum", format!("{:?}", Enum::A));
}

#[test]
fn unnamed_variant_6() {
    #[derive(Educe)]
    #[educe(Debug(name = true))]
    enum Enum {
        #[educe(Debug(""))]
        A
    };

    assert_eq!("Enum", format!("{:?}", Enum::A));
}

#[test]
fn rename_variant_1() {
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        #[educe(Debug = "Hi")]
        A
    };

    assert_eq!("Hi", format!("{:?}", Enum::A));
}

#[test]
fn rename_variant_2() {
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        #[educe(Debug("Hi"))]
        A
    };

    assert_eq!("Hi", format!("{:?}", Enum::A));
}

#[test]
fn rename_variant_3() {
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        #[educe(Debug(name = "Hi"))]
        A
    };

    assert_eq!("Hi", format!("{:?}", Enum::A));
}

#[test]
fn rename_variant_4() {
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        #[educe(Debug(name("Hi")))]
        A
    };

    assert_eq!("Hi", format!("{:?}", Enum::A));
}

#[test]
fn rename_variant_5() {
    #[derive(Educe)]
    #[educe(Debug(name = true))]
    enum Enum {
        #[educe(Debug(name("Hi")))]
        A
    };

    assert_eq!("Enum::Hi", format!("{:?}", Enum::A));
}

#[test]
fn rename_variant_6() {
    #[derive(Educe)]
    #[educe(Debug(name = true))]
    enum Enum {
        #[educe(Debug(name("::Hi")))]
        A
    };

    assert_eq!("Enum::Hi", format!("{:?}", Enum::A));
}