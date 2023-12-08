#![cfg(all(feature = "PartialEq", feature = "Eq", feature = "Hash"))]

use std::collections::HashSet;

use educe::Educe;

#[allow(dead_code)]
#[test]
fn basic() {
    #[derive(Educe)]
    #[educe(PartialEq(unsafe), Eq, Hash(unsafe))]
    union Union {
        f1: u8,
    }

    let mut set = HashSet::new();

    set.insert(Union {
        f1: 0
    });

    assert!(set.contains(&Union {
        f1: 0
    }));

    assert!(!set.contains(&Union {
        f1: 1
    }));
}

#[allow(dead_code)]
#[test]
fn bound() {
    #[derive(Educe)]
    #[educe(PartialEq(unsafe), Eq, Hash(unsafe))]
    union Union<T: Copy> {
        f1: T,
    }

    let mut set = HashSet::new();

    set.insert(Union {
        f1: 0
    });

    assert!(set.contains(&Union {
        f1: 0
    }));

    assert!(!set.contains(&Union {
        f1: 1
    }));
}
