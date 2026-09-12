#![cfg(feature = "Hash")]

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use educe::Educe;

#[test]
fn empty() {
    #[allow(dead_code)]
    #[derive(Educe)]
    #[educe(Hash)]
    struct Struct {}

    #[allow(dead_code)]
    #[derive(Educe)]
    #[educe(Hash)]
    struct Tuple();
}

#[test]
fn basic() {
    #[derive(Educe)]
    #[educe(Hash)]
    struct Unit;

    #[derive(Educe)]
    #[educe(Hash)]
    struct Struct {
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Hash)]
    struct Tuple(u8);

    let unit_hash = {
        let mut hasher = DefaultHasher::new();

        Unit.hash(&mut hasher);

        hasher.finish()
    };

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Struct {
            f1: 1
        }
        .hash(&mut hasher);

        hasher.finish()
    };

    let tuple_hash = {
        let mut hasher = DefaultHasher::new();

        Tuple(1).hash(&mut hasher);

        hasher.finish()
    };

    assert_ne!(unit_hash, struct_hash);
    assert_eq!(struct_hash, tuple_hash);
}

#[allow(dead_code)]
#[test]
fn ignore_1() {
    #[derive(Educe)]
    #[educe(Hash)]
    struct Unit;

    #[derive(Educe)]
    #[educe(Hash)]
    struct Struct {
        #[educe(Hash = false)]
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Hash)]
    struct Tuple(#[educe(Hash = false)] u8);

    let unit_hash = {
        let mut hasher = DefaultHasher::new();

        Unit.hash(&mut hasher);

        hasher.finish()
    };

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Struct {
            f1: 1
        }
        .hash(&mut hasher);

        hasher.finish()
    };

    let tuple_hash = {
        let mut hasher = DefaultHasher::new();

        Tuple(1).hash(&mut hasher);

        hasher.finish()
    };

    assert_eq!(unit_hash, struct_hash);
    assert_eq!(struct_hash, tuple_hash);
}

#[allow(dead_code)]
#[test]
fn ignore_2() {
    #[derive(Educe)]
    #[educe(Hash)]
    struct Unit;

    #[derive(Educe)]
    #[educe(Hash)]
    struct Struct {
        #[educe(Hash(ignore))]
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Hash)]
    struct Tuple(#[educe(Hash(ignore))] u8);

    let unit_hash = {
        let mut hasher = DefaultHasher::new();

        Unit.hash(&mut hasher);

        hasher.finish()
    };

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Struct {
            f1: 1
        }
        .hash(&mut hasher);

        hasher.finish()
    };

    let tuple_hash = {
        let mut hasher = DefaultHasher::new();

        Tuple(1).hash(&mut hasher);

        hasher.finish()
    };

    assert_eq!(unit_hash, struct_hash);
    assert_eq!(struct_hash, tuple_hash);
}

#[test]
fn method_1() {
    fn hash<H: Hasher>(_s: &u8, state: &mut H) {
        100.hash(state)
    }

    #[derive(Educe)]
    #[educe(Hash)]
    struct Unit;

    #[derive(Educe)]
    #[educe(Hash)]
    struct Struct {
        #[educe(Hash(method = hash))]
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Hash)]
    struct Struct2 {
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Hash)]
    struct Tuple(#[educe(Hash(method = hash))] u8);

    let unit_hash = {
        let mut hasher = DefaultHasher::new();

        Unit.hash(&mut hasher);

        hasher.finish()
    };

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Struct {
            f1: 1
        }
        .hash(&mut hasher);

        hasher.finish()
    };

    let struct2_hash = {
        let mut hasher = DefaultHasher::new();

        Struct2 {
            f1: 1
        }
        .hash(&mut hasher);

        hasher.finish()
    };

    let tuple_hash = {
        let mut hasher = DefaultHasher::new();

        Tuple(1).hash(&mut hasher);

        hasher.finish()
    };

    assert_ne!(unit_hash, struct_hash);
    assert_ne!(struct_hash, struct2_hash);
    assert_eq!(struct_hash, tuple_hash);
}

#[test]
fn method_2() {
    fn hash<H: Hasher>(_s: &u8, state: &mut H) {
        100.hash(state)
    }

    #[derive(Educe)]
    #[educe(Hash)]
    struct Unit;

    #[derive(Educe)]
    #[educe(Hash)]
    struct Struct {
        #[educe(Hash(method(hash)))]
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Hash)]
    struct Struct2 {
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Hash)]
    struct Tuple(#[educe(Hash(method(hash)))] u8);

    let unit_hash = {
        let mut hasher = DefaultHasher::new();

        Unit.hash(&mut hasher);

        hasher.finish()
    };

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Struct {
            f1: 1
        }
        .hash(&mut hasher);

        hasher.finish()
    };

    let struct2_hash = {
        let mut hasher = DefaultHasher::new();

        Struct2 {
            f1: 1
        }
        .hash(&mut hasher);

        hasher.finish()
    };

    let tuple_hash = {
        let mut hasher = DefaultHasher::new();

        Tuple(1).hash(&mut hasher);

        hasher.finish()
    };

    assert_ne!(unit_hash, struct_hash);
    assert_ne!(struct_hash, struct2_hash);
    assert_eq!(struct_hash, tuple_hash);
}

#[test]
fn bound_1() {
    #[derive(Educe)]
    #[educe(Hash)]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(Hash)]
    struct Tuple<T>(T);

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Struct {
            f1: 1
        }
        .hash(&mut hasher);

        hasher.finish()
    };

    let tuple_hash = {
        let mut hasher = DefaultHasher::new();

        Tuple(1).hash(&mut hasher);

        hasher.finish()
    };

    assert_eq!(struct_hash, tuple_hash);
}

#[test]
fn bound_2() {
    #[derive(Educe)]
    #[educe(Hash(bound = "T: core::hash::Hash"))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(Hash(bound = "T: core::hash::Hash"))]
    struct Tuple<T>(T);

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Struct {
            f1: 1
        }
        .hash(&mut hasher);

        hasher.finish()
    };

    let tuple_hash = {
        let mut hasher = DefaultHasher::new();

        Tuple(1).hash(&mut hasher);

        hasher.finish()
    };

    assert_eq!(struct_hash, tuple_hash);
}

#[test]
fn bound_3() {
    #[derive(Educe)]
    #[educe(Hash(bound(T: core::hash::Hash)))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(Hash(bound(T: core::hash::Hash)))]
    struct Tuple<T>(T);

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Struct {
            f1: 1
        }
        .hash(&mut hasher);

        hasher.finish()
    };

    let tuple_hash = {
        let mut hasher = DefaultHasher::new();

        Tuple(1).hash(&mut hasher);

        hasher.finish()
    };

    assert_eq!(struct_hash, tuple_hash);
}

#[test]
fn explicit_bounds() {
    // `bound(*)` adds a predicate for every type parameter, and `bound(false)` adds none, so the impl relies on what the type itself declares.
    #[derive(Educe)]
    #[educe(Hash(bound(*)))]
    struct All<T>(T, u8);

    #[derive(Educe)]
    #[educe(Hash(bound(false)))]
    struct Disabled<T: Hash>(T);

    fn hash<T: Hash>(value: &T) -> u64 {
        let mut hasher = DefaultHasher::new();

        value.hash(&mut hasher);

        hasher.finish()
    }

    assert_eq!(hash(&All(1u8, 2u8)), hash(&All(1u8, 2u8)));
    assert_eq!(hash(&Disabled(1u8)), hash(&Disabled(1u8)));
}
