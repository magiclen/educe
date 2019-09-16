#![allow(clippy::trivially_copy_pass_by_ref)]
#![cfg(feature = "Hash")]

#[macro_use]
extern crate educe;

use core::hash::{Hash, Hasher};

use std::collections::hash_map::DefaultHasher;

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
            f1: 1,
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

#[test]
#[allow(dead_code)]
fn ignore() {
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
            f1: 1,
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
fn hash_without_trait_1() {
    use core::hash::Hasher;

    fn hash<H: Hasher>(_s: &u8, state: &mut H) {
        100.hash(state)
    }

    #[derive(Educe)]
    #[educe(Hash)]
    struct Unit;

    #[derive(Educe)]
    #[educe(Hash)]
    struct Struct {
        #[educe(Hash(method = "hash"))]
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Hash)]
    struct Struct2 {
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Hash)]
    struct Tuple(#[educe(Hash(method = "hash"))] u8);

    let unit_hash = {
        let mut hasher = DefaultHasher::new();

        Unit.hash(&mut hasher);

        hasher.finish()
    };

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Struct {
            f1: 1,
        }
        .hash(&mut hasher);

        hasher.finish()
    };

    let struct2_hash = {
        let mut hasher = DefaultHasher::new();

        Struct2 {
            f1: 1,
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
fn hash_without_trait_2() {
    use core::hash::Hasher;

    fn hash<H: Hasher>(_s: &u8, state: &mut H) {
        100.hash(state)
    }

    #[derive(Educe)]
    #[educe(Hash)]
    struct Unit;

    #[derive(Educe)]
    #[educe(Hash)]
    struct Struct {
        #[educe(Hash(method("hash")))]
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Hash)]
    struct Struct2 {
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Hash)]
    struct Tuple(#[educe(Hash(method("hash")))] u8);

    let unit_hash = {
        let mut hasher = DefaultHasher::new();

        Unit.hash(&mut hasher);

        hasher.finish()
    };

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Struct {
            f1: 1,
        }
        .hash(&mut hasher);

        hasher.finish()
    };

    let struct2_hash = {
        let mut hasher = DefaultHasher::new();

        Struct2 {
            f1: 1,
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
    struct Unit;

    #[derive(Educe)]
    #[educe(Hash)]
    struct Struct {
        #[educe(Hash(trait = "A"))]
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Hash)]
    struct Struct2 {
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Hash)]
    struct Tuple(#[educe(Hash(trait = "A"))] u8);

    let unit_hash = {
        let mut hasher = DefaultHasher::new();

        Unit.hash(&mut hasher);

        hasher.finish()
    };

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Struct {
            f1: 1,
        }
        .hash(&mut hasher);

        hasher.finish()
    };

    let struct2_hash = {
        let mut hasher = DefaultHasher::new();

        Struct2 {
            f1: 1,
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
    struct Unit;

    #[derive(Educe)]
    #[educe(Hash)]
    struct Struct {
        #[educe(Hash(trait("A")))]
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Hash)]
    struct Struct2 {
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Hash)]
    struct Tuple(#[educe(Hash(trait("A")))] u8);

    let unit_hash = {
        let mut hasher = DefaultHasher::new();

        Unit.hash(&mut hasher);

        hasher.finish()
    };

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Struct {
            f1: 1,
        }
        .hash(&mut hasher);

        hasher.finish()
    };

    let struct2_hash = {
        let mut hasher = DefaultHasher::new();

        Struct2 {
            f1: 1,
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
    struct Unit;

    #[derive(Educe)]
    #[educe(Hash)]
    struct Struct {
        #[educe(Hash(trait = "A", method = "my_hash"))]
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Hash)]
    struct Struct2 {
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Hash)]
    struct Tuple(#[educe(Hash(trait = "A", method = "my_hash"))] u8);

    let unit_hash = {
        let mut hasher = DefaultHasher::new();

        Unit.hash(&mut hasher);

        hasher.finish()
    };

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Struct {
            f1: 1,
        }
        .hash(&mut hasher);

        hasher.finish()
    };

    let struct2_hash = {
        let mut hasher = DefaultHasher::new();

        Struct2 {
            f1: 1,
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
    struct Unit;

    #[derive(Educe)]
    #[educe(Hash)]
    struct Struct {
        #[educe(Hash(trait("A"), method("my_hash")))]
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Hash)]
    struct Struct2 {
        f1: u8,
    }

    #[derive(Educe)]
    #[educe(Hash)]
    struct Tuple(#[educe(Hash(trait("A"), method("my_hash")))] u8);

    let unit_hash = {
        let mut hasher = DefaultHasher::new();

        Unit.hash(&mut hasher);

        hasher.finish()
    };

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Struct {
            f1: 1,
        }
        .hash(&mut hasher);

        hasher.finish()
    };

    let struct2_hash = {
        let mut hasher = DefaultHasher::new();

        Struct2 {
            f1: 1,
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
    #[educe(Hash(bound))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(Hash(bound))]
    struct Tuple<T>(T);

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Struct {
            f1: 1,
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
            f1: 1,
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
    #[educe(Hash(bound("T: core::hash::Hash")))]
    struct Struct<T> {
        f1: T,
    }

    #[derive(Educe)]
    #[educe(Hash(bound("T: core::hash::Hash")))]
    struct Tuple<T>(T);

    let struct_hash = {
        let mut hasher = DefaultHasher::new();

        Struct {
            f1: 1,
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
