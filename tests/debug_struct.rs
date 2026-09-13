#![cfg(feature = "Debug")]
#![no_std]
// The types in these tests only exist to exercise the derived impls, and `#[automatically_derived]` impls do not count as uses for dead-code analysis.
#![allow(dead_code)]

#[macro_use]
extern crate alloc;

use educe::Educe;

#[test]
fn doc_comment_with_apostrophe() {
    /// Stores the group's title.
    #[derive(Educe)]
    #[educe(Debug)]
    struct Group {
        title: &'static str,
    }

    assert_eq!(
        "Group { title: \"Example\" }",
        format!("{:?}", Group {
            title: "Example"
        })
    );
}

#[test]
fn empty() {
    #[derive(Educe)]
    #[educe(Debug)]
    struct Struct {}

    assert_eq!("Struct", format!("{:?}", Struct {}));

    #[derive(Educe)]
    #[educe(Debug)]
    struct Tuple();

    assert_eq!("Tuple", format!("{:?}", Tuple()));
}

#[test]
fn name_1() {
    #[derive(Educe)]
    #[educe(Debug)]
    struct Unit;

    assert_eq!("Unit", format!("{:?}", Unit));

    #[derive(Educe)]
    #[educe(Debug)]
    struct Struct {
        f1: u8,
    }

    assert_eq!(
        "Struct { f1: 1 }",
        format!("{:?}", Struct {
            f1: 1
        })
    );

    #[derive(Educe)]
    #[educe(Debug)]
    struct Tuple(u8);

    assert_eq!("Tuple(1)", format!("{:?}", Tuple(1)));
}

#[test]
fn name_2() {
    #[derive(Educe)]
    #[educe(Debug = A)]
    struct Unit;

    assert_eq!("A", format!("{:?}", Unit));

    #[derive(Educe)]
    #[educe(Debug = B)]
    struct Struct {
        f1: u8,
    }

    assert_eq!(
        "B { f1: 1 }",
        format!("{:?}", Struct {
            f1: 1
        })
    );

    #[derive(Educe)]
    #[educe(Debug = C)]
    struct Tuple(u8);

    assert_eq!("C(1)", format!("{:?}", Tuple(1)));
}

#[test]
fn name_3() {
    #[derive(Educe)]
    #[educe(Debug(name = A))]
    struct Unit;

    assert_eq!("A", format!("{:?}", Unit));

    #[derive(Educe)]
    #[educe(Debug(name = B))]
    struct Struct {
        f1: u8,
    }

    assert_eq!(
        "B { f1: 1 }",
        format!("{:?}", Struct {
            f1: 1
        })
    );

    #[derive(Educe)]
    #[educe(Debug(name = C))]
    struct Tuple(u8);

    assert_eq!("C(1)", format!("{:?}", Tuple(1)));
}

#[test]
fn name_4() {
    #[derive(Educe)]
    #[educe(Debug(name(A)))]
    struct Unit;

    assert_eq!("A", format!("{:?}", Unit));

    #[derive(Educe)]
    #[educe(Debug(name(B)))]
    struct Struct {
        f1: u8,
    }

    assert_eq!(
        "B { f1: 1 }",
        format!("{:?}", Struct {
            f1: 1
        })
    );

    #[derive(Educe)]
    #[educe(Debug(name(C)))]
    struct Tuple(u8);

    assert_eq!("C(1)", format!("{:?}", Tuple(1)));
}

#[test]
fn unnamed_1() {
    #[derive(Educe)]
    #[educe(Debug(name = false))]
    struct Struct {
        f1: u8,
    }

    assert_eq!(
        "{f1: 1}",
        format!("{:?}", Struct {
            f1: 1
        })
    );

    #[derive(Educe)]
    #[educe(Debug(name = false))]
    struct Tuple(u8);

    assert_eq!("(1,)", format!("{:?}", Tuple(1)));
}

#[test]
fn unnamed_2() {
    #[derive(Educe)]
    #[educe(Debug(name(false)))]
    struct Struct {
        f1: u8,
    }

    assert_eq!(
        "{f1: 1}",
        format!("{:?}", Struct {
            f1: 1
        })
    );

    #[derive(Educe)]
    #[educe(Debug(name(false)))]
    struct Tuple(u8);

    assert_eq!("(1,)", format!("{:?}", Tuple(1)));
}

