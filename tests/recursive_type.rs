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
fn nested_self() {
    #[derive(Educe)]
    #[educe(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Node<T>(T, Option<Box<(Self, T)>>);

    let node = Node(1, Some(Box::new((Node(2, None), 3))));
    let cloned = Clone::clone(&node);

    assert_eq!(node, cloned);
    assert_eq!(core::cmp::Ordering::Equal, node.cmp(&cloned));
}

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
    #[educe(Debug, Clone, PartialEq, Eq)]
    struct Node<T> {
        value:    T,
        children: Vec<Self>,
    }

    let node = Node {
        value:    1,
        children: alloc::vec![Node {
            value: 2, children: Vec::new()
        }],
    };

    assert_eq!(node, node.clone());
}
