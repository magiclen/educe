#![cfg(any(feature = "Debug", feature = "Default", feature = "Hash"))]

use educe::Educe;

#[cfg(feature = "Default")]
#[test]
fn default_hasher_bound() {
    use std::collections::{HashMap, HashSet, hash_map::RandomState};

    struct NotDefault;

    #[derive(Educe)]
    #[educe(Default)]
    struct Struct<T, S> {
        map: HashMap<T, T, S>,
        set: HashSet<T, S>,
    }

    let value = Struct::<NotDefault, RandomState>::default();
    assert!(value.map.is_empty());
    assert!(value.set.is_empty());
}

#[cfg(feature = "Debug")]
#[test]
fn debug_without_hasher_bound() {
    use std::collections::{HashMap, HashSet};

    struct NotDebug;

    #[allow(dead_code)]
    #[derive(Educe)]
    #[educe(Debug)]
    struct Struct<T, S> {
        map: HashMap<T, T, S>,
        set: HashSet<T, S>,
    }

    let value = Struct::<u8, NotDebug> {
        map: HashMap::with_hasher(NotDebug),
        set: HashSet::with_hasher(NotDebug),
    };
    assert_eq!("Struct { map: {}, set: {} }", format!("{value:?}"));
}

#[cfg(feature = "Hash")]
#[test]
fn generic_names() {
    use std::hash::{DefaultHasher, Hash, Hasher};

    #[derive(Educe)]
    #[educe(Hash)]
    struct Struct<H, _H, const __H: usize>(H, _H);

    #[derive(Educe)]
    #[educe(Hash)]
    enum Enum<H> {
        Tuple(H),
    }

    #[allow(dead_code)]
    #[derive(Educe)]
    #[educe(Hash(unsafe))]
    union Union<H: Copy> {
        value: H,
    }

    fn hash(value: impl Hash) -> u64 {
        let mut state = DefaultHasher::new();
        value.hash(&mut state);
        state.finish()
    }

    assert_eq!(hash((1u8, 2u8)), hash(Struct::<_, _, 3>(1u8, 2u8)));
    assert_eq!(hash((0usize, 1u8)), hash(Enum::Tuple(1u8)));
    assert_eq!(
        hash([1u8]),
        hash(Union {
            value: 1u8
        })
    );
}

#[cfg(feature = "Hash")]
#[test]
fn method_type_names() {
    use std::hash::{DefaultHasher, Hash, Hasher};

    #[derive(Educe)]
    #[educe(Hash)]
    struct H(#[educe(Hash(method = H::custom_hash))] u8);

    impl H {
        fn custom_hash<S: Hasher>(value: &u8, state: &mut S) {
            value.hash(state);
        }
    }

    use H as _H;

    #[derive(Educe)]
    #[educe(Hash)]
    enum Enum {
        Value(
            #[educe(Hash(method = "_H::custom_hash"))] u8,
            #[educe(Hash(method = H::custom_hash))] u8,
        ),
    }

    fn hash(value: impl Hash) -> u64 {
        let mut state = DefaultHasher::new();
        value.hash(&mut state);
        state.finish()
    }

    assert_eq!(hash(7u8), hash(H(7)));
    assert_eq!(hash((0usize, 7u8, 9u8)), hash(Enum::Value(7, 9)));
}
