#![cfg(all(feature = "Clone", feature = "Copy"))]
#![no_std]
#![allow(dead_code, clippy::clone_on_copy)]

use core::cell::Cell;

use educe::Educe;

#[test]
fn conditional_lifetime() {
    #[derive(Educe)]
    #[educe(Clone, Copy(bound('a: 'static)))]
    struct Struct<'a> {
        value: &'a i32,
    }

    #[derive(Educe)]
    #[educe(Clone, Copy(bound = "'a: 'static"))]
    enum Enum<'a> {
        Tuple(&'a i32),
        Named { value: &'a i32 },
    }

    fn assert_copy<T: Copy>() {}
    assert_copy::<Struct<'static>>();
    assert_copy::<Enum<'static>>();

    let first = 1;
    let second = 2;
    let mut value = Struct {
        value: &first
    }
    .clone();
    value.clone_from(&Struct {
        value: &second
    });
    assert_eq!(2, *value.value);

    let mut value = Enum::Tuple(&first).clone();
    value.clone_from(&Enum::Tuple(&second));
    assert!(matches!(value, Enum::Tuple(&2)));
    value.clone_from(&Enum::Named {
        value: &first
    });
    value.clone_from(&Enum::Named {
        value: &second
    });
    assert!(matches!(value, Enum::Named {
        value: &2
    }));
}

#[test]
fn conditional_unused_const() {
    struct Check<const N: usize>;
    trait Enabled {}
    impl Enabled for Check<1> {}

    #[derive(Educe)]
    #[educe(Clone, Copy(bound(Check<N>: Enabled)))]
    struct Struct<const N: usize>(u8);

    #[derive(Educe)]
    #[educe(Clone, Copy(bound(Check<N>: Enabled)))]
    enum Enum<const N: usize> {
        Value(u8),
    }

    fn assert_copy<T: Copy>() {}
    assert_copy::<Struct<1>>();
    assert_copy::<Enum<1>>();

    let mut value = Struct::<0>(1).clone();
    value.clone_from(&Struct(2));
    assert_eq!(2, value.0);
    let mut value = Enum::<0>::Value(1).clone();
    value.clone_from(&Enum::Value(2));
    assert!(matches!(value, Enum::Value(2)));
}

#[derive(Copy)]
struct Tracked<'a>(&'a Cell<usize>);

// The counter makes the choice between a field clone and a whole-value copy visible.
#[allow(clippy::non_canonical_clone_impl)]
impl Clone for Tracked<'_> {
    fn clone(&self) -> Self {
        self.0.set(self.0.get() + 1);
        *self
    }

    fn clone_from(&mut self, source: &Self) {
        source.0.set(source.0.get() + 10);
        *self = *source;
    }
}

#[test]
fn automatic_copy_keeps_shortcut() {
    #[derive(Educe)]
    #[educe(Clone, Copy)]
    struct Struct<'a>(Tracked<'a>);

    #[derive(Educe)]
    #[educe(Clone, Copy)]
    enum Enum<'a> {
        Value(Tracked<'a>),
    }

    let count = Cell::new(0);
    let source = Struct(Tracked(&count));
    let mut value = source.clone();
    value.clone_from(&source);
    let source = Enum::Value(Tracked(&count));
    let mut value = source.clone();
    value.clone_from(&source);
    assert_eq!(0, count.get());
}

#[test]
fn custom_copy_uses_field_methods() {
    #[derive(Educe)]
    #[educe(Clone, Copy(bound('a: 'static)))]
    struct Struct<'a>(Tracked<'a>);

    #[derive(Educe)]
    #[educe(Clone, Copy(bound('a: 'static)))]
    enum Enum<'a> {
        Value(Tracked<'a>),
    }

    let count = Cell::new(0);
    let source = Struct(Tracked(&count));
    let mut value = source.clone();
    value.clone_from(&source);
    assert_eq!(11, count.get());
    let source = Enum::Value(Tracked(&count));
    let mut value = source.clone();
    value.clone_from(&source);
    assert_eq!(22, count.get());
}
