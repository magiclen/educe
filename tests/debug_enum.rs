#![cfg(feature = "Debug")]
#![no_std]

#[macro_use]
extern crate alloc;

use educe::Educe;

#[test]
fn empty() {
    #[allow(dead_code)]
    #[derive(Educe)]
    #[educe(Debug = Enum)]
    enum Enum {}

    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum2 {
        Struct {},
        Tuple(),
    }

    assert_eq!("Struct", format!("{:?}", Enum2::Struct {}));
    assert_eq!("Tuple", format!("{:?}", Enum2::Tuple()));
}

#[test]
fn name_1() {
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        Unit,
        Struct { f1: u8 },
        Tuple(u8),
    }

    assert_eq!("Unit", format!("{:?}", Enum::Unit));
    assert_eq!(
        "Struct { f1: 1 }",
        format!("{:?}", Enum::Struct {
            f1: 1
        })
    );
    assert_eq!("Tuple(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
fn name_2() {
    #[derive(Educe)]
    #[educe(Debug = Hi)]
    enum Enum {
        Unit,
        Struct { f1: u8 },
        Tuple(u8),
    }

    assert_eq!("Hi::Unit", format!("{:?}", Enum::Unit));
    assert_eq!(
        "Hi::Struct { f1: 1 }",
        format!("{:?}", Enum::Struct {
            f1: 1
        })
    );
    assert_eq!("Hi::Tuple(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
fn name_3() {
    #[derive(Educe)]
    #[educe(Debug(name = Hi))]
    enum Enum {
        Unit,
        Struct { f1: u8 },
        Tuple(u8),
    }

    assert_eq!("Hi::Unit", format!("{:?}", Enum::Unit));
    assert_eq!(
        "Hi::Struct { f1: 1 }",
        format!("{:?}", Enum::Struct {
            f1: 1
        })
    );
    assert_eq!("Hi::Tuple(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
fn name_4() {
    #[derive(Educe)]
    #[educe(Debug(name(Hi)))]
    enum Enum {
        Unit,
        Struct { f1: u8 },
        Tuple(u8),
    }

    assert_eq!("Hi::Unit", format!("{:?}", Enum::Unit));
    assert_eq!(
        "Hi::Struct { f1: 1 }",
        format!("{:?}", Enum::Struct {
            f1: 1
        })
    );
    assert_eq!("Hi::Tuple(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
fn name_5() {
    #[derive(Educe)]
    #[educe(Debug(name = true))]
    enum Enum {
        Unit,
        Struct { f1: u8 },
        Tuple(u8),
    }

    assert_eq!("Enum::Unit", format!("{:?}", Enum::Unit));
    assert_eq!(
        "Enum::Struct { f1: 1 }",
        format!("{:?}", Enum::Struct {
            f1: 1
        })
    );
    assert_eq!("Enum::Tuple(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
fn name_6() {
    #[derive(Educe)]
    #[educe(Debug(name(true)))]
    enum Enum {
        Unit,
        Struct { f1: u8 },
        Tuple(u8),
    }

    assert_eq!("Enum::Unit", format!("{:?}", Enum::Unit));
    assert_eq!(
        "Enum::Struct { f1: 1 }",
        format!("{:?}", Enum::Struct {
            f1: 1
        })
    );
    assert_eq!("Enum::Tuple(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
fn unnamed_variant_1() {
    #[derive(Educe)]
    #[educe(Debug(name = true))]
    enum Enum {
        #[educe(Debug(name = false))]
        Unit,
        #[educe(Debug(name = false))]
        Struct { f1: u8 },
        #[educe(Debug(name = false))]
        Tuple(u8),
    }

    assert_eq!("Enum", format!("{:?}", Enum::Unit));
    assert_eq!(
        "Enum { f1: 1 }",
        format!("{:?}", Enum::Struct {
            f1: 1
        })
    );
    assert_eq!("Enum(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
fn unnamed_variant_2() {
    #[derive(Educe)]
    #[educe(Debug(name = true))]
    enum Enum {
        #[educe(Debug(name(false)))]
        Unit,
        #[educe(Debug(name(false)))]
        Struct { f1: u8 },
        #[educe(Debug(name(false)))]
        Tuple(u8),
    }

    assert_eq!("Enum", format!("{:?}", Enum::Unit));
    assert_eq!(
        "Enum { f1: 1 }",
        format!("{:?}", Enum::Struct {
            f1: 1
        })
    );
    assert_eq!("Enum(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
fn unnamed_variant_3() {
    #[derive(Educe)]
    #[educe(Debug(name = true))]
    enum Enum {
        #[educe(Debug(name = ""))]
        Unit,
        #[educe(Debug(name = ""))]
        Struct { f1: u8 },
        #[educe(Debug(name = ""))]
        Tuple(u8),
    }

    assert_eq!("Enum", format!("{:?}", Enum::Unit));
    assert_eq!(
        "Enum { f1: 1 }",
        format!("{:?}", Enum::Struct {
            f1: 1
        })
    );
    assert_eq!("Enum(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
fn unnamed_variant_4() {
    #[derive(Educe)]
    #[educe(Debug(name = true))]
    enum Enum {
        #[educe(Debug(name("")))]
        Unit,
        #[educe(Debug(name("")))]
        Struct { f1: u8 },
        #[educe(Debug(name("")))]
        Tuple(u8),
    }

    assert_eq!("Enum", format!("{:?}", Enum::Unit));
    assert_eq!(
        "Enum { f1: 1 }",
        format!("{:?}", Enum::Struct {
            f1: 1
        })
    );
    assert_eq!("Enum(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
fn named_field_1() {
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        #[educe(Debug(named_field = false))]
        Struct { f1: u8 },
        #[educe(Debug(named_field = true))]
        Tuple(u8),
    }

    assert_eq!(
        "Struct(1)",
        format!("{:?}", Enum::Struct {
            f1: 1
        })
    );
    assert_eq!("Tuple { _0: 1 }", format!("{:?}", Enum::Tuple(1)));
}

#[test]
fn named_field_2() {
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        #[educe(Debug(named_field(false)))]
        Struct { f1: u8 },
        #[educe(Debug(named_field(true)))]
        Tuple(u8),
    }

    assert_eq!(
        "Struct(1)",
        format!("{:?}", Enum::Struct {
            f1: 1
        })
    );
    assert_eq!("Tuple { _0: 1 }", format!("{:?}", Enum::Tuple(1)));
}

#[test]
fn named_field_3() {
    use core::fmt::{self, Formatter};

    fn fmt(_s: &u8, f: &mut Formatter) -> fmt::Result {
        f.write_str("Hi")
    }

    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        #[educe(Debug(named_field = false))]
        Struct {
            #[educe(Debug(method = fmt))]
            f1: u8,
        },
        #[educe(Debug(named_field = true))]
        Tuple(#[educe(Debug(method = fmt))] u8),
    }

    assert_eq!(
        "Struct(Hi)",
        format!("{:?}", Enum::Struct {
            f1: 1
        })
    );
    assert_eq!("Tuple { _0: Hi }", format!("{:?}", Enum::Tuple(1)));
}

#[test]
fn rename_variant_1() {
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        #[educe(Debug = Hi)]
        Unit,
        #[educe(Debug = Hi)]
        Struct { f1: u8 },
        #[educe(Debug = Hi)]
        Tuple(u8),
    }

    assert_eq!("Hi", format!("{:?}", Enum::Unit));
    assert_eq!(
        "Hi { f1: 1 }",
        format!("{:?}", Enum::Struct {
            f1: 1
        })
    );
    assert_eq!("Hi(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
fn rename_variant_2() {
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        #[educe(Debug(name = Hi))]
        Unit,
        #[educe(Debug(name = Hi))]
        Struct { f1: u8 },
        #[educe(Debug(name = Hi))]
        Tuple(u8),
    }

    assert_eq!("Hi", format!("{:?}", Enum::Unit));
    assert_eq!(
        "Hi { f1: 1 }",
        format!("{:?}", Enum::Struct {
            f1: 1
        })
    );
    assert_eq!("Hi(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
fn rename_variant_3() {
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        #[educe(Debug(name(Hi)))]
        Unit,
        #[educe(Debug(name(Hi)))]
        Struct { f1: u8 },
        #[educe(Debug(name(Hi)))]
        Tuple(u8),
    }

    assert_eq!("Hi", format!("{:?}", Enum::Unit));
    assert_eq!(
        "Hi { f1: 1 }",
        format!("{:?}", Enum::Struct {
            f1: 1
        })
    );
    assert_eq!("Hi(1)", format!("{:?}", Enum::Tuple(1)));
}

#[allow(dead_code)]
#[test]
fn rename_variant_4() {
    #[derive(Educe)]
    #[educe(Debug(name = true))]
    enum Enum {
        #[educe(Debug(name(Hi)))]
        Unit,
        #[educe(Debug(name(Hi)))]
        Struct { f1: u8 },
        #[educe(Debug(name(Hi)))]
        Tuple(u8),
    }

    assert_eq!("Enum::Hi", format!("{:?}", Enum::Unit));
    assert_eq!(
        "Enum::Hi { f1: 1 }",
        format!("{:?}", Enum::Struct {
            f1: 1
        })
    );
    assert_eq!("Enum::Hi(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
fn rename_field_1() {
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        Struct {
            #[educe(Debug = f)]
            f1: u8,
        },
        #[educe(Debug(named_field(true)))]
        Tuple(#[educe(Debug = f)] u8),
    }

    assert_eq!(
        "Struct { f: 1 }",
        format!("{:?}", Enum::Struct {
            f1: 1
        })
    );
    assert_eq!("Tuple { f: 1 }", format!("{:?}", Enum::Tuple(1)));
}

#[test]
fn rename_field_2() {
    #[derive(Educe)]
    #[educe(Debug)]
    struct Struct {
        #[educe(Debug(name = f))]
        f1: u8,
    }

    assert_eq!(
        "Struct { f: 1 }",
        format!("{:?}", Struct {
            f1: 1
        })
    );

    #[derive(Educe)]
    #[educe(Debug(named_field(true)))]
    struct Tuple(#[educe(Debug(name = f))] u8);

    assert_eq!("Tuple { f: 1 }", format!("{:?}", Tuple(1)));
}

#[test]
fn rename_field_3() {
    #[derive(Educe)]
    #[educe(Debug)]
    struct Struct {
        #[educe(Debug(name(f)))]
        f1: u8,
    }

    assert_eq!(
        "Struct { f: 1 }",
        format!("{:?}", Struct {
            f1: 1
        })
    );

    #[derive(Educe)]
    #[educe(Debug(named_field(true)))]
    struct Tuple(#[educe(Debug(name(f)))] u8);

    assert_eq!("Tuple { f: 1 }", format!("{:?}", Tuple(1)));
}

#[allow(dead_code)]
#[test]
fn ignore_1() {
    #[derive(Educe)]
    #[educe(Debug)]
    struct Struct {
        #[educe(Debug = false)]
        f1: u8,
        f2: u8,
    }

    assert_eq!(
        "Struct { f2: 2 }",
        format!("{:?}", Struct {
            f1: 1, f2: 2
        })
    );

    #[derive(Educe)]
    #[educe(Debug)]
    struct Tuple(#[educe(Debug = false)] u8, u8);

    assert_eq!("Tuple(2)", format!("{:?}", Tuple(1, 2)));
}

#[allow(dead_code)]
#[test]
fn ignore_2() {
    #[derive(Educe)]
    #[educe(Debug)]
    struct Struct {
        #[educe(Debug = "")]
        f1: u8,
        f2: u8,
    }

    assert_eq!(
        "Struct { f2: 2 }",
        format!("{:?}", Struct {
            f1: 1, f2: 2
        })
    );

    #[derive(Educe)]
    #[educe(Debug)]
    struct Tuple(#[educe(Debug = false)] u8, u8);

    assert_eq!("Tuple(2)", format!("{:?}", Tuple(1, 2)));
}

#[allow(dead_code)]
#[test]
fn ignore_3() {
    #[derive(Educe)]
    #[educe(Debug)]
    struct Struct {
        #[educe(Debug(ignore))]
        f1: u8,
        f2: u8,
    }

    assert_eq!(
        "Struct { f2: 2 }",
        format!("{:?}", Struct {
            f1: 1, f2: 2
        })
    );

    #[derive(Educe)]
    #[educe(Debug)]
    struct Tuple(#[educe(Debug(ignore))] u8, u8);

    assert_eq!("Tuple(2)", format!("{:?}", Tuple(1, 2)));
}

#[test]
fn method_1() {
    use core::fmt::{self, Formatter};

    fn fmt(_s: &u8, f: &mut Formatter) -> fmt::Result {
        f.write_str("Hi")
    }

    #[derive(Educe)]
    #[educe(Debug)]
    struct Struct {
        #[educe(Debug(method = fmt))]
        f1: u8,
    }

    assert_eq!(
        "Struct { f1: Hi }",
        format!("{:?}", Struct {
            f1: 1
        })
    );

    #[derive(Educe)]
    #[educe(Debug)]
    struct Tuple(#[educe(Debug(method = fmt))] u8);

    assert_eq!("Tuple(Hi)", format!("{:?}", Tuple(1)));
}

#[test]
fn method_2() {
    use core::fmt::{self, Formatter};

    fn fmt(_s: &u8, f: &mut Formatter) -> fmt::Result {
        f.write_str("Hi")
    }

    #[derive(Educe)]
    #[educe(Debug)]
    struct Struct {
        #[educe(Debug(method(fmt)))]
        f1: u8,
    }

    assert_eq!(
        "Struct { f1: Hi }",
        format!("{:?}", Struct {
            f1: 1
        })
    );

    #[derive(Educe)]
    #[educe(Debug)]
    struct Tuple(#[educe(Debug(method(fmt)))] u8);

    assert_eq!("Tuple(Hi)", format!("{:?}", Tuple(1)));
}

#[test]
fn bound_1() {
    #[derive(Educe)]
    #[educe(Debug)]
    struct Struct<T> {
        f1: T,
    }

    assert_eq!(
        "Struct { f1: 1 }",
        format!("{:?}", Struct {
            f1: 1
        })
    );

    #[derive(Educe)]
    #[educe(Debug)]
    struct Tuple<T>(T);

    assert_eq!("Tuple(1)", format!("{:?}", Tuple(1)));
}

#[test]
fn bound_2() {
    #[derive(Educe)]
    #[educe(Debug(bound = "T: core::fmt::Debug"))]
    struct Struct<T> {
        f1: T,
    }

    assert_eq!(
        "Struct { f1: 1 }",
        format!("{:?}", Struct {
            f1: 1
        })
    );

    #[derive(Educe)]
    #[educe(Debug(bound = "T: core::fmt::Debug"))]
    struct Tuple<T>(T);

    assert_eq!("Tuple(1)", format!("{:?}", Tuple(1)));
}

#[test]
fn bound_3() {
    #[derive(Educe)]
    #[educe(Debug(bound(T: core::fmt::Debug)))]
    struct Struct<T> {
        f1: T,
    }

    assert_eq!(
        "Struct { f1: 1 }",
        format!("{:?}", Struct {
            f1: 1
        })
    );

    #[derive(Educe)]
    #[educe(Debug(bound(T: core::fmt::Debug)))]
    struct Tuple<T>(T);

    assert_eq!("Tuple(1)", format!("{:?}", Tuple(1)));
}
