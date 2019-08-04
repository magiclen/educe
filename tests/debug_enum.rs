#![cfg(feature = "Debug")]
#![no_std]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate educe;

#[test]
#[allow(dead_code)]
fn name_1() {
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        Unit,
        Struct { f1: u8 },
        Tuple(u8),
    }

    assert_eq!("Unit", format!("{:?}", Enum::Unit));
    assert_eq!("Struct { f1: 1 }", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Tuple(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn name_2() {
    #[derive(Educe)]
    #[educe(Debug = "Hi")]
    enum Enum {
        Unit,
        Struct { f1: u8 },
        Tuple(u8),
    }

    assert_eq!("Hi::Unit", format!("{:?}", Enum::Unit));
    assert_eq!(
        "Hi::Struct { f1: 1 }",
        format!("{:?}", Enum::Struct { f1: 1 })
    );
    assert_eq!("Hi::Tuple(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn name_3() {
    #[derive(Educe)]
    #[educe(Debug("Hi"))]
    enum Enum {
        Unit,
        Struct { f1: u8 },
        Tuple(u8),
    }

    assert_eq!("Hi::Unit", format!("{:?}", Enum::Unit));
    assert_eq!(
        "Hi::Struct { f1: 1 }",
        format!("{:?}", Enum::Struct { f1: 1 })
    );
    assert_eq!("Hi::Tuple(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn name_4() {
    #[derive(Educe)]
    #[educe(Debug(name = "Hi"))]
    enum Enum {
        Unit,
        Struct { f1: u8 },
        Tuple(u8),
    }

    assert_eq!("Hi::Unit", format!("{:?}", Enum::Unit));
    assert_eq!(
        "Hi::Struct { f1: 1 }",
        format!("{:?}", Enum::Struct { f1: 1 })
    );
    assert_eq!("Hi::Tuple(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn name_5() {
    #[derive(Educe)]
    #[educe(Debug(name("Hi")))]
    enum Enum {
        Unit,
        Struct { f1: u8 },
        Tuple(u8),
    }

    assert_eq!("Hi::Unit", format!("{:?}", Enum::Unit));
    assert_eq!(
        "Hi::Struct { f1: 1 }",
        format!("{:?}", Enum::Struct { f1: 1 })
    );
    assert_eq!("Hi::Tuple(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn name_7() {
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
        format!("{:?}", Enum::Struct { f1: 1 })
    );
    assert_eq!("Enum::Tuple(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn name_8() {
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
        format!("{:?}", Enum::Struct { f1: 1 })
    );
    assert_eq!("Enum::Tuple(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
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
    assert_eq!("Enum { f1: 1 }", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Enum(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
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
    assert_eq!("Enum { f1: 1 }", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Enum(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
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
    assert_eq!("Enum { f1: 1 }", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Enum(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
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
    assert_eq!("Enum { f1: 1 }", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Enum(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn unnamed_variant_5() {
    #[derive(Educe)]
    #[educe(Debug(name = true))]
    enum Enum {
        #[educe(Debug = "")]
        Unit,
        #[educe(Debug = "")]
        Struct { f1: u8 },
        #[educe(Debug = "")]
        Tuple(u8),
    }

    assert_eq!("Enum", format!("{:?}", Enum::Unit));
    assert_eq!("Enum { f1: 1 }", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Enum(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn unnamed_variant_6() {
    #[derive(Educe)]
    #[educe(Debug(name = true))]
    enum Enum {
        #[educe(Debug(""))]
        Unit,
        #[educe(Debug(""))]
        Struct { f1: u8 },
        #[educe(Debug(""))]
        Tuple(u8),
    }

    assert_eq!("Enum", format!("{:?}", Enum::Unit));
    assert_eq!("Enum { f1: 1 }", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Enum(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn named_field_1() {
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        #[educe(Debug(named_field = false))]
        Struct { f1: u8 },
        #[educe(Debug(named_field = true))]
        Tuple(u8),
    }

    assert_eq!("Struct(1)", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Tuple { _0: 1 }", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn named_field_2() {
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        #[educe(Debug(named_field(false)))]
        Struct { f1: u8 },
        #[educe(Debug(named_field(true)))]
        Tuple(u8),
    }

    assert_eq!("Struct(1)", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Tuple { _0: 1 }", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
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
            #[educe(Debug(method = "fmt"))]
            f1: u8,
        },
        #[educe(Debug(named_field = true))]
        Tuple(#[educe(Debug(method = "fmt"))] u8),
    }

    assert_eq!("Struct(Hi)", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Tuple { _0: Hi }", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn named_field_4() {
    use core::fmt::{self, Formatter};

    trait A {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            f.write_str("Hi")
        }
    }

    impl A for u8 {}

    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum<T: A> {
        #[educe(Debug(named_field = false))]
        Struct {
            #[educe(Debug(trait = "A"))]
            f1: T,
        },
        #[educe(Debug(named_field = true))]
        Tuple(#[educe(Debug(trait = "A"))] T),
    }

    assert_eq!("Struct(Hi)", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Tuple { _0: Hi }", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn rename_variant_1() {
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        #[educe(Debug = "Hi")]
        Unit,
        #[educe(Debug = "Hi")]
        Struct { f1: u8 },
        #[educe(Debug = "Hi")]
        Tuple(u8),
    }

    assert_eq!("Hi", format!("{:?}", Enum::Unit));
    assert_eq!("Hi { f1: 1 }", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Hi(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn rename_variant_2() {
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        #[educe(Debug("Hi"))]
        Unit,
        #[educe(Debug("Hi"))]
        Struct { f1: u8 },
        #[educe(Debug("Hi"))]
        Tuple(u8),
    }

    assert_eq!("Hi", format!("{:?}", Enum::Unit));
    assert_eq!("Hi { f1: 1 }", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Hi(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn rename_variant_3() {
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        #[educe(Debug(name = "Hi"))]
        Unit,
        #[educe(Debug(name = "Hi"))]
        Struct { f1: u8 },
        #[educe(Debug(name = "Hi"))]
        Tuple(u8),
    }

    assert_eq!("Hi", format!("{:?}", Enum::Unit));
    assert_eq!("Hi { f1: 1 }", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Hi(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn rename_variant_4() {
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        #[educe(Debug(name("Hi")))]
        Unit,
        #[educe(Debug(name("Hi")))]
        Struct { f1: u8 },
        #[educe(Debug(name("Hi")))]
        Tuple(u8),
    }

    assert_eq!("Hi", format!("{:?}", Enum::Unit));
    assert_eq!("Hi { f1: 1 }", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Hi(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn rename_variant_5() {
    #[derive(Educe)]
    #[educe(Debug(name = true))]
    enum Enum {
        #[educe(Debug(name("Hi")))]
        Unit,
        #[educe(Debug(name("Hi")))]
        Struct { f1: u8 },
        #[educe(Debug(name("Hi")))]
        Tuple(u8),
    }

    assert_eq!("Enum::Hi", format!("{:?}", Enum::Unit));
    assert_eq!(
        "Enum::Hi { f1: 1 }",
        format!("{:?}", Enum::Struct { f1: 1 })
    );
    assert_eq!("Enum::Hi(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn rename_variant_6() {
    #[derive(Educe)]
    #[educe(Debug(name = true))]
    enum Enum {
        #[educe(Debug(name("::Hi")))]
        Unit,
        #[educe(Debug(name("::Hi")))]
        Struct { f1: u8 },
        #[educe(Debug(name("::Hi")))]
        Tuple(u8),
    }

    assert_eq!("Enum::Hi", format!("{:?}", Enum::Unit));
    assert_eq!(
        "Enum::Hi { f1: 1 }",
        format!("{:?}", Enum::Struct { f1: 1 })
    );
    assert_eq!("Enum::Hi(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn rename_field_1() {
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        Struct {
            #[educe(Debug = "f")]
            f1: u8,
        },
        #[educe(Debug(named_field(true)))]
        Tuple(#[educe(Debug = "f")] u8),
    }

    assert_eq!("Struct { f: 1 }", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Tuple { f: 1 }", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn rename_field_2() {
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        Struct {
            #[educe(Debug("f"))]
            f1: u8,
        },
        #[educe(Debug(named_field(true)))]
        Tuple(#[educe(Debug = "f")] u8),
    }

    assert_eq!("Struct { f: 1 }", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Tuple { f: 1 }", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn rename_field_3() {
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        Struct {
            #[educe(Debug(name = "f"))]
            f1: u8,
        },
        #[educe(Debug(named_field(true)))]
        Tuple(#[educe(Debug = "f")] u8),
    }

    assert_eq!("Struct { f: 1 }", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Tuple { f: 1 }", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn rename_field_4() {
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        Struct {
            #[educe(Debug(name("f")))]
            f1: u8,
        },
        #[educe(Debug(named_field(true)))]
        Tuple(#[educe(Debug = "f")] u8),
    }

    assert_eq!("Struct { f: 1 }", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Tuple { f: 1 }", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn ignore_1() {
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        Struct {
            #[educe(Debug = false)]
            f1: u8,
            f2: u8,
        },
        Tuple(#[educe(Debug = false)] u8, u8),
    }

    assert_eq!(
        "Struct { f2: 2 }",
        format!("{:?}", Enum::Struct { f1: 1, f2: 2 })
    );
    assert_eq!("Tuple(2)", format!("{:?}", Enum::Tuple(1, 2)));
}

#[test]
#[allow(dead_code)]
fn ignore_2() {
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        Struct {
            #[educe(Debug(false))]
            f1: u8,
            f2: u8,
        },
        Tuple(#[educe(Debug(false))] u8, u8),
    }

    assert_eq!(
        "Struct { f2: 2 }",
        format!("{:?}", Enum::Struct { f1: 1, f2: 2 })
    );
    assert_eq!("Tuple(2)", format!("{:?}", Enum::Tuple(1, 2)));
}

#[test]
#[allow(dead_code)]
fn ignore_3() {
    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        Struct {
            #[educe(Debug(ignore))]
            f1: u8,
            f2: u8,
        },
        Tuple(#[educe(Debug(ignore))] u8, u8),
    }

    assert_eq!(
        "Struct { f2: 2 }",
        format!("{:?}", Enum::Struct { f1: 1, f2: 2 })
    );
    assert_eq!("Tuple(2)", format!("{:?}", Enum::Tuple(1, 2)));
}

#[test]
#[allow(dead_code)]
fn format_without_trait_1() {
    use core::fmt::{self, Formatter};

    fn fmt(_s: &u8, f: &mut Formatter) -> fmt::Result {
        f.write_str("Hi")
    }

    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        Struct {
            #[educe(Debug(method = "fmt"))]
            f1: u8,
        },
        Tuple(#[educe(Debug(method = "fmt"))] u8),
    }

    assert_eq!("Struct { f1: Hi }", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Tuple(Hi)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn format_without_trait_2() {
    use core::fmt::{self, Formatter};

    fn fmt(_s: &u8, f: &mut Formatter) -> fmt::Result {
        f.write_str("Hi")
    }

    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum {
        Struct {
            #[educe(Debug(method("fmt")))]
            f1: u8,
        },
        Tuple(#[educe(Debug(method("fmt")))] u8),
    }

    assert_eq!("Struct { f1: Hi }", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Tuple(Hi)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn format_with_trait_1() {
    use core::fmt::{self, Formatter};

    trait A {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            f.write_str("Hi")
        }
    }

    impl A for u8 {}

    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum<T: A> {
        Struct {
            #[educe(Debug(trait = "A"))]
            f1: T,
        },
        Tuple(#[educe(Debug(trait = "A"))] T),
    }

    assert_eq!("Struct { f1: Hi }", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Tuple(Hi)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn format_with_trait_2() {
    use core::fmt::{self, Formatter};

    trait A {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            f.write_str("Hi")
        }
    }

    impl A for u8 {}

    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum<T: A> {
        Struct {
            #[educe(Debug(trait("A")))]
            f1: T,
        },
        Tuple(#[educe(Debug(trait("A")))] T),
    }

    assert_eq!("Struct { f1: Hi }", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Tuple(Hi)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn format_with_trait_3() {
    use core::fmt::{self, Formatter};

    trait A {
        fn format(&self, f: &mut Formatter) -> fmt::Result {
            f.write_str("Hi")
        }
    }

    impl A for u8 {}

    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum<T: A> {
        Struct {
            #[educe(Debug(trait = "A", method = "format"))]
            f1: T,
        },
        Tuple(#[educe(Debug(trait = "A", method = "format"))] T),
    }

    assert_eq!("Struct { f1: Hi }", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Tuple(Hi)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn format_with_trait_4() {
    use core::fmt::{self, Formatter};

    trait A {
        fn format(&self, f: &mut Formatter) -> fmt::Result {
            f.write_str("Hi")
        }
    }

    impl A for u8 {}

    #[derive(Educe)]
    #[educe(Debug)]
    enum Enum<T: A> {
        Struct {
            #[educe(Debug(trait("A"), method("format")))]
            f1: T,
        },
        Tuple(#[educe(Debug(trait("A"), method("format")))] T),
    }

    assert_eq!("Struct { f1: Hi }", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Tuple(Hi)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn bound_1() {
    #[derive(Educe)]
    #[educe(Debug(bound))]
    enum Enum<T> {
        Unit,
        Struct { f1: T },
        Tuple(T),
    }

    assert_eq!("Unit", format!("{:?}", Enum::<u8>::Unit));
    assert_eq!("Struct { f1: 1 }", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Tuple(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn bound_2() {
    #[derive(Educe)]
    #[educe(Debug(bound = "T: core::fmt::Debug"))]
    enum Enum<T> {
        Unit,
        Struct { f1: T },
        Tuple(T),
    }

    assert_eq!("Unit", format!("{:?}", Enum::<u8>::Unit));
    assert_eq!("Struct { f1: 1 }", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Tuple(1)", format!("{:?}", Enum::Tuple(1)));
}

#[test]
#[allow(dead_code)]
fn bound_3() {
    #[derive(Educe)]
    #[educe(Debug(bound("T: core::fmt::Debug")))]
    enum Enum<T> {
        Unit,
        Struct { f1: T },
        Tuple(T),
    }

    assert_eq!("Unit", format!("{:?}", Enum::<u8>::Unit));
    assert_eq!("Struct { f1: 1 }", format!("{:?}", Enum::Struct { f1: 1 }));
    assert_eq!("Tuple(1)", format!("{:?}", Enum::Tuple(1)));
}