#[test]
fn unnamed_3() {
    #[derive(Educe)]
    #[educe(Debug(name = ""))]
    struct Struct {
        f1: u8,
    }

    assert_eq!(
        "{f1: 1}",
        format!("{:?}", Struct {
            f1: 1
        })
    );

    #[derive(Educe)]
    #[educe(Debug(name = ""))]
    struct Tuple(u8);

    assert_eq!("(1,)", format!("{:?}", Tuple(1)));
}

#[test]
fn unnamed_4() {
    #[derive(Educe)]
    #[educe(Debug(name("")))]
    struct Struct {
        f1: u8,
    }

    assert_eq!(
        "{f1: 1}",
        format!("{:?}", Struct {
            f1: 1
        })
    );

    #[derive(Educe)]
    #[educe(Debug(name("")))]
    struct Tuple(u8);

    assert_eq!("(1,)", format!("{:?}", Tuple(1)));
}

#[test]
fn named_field_1() {
    #[derive(Educe)]
    #[educe(Debug(named_field = false))]
    struct Struct {
        f1: u8,
    }

    assert_eq!(
        "Struct(1)",
        format!("{:?}", Struct {
            f1: 1
        })
    );

    #[derive(Educe)]
    #[educe(Debug(named_field = true))]
    struct Tuple(u8);

    assert_eq!("Tuple { 0: 1 }", format!("{:?}", Tuple(1)));
}

#[test]
fn named_field_2() {
    #[derive(Educe)]
    #[educe(Debug(named_field(false)))]
    struct Struct {
        f1: u8,
    }

    assert_eq!(
        "Struct(1)",
        format!("{:?}", Struct {
            f1: 1
        })
    );

    #[derive(Educe)]
    #[educe(Debug(named_field(true)))]
    struct Tuple(u8);

    assert_eq!("Tuple { 0: 1 }", format!("{:?}", Tuple(1)));
}

#[test]
fn rename_field_1() {
    #[derive(Educe)]
    #[educe(Debug)]
    struct Struct {
        #[educe(Debug = f)]
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
    struct Tuple(#[educe(Debug = f)] u8);

    assert_eq!("Tuple { f: 1 }", format!("{:?}", Tuple(1)));
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

    fn fmt(_s: &u8, f: &mut Formatter<'_>) -> fmt::Result {
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

    fn fmt(_s: &u8, f: &mut Formatter<'_>) -> fmt::Result {
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

#[test]
fn dynamically_sized_fields() {
    #[derive(Educe)]
    #[educe(Debug)]
    struct Struct<T: ?Sized> {
        field: T,
    }

    #[derive(Educe)]
    #[educe(Debug)]
    struct Tuple<T: ?Sized>(T);

    #[derive(Educe)]
    #[educe(Debug(name = false))]
    struct Map<T: ?Sized> {
        field: T,
    }

    let named = Struct {
        field: [1u8, 2]
    };
    let tuple = Tuple([1u8, 2]);
    let map = Map {
        field: [1u8, 2]
    };
    let named: &Struct<[u8]> = &named;
    let tuple: &Tuple<[u8]> = &tuple;
    let map: &Map<[u8]> = &map;

    assert_eq!("Struct { field: [1, 2] }", format!("{named:?}"));
    assert_eq!("Tuple([1, 2])", format!("{tuple:?}"));
    assert_eq!("{field: [1, 2]}", format!("{map:?}"));
}

#[test]
fn explicit_bounds() {
    // `bound(*)` adds a predicate for every type parameter, and `bound(false)` adds none, so the impl relies on what the type itself declares.
    #[derive(Educe)]
    #[educe(Debug(bound(*)))]
    struct All<T>(T, u8);

    #[derive(Educe)]
    #[educe(Debug(bound(false)))]
    struct Disabled<T: core::fmt::Debug>(T);

    assert_eq!("All(1, 2)", format!("{:?}", All(1u8, 2u8)));
    assert_eq!("Disabled(1)", format!("{:?}", Disabled(1u8)));
}
