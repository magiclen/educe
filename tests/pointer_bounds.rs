#![cfg(any(feature = "Clone", feature = "Default"))]
#![no_std]
#![allow(dead_code)]

extern crate alloc;

use educe::Educe;

#[cfg(feature = "Clone")]
#[test]
fn clone_boxed_slices() {
    use alloc::boxed::Box;

    #[derive(Educe)]
    #[educe(Clone)]
    struct Struct<T>(Option<Box<[T]>>);

    #[derive(Educe)]
    #[educe(Clone)]
    enum Enum<T> {
        Values(Box<[T]>),
    }

    let value = Struct(Some(alloc::vec![1u8, 2].into_boxed_slice()));
    assert_eq!(Some(&[1u8, 2][..]), value.clone().0.as_deref());

    let value = Enum::Values(alloc::vec![3u8, 4].into_boxed_slice());
    let Enum::Values(value) = value.clone();
    assert_eq!(&[3u8, 4], value.as_ref());
}

#[cfg(feature = "Clone")]
#[test]
fn clone_maybe_unsized_pointees() {
    use alloc::boxed::Box;

    #[derive(Educe)]
    #[educe(Clone)]
    struct Inline<T: ?Sized>(Box<T>);

    #[derive(Educe)]
    #[educe(Clone)]
    struct Where<T>(Box<T>)
    where
        T: ?Sized;

    let text = Inline(Box::<str>::from("inline"));
    assert_eq!("inline", text.clone().0.as_ref());
    let text = Where(Box::<str>::from("where"));
    assert_eq!("where", text.clone().0.as_ref());
    let values = Where(alloc::vec![1u8, 2].into_boxed_slice());
    assert_eq!(&[1u8, 2], values.clone().0.as_ref());
}

#[cfg(feature = "Clone")]
#[test]
fn clone_forwarding_reaches_custom_methods() {
    use alloc::boxed::Box;
    use core::marker::PhantomData;

    fn clone_marker<T: Clone>(_: &PhantomData<T>) -> PhantomData<T> {
        PhantomData
    }

    #[derive(Educe)]
    #[educe(Clone)]
    struct Struct<T> {
        value:  Box<T>,
        #[educe(Clone(method = clone_marker::<T>))]
        marker: PhantomData<T>,
    }

    #[derive(Educe)]
    #[educe(Clone)]
    struct MaybeSized<T: ?Sized> {
        value:  Box<T>,
        #[educe(Clone(method = clone_marker::<T>))]
        marker: PhantomData<T>,
    }

    let value = Struct {
        value: Box::new(7u8), marker: PhantomData
    };
    assert_eq!(7, *value.clone().value);

    #[derive(Educe)]
    #[educe(Clone)]
    struct Slice<T> {
        values: Box<[T]>,
        #[educe(Clone(method = clone_marker::<T>))]
        marker: PhantomData<T>,
    }

    let value = MaybeSized {
        value: Box::new(8u8), marker: PhantomData
    };
    assert_eq!(8, *value.clone().value);

    let value = Slice {
        values: alloc::vec![9u8].into_boxed_slice(), marker: PhantomData
    };
    assert_eq!(&[9u8], value.clone().values.as_ref());
}

#[cfg(feature = "Default")]
#[test]
fn default_slice_pointers() {
    use alloc::{boxed::Box, rc::Rc, sync::Arc};
    use core::marker::PhantomData;

    struct NotDefault;

    #[derive(Educe)]
    #[educe(Default)]
    struct Struct<T> {
        boxed: Box<[T]>,
        rc:    Rc<[T]>,
        arc:   Arc<[T]>,
    }

    #[derive(Educe)]
    #[educe(Default)]
    enum Enum<T> {
        Values(Box<[T]>, Rc<[T]>, Arc<[T]>),
    }

    #[derive(Educe)]
    #[educe(Default)]
    struct Recursive<T> {
        children: Box<[Self]>,
        marker:   PhantomData<T>,
    }

    let value = Struct::<NotDefault>::default();
    assert!(value.boxed.is_empty());
    assert!(value.rc.is_empty());
    assert!(value.arc.is_empty());
    let Enum::Values(boxed, rc, arc) = Enum::<NotDefault>::default();
    assert!(boxed.is_empty());
    assert!(rc.is_empty());
    assert!(arc.is_empty());
    assert!(Recursive::<NotDefault>::default().children.is_empty());
}

