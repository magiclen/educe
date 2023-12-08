#![cfg(feature = "Hash")]

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use educe::Educe;

#[test]
fn basic_1() {
    #[derive(Educe)]
    #[educe(Hash)]
    enum Enum {
        Unit,
        Unit2,
        Struct { f1: u8 },
        Tuple(u8),
    }

    let unit_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Unit.hash(&mut hasher);

        hasher.finish()
    };

    let unit2_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Unit2.hash(&mut hasher);

        hasher.finish()
    };

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Struct {
            f1: 1
        }
        .hash(&mut hasher);

        hasher.finish()
    };

    let tuple_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Tuple(1).hash(&mut hasher);

        hasher.finish()
    };

    assert_ne!(unit_hash, unit2_hash);
    assert_ne!(unit_hash, struct_hash);
    assert_ne!(struct_hash, tuple_hash);
}

#[test]
fn basic_2() {
    #[derive(Educe)]
    #[educe(Hash)]
    enum Enum {
        Unit,
        Unit2,
        Unit3,
    }

    let unit_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Unit.hash(&mut hasher);

        hasher.finish()
    };

    let unit2_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Unit2.hash(&mut hasher);

        hasher.finish()
    };

    let unit3_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Unit3.hash(&mut hasher);

        hasher.finish()
    };

    assert_ne!(unit_hash, unit2_hash);
    assert_ne!(unit_hash, unit3_hash);
    assert_ne!(unit2_hash, unit3_hash);
}

#[allow(dead_code)]
#[test]
fn ignore_1() {
    #[derive(Educe)]
    #[educe(Hash)]
    enum Enum<T> {
        Unit,
        Struct {
            #[educe(Hash = false)]
            f1: T,
        },
        Tuple(#[educe(Hash = false)] T),
    }

    let unit_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::<u8>::Unit.hash(&mut hasher);

        hasher.finish()
    };

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Struct {
            f1: 1
        }
        .hash(&mut hasher);

        hasher.finish()
    };

    let tuple_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Tuple(1).hash(&mut hasher);

        hasher.finish()
    };

    assert_ne!(unit_hash, struct_hash);
    assert_ne!(struct_hash, tuple_hash);
}

#[allow(dead_code)]
#[test]
fn ignore_2() {
    #[derive(Educe)]
    #[educe(Hash)]
    enum Enum<T> {
        Unit,
        Struct {
            #[educe(Hash(ignore))]
            f1: T,
        },
        Tuple(#[educe(Hash(ignore))] T),
    }

    let unit_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::<u8>::Unit.hash(&mut hasher);

        hasher.finish()
    };

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Struct {
            f1: 1
        }
        .hash(&mut hasher);

        hasher.finish()
    };

    let tuple_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Tuple(1).hash(&mut hasher);

        hasher.finish()
    };

    assert_ne!(unit_hash, struct_hash);
    assert_ne!(struct_hash, tuple_hash);
}

#[test]
fn method_1() {
    fn hash<H: Hasher, T>(_s: &T, state: &mut H) {
        100.hash(state)
    }

    #[derive(Educe)]
    #[educe(Hash)]
    enum Enum<T> {
        Unit,
        Struct {
            #[educe(Hash(method = hash))]
            f1: T,
        },
        Tuple(#[educe(Hash(method = hash))] T),
    }

    let unit_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::<u8>::Unit.hash(&mut hasher);

        hasher.finish()
    };

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Struct {
            f1: 1
        }
        .hash(&mut hasher);

        hasher.finish()
    };

    let tuple_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Tuple(1).hash(&mut hasher);

        hasher.finish()
    };

    assert_ne!(unit_hash, struct_hash);
    assert_ne!(struct_hash, tuple_hash);
}

#[test]
fn method_2() {
    fn hash<H: Hasher, T>(_s: &T, state: &mut H) {
        100.hash(state)
    }

    #[derive(Educe)]
    #[educe(Hash)]
    enum Enum<T> {
        Unit,
        Struct {
            #[educe(Hash(method(hash)))]
            f1: T,
        },
        Tuple(#[educe(Hash(method(hash)))] T),
    }

    let unit_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::<u8>::Unit.hash(&mut hasher);

        hasher.finish()
    };

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Struct {
            f1: 1
        }
        .hash(&mut hasher);

        hasher.finish()
    };

    let tuple_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Tuple(1).hash(&mut hasher);

        hasher.finish()
    };

    assert_ne!(unit_hash, struct_hash);
    assert_ne!(struct_hash, tuple_hash);
}

#[test]
fn bound_1() {
    #[derive(Educe)]
    #[educe(Hash)]
    enum Enum<T> {
        Unit,
        Struct { f1: T },
        Tuple(T),
    }

    let unit_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::<u8>::Unit.hash(&mut hasher);

        hasher.finish()
    };

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Struct {
            f1: 1
        }
        .hash(&mut hasher);

        hasher.finish()
    };

    let tuple_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Tuple(1).hash(&mut hasher);

        hasher.finish()
    };

    assert_ne!(unit_hash, struct_hash);
    assert_ne!(struct_hash, tuple_hash);
}

#[test]
fn bound_2() {
    #[derive(Educe)]
    #[educe(Hash(bound = "T: core::hash::Hash"))]
    enum Enum<T> {
        Unit,
        Struct { f1: T },
        Tuple(T),
    }

    let unit_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::<u8>::Unit.hash(&mut hasher);

        hasher.finish()
    };

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Struct {
            f1: 1
        }
        .hash(&mut hasher);

        hasher.finish()
    };

    let tuple_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Tuple(1).hash(&mut hasher);

        hasher.finish()
    };

    assert_ne!(unit_hash, struct_hash);
    assert_ne!(struct_hash, tuple_hash);
}

#[test]
fn bound_3() {
    #[derive(Educe)]
    #[educe(Hash(bound(T: core::hash::Hash)))]
    enum Enum<T> {
        Unit,
        Struct { f1: T },
        Tuple(T),
    }

    let unit_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::<u8>::Unit.hash(&mut hasher);

        hasher.finish()
    };

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Struct {
            f1: 1
        }
        .hash(&mut hasher);

        hasher.finish()
    };

    let tuple_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Tuple(1).hash(&mut hasher);

        hasher.finish()
    };

    assert_ne!(unit_hash, struct_hash);
    assert_ne!(struct_hash, tuple_hash);
}
