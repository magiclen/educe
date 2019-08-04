#![cfg(feature = "Hash")]

#[macro_use]
extern crate educe;

use core::hash::{Hash, Hasher};

use std::collections::hash_map::DefaultHasher;

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

        Enum::Struct { f1: 1 }.hash(&mut hasher);

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

#[test]
#[allow(dead_code)]
fn ignore() {
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

        Enum::Struct { f1: 1 }.hash(&mut hasher);

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
fn hash_without_trait_1() {
    use core::hash::Hasher;

    fn hash<H: Hasher, T>(_s: &T, state: &mut H) {
        100.hash(state)
    }

    #[derive(Educe)]
    #[educe(Hash)]
    enum Enum<T> {
        Unit,
        Struct {
            #[educe(Hash(method = "hash"))]
            f1: T,
        },
        Tuple(#[educe(Hash(method = "hash"))] T),
    }

    let unit_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::<u8>::Unit.hash(&mut hasher);

        hasher.finish()
    };

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Struct { f1: 1 }.hash(&mut hasher);

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
fn hash_without_trait_2() {
    use core::hash::Hasher;

    fn hash<H: Hasher, T>(_s: &T, state: &mut H) {
        100.hash(state)
    }

    #[derive(Educe)]
    #[educe(Hash)]
    enum Enum<T> {
        Unit,
        Struct {
            #[educe(Hash(method("hash")))]
            f1: T,
        },
        Tuple(#[educe(Hash(method("hash")))] T),
    }

    let unit_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::<u8>::Unit.hash(&mut hasher);

        hasher.finish()
    };

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Struct { f1: 1 }.hash(&mut hasher);

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
fn hash_with_trait_1() {
    use core::hash::{Hash, Hasher};

    trait A {
        fn hash<H: Hasher>(&self, state: &mut H) {
            Hash::hash(&100, state)
        }
    }

    impl A for u8 {};

    #[derive(Educe)]
    #[educe(Hash)]
    enum Enum {
        Unit,
        Struct {
            #[educe(Hash(trait = "A"))]
            f1: u8,
        },
        Tuple(#[educe(Hash(trait = "A"))] u8),
    }

    let unit_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Unit.hash(&mut hasher);

        hasher.finish()
    };

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Struct { f1: 1 }.hash(&mut hasher);

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
fn hash_with_trait_2() {
    use core::hash::{Hash, Hasher};

    trait A {
        fn hash<H: Hasher>(&self, state: &mut H) {
            Hash::hash(&100, state)
        }
    }

    impl A for u8 {};

    #[derive(Educe)]
    #[educe(Hash)]
    enum Enum {
        Unit,
        Struct {
            #[educe(Hash(trait("A")))]
            f1: u8,
        },
        Tuple(#[educe(Hash(trait("A")))] u8),
    }

    let unit_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Unit.hash(&mut hasher);

        hasher.finish()
    };

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Struct { f1: 1 }.hash(&mut hasher);

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
fn hash_with_trait_3() {
    use core::hash::{Hash, Hasher};

    trait A {
        fn my_hash<H: Hasher>(&self, state: &mut H) {
            Hash::hash(&100, state)
        }
    }

    impl A for u8 {};

    #[derive(Educe)]
    #[educe(Hash)]
    enum Enum {
        Unit,
        Struct {
            #[educe(Hash(trait = "A", method = "my_hash"))]
            f1: u8,
        },
        Tuple(#[educe(Hash(trait = "A", method = "my_hash"))] u8),
    }

    let unit_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Unit.hash(&mut hasher);

        hasher.finish()
    };

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Struct { f1: 1 }.hash(&mut hasher);

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
fn hash_with_trait_4() {
    use core::hash::{Hash, Hasher};

    trait A {
        fn my_hash<H: Hasher>(&self, state: &mut H) {
            Hash::hash(&100, state)
        }
    }

    impl A for u8 {};

    #[derive(Educe)]
    #[educe(Hash)]
    enum Enum {
        Unit,
        Struct {
            #[educe(Hash(trait("A"), method("my_hash")))]
            f1: u8,
        },
        Tuple(#[educe(Hash(trait("A"), method("my_hash")))] u8),
    }

    let unit_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Unit.hash(&mut hasher);

        hasher.finish()
    };

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Enum::Struct { f1: 1 }.hash(&mut hasher);

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
    #[educe(Hash(bound))]
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

        Enum::Struct { f1: 1 }.hash(&mut hasher);

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

        Enum::Struct { f1: 1 }.hash(&mut hasher);

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
    #[educe(Hash(bound("T: core::hash::Hash")))]
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

        Enum::Struct { f1: 1 }.hash(&mut hasher);

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
