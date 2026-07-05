#![cfg(all(
    feature = "Debug",
    feature = "Clone",
    feature = "PartialEq",
    feature = "Eq",
    feature = "PartialOrd",
    feature = "Ord",
    feature = "Hash"
))]
#![no_std]

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};

use educe::Educe;

#[test]
fn recursive_enum() {
    #[derive(Educe)]
    #[educe(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum List<T> {
        Nil,
        Cons(T, Box<List<T>>),
    }

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));

    let cloned = list.clone();

    assert_eq!(list, cloned);
    assert!(list <= cloned);
    assert!(matches!(List::<u8>::Nil.cmp(&List::Nil), core::cmp::Ordering::Equal));
}

#[test]
fn recursive_struct() {
    #[derive(Educe)]
    #[educe(Debug, Clone, PartialEq)]
    struct Node<T> {
        value:    T,
        children: Vec<Node<T>>,
    }

    let node = Node {
        value:    1,
        children: alloc::vec![Node {
            value: 2, children: Vec::new()
        }],
    };

    assert_eq!(node, node.clone());
}