#[cfg(feature = "Default")]
#[test]
fn default_maybe_unsized_pointees() {
    use alloc::{boxed::Box, rc::Rc, sync::Arc};

    struct NotDefault;

    #[derive(Educe)]
    #[educe(Default)]
    struct Inline<T: ?Sized>(Box<T>, Rc<T>, Arc<T>);

    #[derive(Educe)]
    #[educe(Default)]
    struct Where<T>(Box<T>, Rc<T>, Arc<T>)
    where
        T: ?Sized;

    let Inline(boxed, rc, arc) = Inline::<str>::default();
    assert_eq!("", boxed.as_ref());
    assert_eq!("", rc.as_ref());
    assert_eq!("", arc.as_ref());
    let Where(boxed, rc, arc) = Where::<str>::default();
    assert_eq!("", boxed.as_ref());
    assert_eq!("", rc.as_ref());
    assert_eq!("", arc.as_ref());
    let Where(boxed, rc, arc) = Where::<[NotDefault]>::default();
    assert!(boxed.is_empty());
    assert!(rc.is_empty());
    assert!(arc.is_empty());
}

#[cfg(feature = "Default")]
#[test]
fn default_forwarding_reaches_custom_expressions() {
    use alloc::{boxed::Box, rc::Rc, sync::Arc};
    use core::marker::PhantomData;

    fn marker<T: Default>() -> PhantomData<T> {
        PhantomData
    }

    #[derive(Educe)]
    #[educe(Default)]
    struct Boxed<T: ?Sized>(Box<T>, #[educe(Default(expression = marker::<T>()))] PhantomData<T>);

    #[derive(Educe)]
    #[educe(Default)]
    struct Shared<T>(Rc<T>, #[educe(Default(expression = marker::<T>()))] PhantomData<T>);

    #[derive(Educe)]
    #[educe(Default)]
    struct Atomic<T>(Arc<T>, #[educe(Default(expression = marker::<T>()))] PhantomData<T>);

    assert_eq!(0, *Boxed::<u8>::default().0);
    assert_eq!(0, *Shared::<u8>::default().0);
    assert_eq!(0, *Atomic::<u8>::default().0);
}

#[cfg(feature = "Default")]
#[test]
fn default_weak_requires_no_pointee_default() {
    use alloc::{rc, sync};
    use core::marker::PhantomData;

    struct NotDefault;

    #[derive(Educe)]
    #[educe(Default)]
    struct Inline<T: ?Sized>(rc::Weak<T>, sync::Weak<T>);

    #[derive(Educe)]
    #[educe(Default)]
    struct Where<T>(rc::Weak<T>, sync::Weak<T>)
    where
        T: ?Sized;

    #[derive(Educe)]
    #[educe(Default)]
    struct Recursive<T> {
        parent: rc::Weak<Recursive<T>>,
        marker: PhantomData<T>,
    }

    struct Tail<T: ?Sized>(T);

    #[derive(Educe)]
    #[educe(Default)]
    struct Nested<T: ?Sized>(rc::Weak<Tail<T>>);

    let Inline(rc, sync) = Inline::<NotDefault>::default();
    assert!(rc.upgrade().is_none());
    assert!(sync.upgrade().is_none());
    let Where(rc, sync) = Where::<NotDefault>::default();
    assert!(rc.upgrade().is_none());
    assert!(sync.upgrade().is_none());
    assert!(Recursive::<NotDefault>::default().parent.upgrade().is_none());
    assert!(Nested::<NotDefault>::default().0.upgrade().is_none());
}
