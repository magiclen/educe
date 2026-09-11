/*!
# Educe

This crate offers procedural macros designed to facilitate the swift implementation of Rust's built-in traits.

## Features

By default, every trait this crate supports will be enabled. You can disable all of them by turning off the default features and enable only the traits that you want to use by adding them to the `features` explicitly.

For example,

```toml
[dependencies.educe]
version = "*"
features = ["Debug", "Clone", "Copy", "Hash", "Default"]
default-features = false
```

Besides the traits, there is one more feature, `full`, which is off by default. It widens the set of expressions that can be parsed in an attribute, and is only needed for the advanced expressions of `Default`; see the `Default` section below.

## Trait Bounds

When a trait is derived with Educe and no explicit `bound` is set, the where predicates of the generated impl are determined automatically. Every field type that the generated code touches (ignored fields and fields handled by a custom `method` are excluded) is processed with the following rules, in order:

1. A type that is known to implement the trait unconditionally produces no predicate at all. This covers `PhantomData`, raw pointers, and function pointers for every trait, shared references for `Clone` and `Copy`, plus the types in table A.
2. A type that does not use any generic type or const parameter produces no predicate, because such a predicate would be constant.
3. A std type that implements the trait whenever its type arguments do (table B) produces the predicates of its type arguments instead, with these rules applied recursively: a field of type `Option<T>` produces `T: Trait`, and one of type `Vec<Box<T>>` produces just `T: Clone` for `Clone`.
4. A type that mentions the derived type itself, by name or through `Self` (e.g. `Box<List<T>>` or `(Self, T)` inside `List<T>`) produces `Param: Trait` bounds for the type parameters it uses, because a self-referencing predicate would overflow the trait solver (E0275).
5. Any other type produces the precise predicate `FieldType: Trait`, so the compiler verifies the real requirement: a field of type `Wrapper<T>` where `Wrapper` has its own conditional `Clone` impl produces `Wrapper<T>: Clone`, which works for exactly the type arguments that `Wrapper` supports.

Table A — types whose type arguments never need a bound:

| Trait | Types |
|-------|-------|
| `Clone`, `Copy` | `Arc`, `Rc`, `Weak`, `NonNull`, `Cow`, `Discriminant` |
| `Debug` | `Weak`, `NonNull`, `AtomicPtr`, `Discriminant` |
| `PartialEq`, `Eq`, `Hash` | `NonNull`, `Discriminant` |
| `PartialOrd`, `Ord` | `NonNull` |
| `Default` | `Option`, `Vec`, `VecDeque`, `LinkedList`, `HashMap`, `HashSet`, `BTreeMap`, `BTreeSet`, `Weak` |

Table B — types that forward the trait to their type arguments:

| Trait | Types |
|-------|-------|
| `Clone` | `Option`, `Result`, `Box`, `Vec`, `VecDeque`, `LinkedList`, `BTreeMap`, `BTreeSet`, `BinaryHeap`, `HashMap`, `HashSet`, `RefCell`, `Wrapping`, `Reverse`, `Saturating` |
| `Copy` | `Option`, `Result`, `Wrapping`, `Reverse`, `Saturating` |
| `Debug` | `Option`, `Result`, `Box`, `Vec`, `VecDeque`, `LinkedList`, `BTreeMap`, `BTreeSet`, `BinaryHeap`, `HashMap`, `HashSet`, `Arc`, `Rc`, `RefCell`, `Mutex`, `RwLock`, `Wrapping`, `Reverse`, `Saturating` |
| `PartialEq`, `Eq`, `PartialOrd`, `Ord` | `Option`, `Result`, `Box`, `Vec`, `VecDeque`, `LinkedList`, `BTreeMap`, `BTreeSet`, `Arc`, `Rc`, `RefCell`, `Wrapping`, `Reverse`, `Saturating` |
| `Hash` | `Option`, `Result`, `Box`, `Vec`, `VecDeque`, `LinkedList`, `BTreeMap`, `BTreeSet`, `Arc`, `Rc`, `Wrapping`, `Reverse`, `Saturating` |
| `Default` | `Box`, `Arc`, `Rc`, `Cell`, `RefCell`, `Mutex`, `RwLock`, `Wrapping`, `Reverse`, `Saturating` |

`HashMap` and `HashSet` with an explicit hasher type use a whole-type predicate instead of either table. This preserves the hasher requirements of each trait: `Default` needs a default hasher, while `Debug` does not require the hasher to implement `Debug`.

`HashMap` and `HashSet` are not in the comparison rows of table B because their comparison impls additionally require `K: Eq + Hash`; such fields get the precise whole-type predicate from rule 5 instead.

Both tables match type names syntactically (by the last path segment), except for declared generic type parameters and paths that start with them, such as `T::PhantomData`. These parameters and associated types use their own trait requirements. Other user-defined types that share a name with a std type are still treated like that std type; if the resulting bounds do not fit, set them explicitly with `bound(...)`.

###### Bound Inheritance

When related traits are derived together with automatic bounds, a trait inherits the final predicates of its prerequisite traits: `Eq` and `PartialOrd` inherit from `PartialEq`, `Ord` inherits from `Eq` and `PartialOrd`, and `Copy` inherits from `Clone`. This way, a custom bound like `#[educe(PartialEq(bound(T: MyTrait)), Eq)]` automatically carries `T: MyTrait` into the `Eq` impl.

Educe cannot see the traits derived by other derive macros, including the built-in ones, so inheritance only applies between traits listed in the same `#[educe(...)]` attribute; a prerequisite trait implemented elsewhere contributes nothing.

When `PartialEq` and `Eq` are derived together, fields marked `PartialEq(ignore)` or `PartialEq(method = ...)` do not add automatic `Eq` bounds.
Explicit bounds are still used as written.

###### Controlling the Bounds

* `bound(where_predicates)` or `bound = "where_predicates"` uses exactly the given predicates, without inheritance.
* `bound(*)` adds `Param: Trait` for every generic type parameter, like the built-in derives.
* `bound(false)` adds no predicates at all.

An explicit bound is used verbatim; if a prerequisite impl carries predicates that the explicit bound does not imply, the compiler reports an unsatisfied supertrait and the missing predicates have to be added by hand.

###### Packed Types

A field of a `#[repr(packed)]` or `#[repr(packed(N))]` type cannot be borrowed where it lies, so the generated code copies each field it reads into a temporary first.
Every field that the generated code reads therefore has to implement `Copy`, and the automatic bound adds the matching `FieldType: Copy` predicates, just like the built-in derives do.
Any packing level is treated the same way, because a field's alignment cannot be worked out from the type syntax.

`Deref` and `DerefMut` return a reference to a field, so they can only be derived for a packed type whose target field does not need more alignment than the packing allows.

###### Limitations

* Mutually recursive generic types (an `A<T>` containing `Vec<B<T>>` while `B<T>` contains `A<T>`) cannot be detected from a single type definition, so automatic bounds make the trait solver overflow (E0275) on them; use `bound(*)` or a custom bound for such types.
* The precise predicates appear in the public where clause of the impl, so private field types become visible in documentation and error messages, and changing a private field type can change the public bounds of the impl.

Custom methods accept full paths, including qualified paths such as `<Type as Trait>::method::<T>`.
The `method = path`, `method(path)`, `method = "path"`, and `method("path")` forms are supported.
In custom method paths, `Self` refers to the type being derived, including its generic arguments.
For `Into`, this also applies when the generated implementation is `From` for the target type; use an explicit target type path to call a target method.
Generated primitive types use `::core::primitive` paths, and the `Debug` helper type names avoid identifiers in the input, including method paths written as strings.

## Traits

* [Debug](#debug)
* [Clone](#clone)
* [Copy](#copy)
* [PartialEq](#partialeq)
* [Eq](#eq)
* [PartialOrd](#partialord)
* [Ord](#ord)
* [Hash](#hash)
* [Default](#default)
* [Deref](#deref)
* [DerefMut](#derefmut)
* [Into](#into)

#### Debug

Use `#[derive(Educe)]` and `#[educe(Debug)]` to implement the `Debug` trait for a struct, enum, or union. This allows you to modify the names of your types, variants, and fields. You can also choose to ignore specific fields or set a method to replace the `Debug` trait. Additionally, you have the option to format a struct as a tuple and vice versa.

###### Basic Usage

```rust
# #[cfg(feature = "Debug")]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(Debug)]
struct Struct {
    f1: u8
}

#[derive(Educe)]
#[educe(Debug)]
enum Enum {
    V1,
    V2 {
        f1: u8,
    },
    V3(u8),
}
# }
```

###### Change the Name of a Type, a Variant or a Field

The `name` parameter can rename a type, a variant or a field. If you set it to `false`, the name can be ignored or forced to show otherwise.

```rust
# #[cfg(feature = "Debug")]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(Debug(name(Struct2)))]
struct Struct {
    #[educe(Debug(name(f)))]
    f1: u8
}

#[derive(Educe)]
#[educe(Debug(name = true))]
enum Enum {
    #[educe(Debug(name = false))]
    V1,
    #[educe(Debug(name(V)))]
    V2 {
        #[educe(Debug(name(f)))]
        f1: u8,
    },
    #[educe(Debug(name = false))]
    V3(u8),
}
# }
```

###### Ignore Fields

The `ignore` parameter can ignore a specific field.

```rust
# #[cfg(feature = "Debug")]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(Debug)]
struct Struct {
    #[educe(Debug(ignore))]
    f1: u8
}

#[derive(Educe)]
#[educe(Debug)]
enum Enum {
    V1,
    V2 {
        #[educe(Debug(ignore))]
        f1: u8,
    },
    V3(
        #[educe(Debug(ignore))]
        u8
    ),
}
# }
```

###### Fake Structs and Tuples

With the `named_field` parameter, structs can be formatted as tuples and tuples can be formatted as structs.

```rust
# #[cfg(feature = "Debug")]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(Debug(named_field = false))]
struct Struct {
    f1: u8
}

#[derive(Educe)]
#[educe(Debug)]
enum Enum {
    V1,
    #[educe(Debug(named_field = false))]
    V2 {
        f1: u8,
    },
    #[educe(Debug(named_field = true))]
    V3(
        u8,
        #[educe(Debug(name(value)))]
        i32
    ),
}
# }
```

###### Use Another Method to Handle the Formatting

The `method` parameter can be utilized to replace the implementation of the `Debug` trait for a field, eliminating the need to implement the `Debug` trait for the type of that field.

```rust
# #[cfg(feature = "Debug")]
# {
use educe::Educe;

use std::fmt::{self, Formatter};

fn fmt<T>(_s: &T, f: &mut Formatter<'_>) -> fmt::Result {
    f.write_str("Hi")
}

#[derive(Educe)]
#[educe(Debug)]
enum Enum<T> {
    V1,
    V2 {
        #[educe(Debug(method(fmt)))]
        f1: u8,
    },
    V3(
        #[educe(Debug(method(std::fmt::UpperHex::fmt)))]
        u8,
        #[educe(Debug(method(fmt)))]
        T
    ),
}
# }
```

###### Generic Parameters Bound to the `Debug` Trait or Others

The where predicates of the generated impl are determined from the field types automatically; see the "Trait Bounds" section above for the exact rules.

```rust
# #[cfg(feature = "Debug")]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(Debug)]
enum Enum<T, K> {
    V1,
    V2 {
        f1: K,
    },
    V3(
        T
    ),
}
# }
```

Or you can set the where predicates by yourself.

```rust
# #[cfg(feature = "Debug")]
# {
use educe::Educe;

use std::fmt::{self, Formatter};

fn fmt<D>(_s: &D, f: &mut Formatter<'_>) -> fmt::Result {
    f.write_str("Hi")
}

#[derive(Educe)]
#[educe(Debug(bound(T: std::fmt::Debug)))]
enum Enum<T, K> {
    V1,
    V2 {
        #[educe(Debug(method(fmt)))]
        f1: K,
    },
    V3(
        T
    ),
}
# }
```

In the above case, `T` is bound to the `Debug` trait, but `K` is not.

###### Union

A union is formatted as a `u8` slice because its active field is not tracked at runtime. Its fields cannot be ignored, renamed, or formatted with custom methods. `Debug(unsafe)` requires every byte of the union to be initialized and readable during each call, including padding and bytes outside the active field. The storage must not change during a call. This condition must hold after construction, writes, moves, and copies. Reading uninitialized bytes is undefined behavior, not just a risk of exposing memory. Initializing one field, or zeroing storage before moving the value, does not by itself guarantee this condition.

```rust
# #[cfg(feature = "Debug")]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(Debug(unsafe))]
union Union {
    f1: u8,
    f2: i32,
}
# }
```

#### Clone

When `Clone` and `Copy` are derived together, Educe can copy the whole value if no field uses a type or const parameter and no field has a custom clone method.
For a struct or enum with any generic parameters, a nonempty custom `Copy(bound(...))` also disables this shortcut.
In that case, `clone` and `clone_from` use the existing field-wise implementation, while the custom `Copy` conditions are not added to `Clone`.
For example, `Copy(bound('a: 'static))` on `S<'a>(&'a i32)` allows `Copy` only for `'static`, but `Clone` still works with shorter lifetimes.
This rule also covers const parameters that do not appear in the fields.
Field clone methods can have visible side effects, and the performance difference has not been measured.
The automatic, `bound(*)`, `bound(false)`, and empty custom `Copy` modes keep the original shortcut rules; unions keep their existing clone behavior.


Use `#[derive(Educe)]` and `#[educe(Clone)]` to implement the `Clone` trait for a struct, an enum, or a union. You can set a method to replace the `Clone` trait.

###### Basic Usage

```rust
# #[cfg(feature = "Clone")]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(Clone)]
struct Struct {
    f1: u8
}

#[derive(Educe)]
#[educe(Clone)]
enum Enum {
    V1,
    V2 {
        f1: u8,
    },
    V3(u8),
}
# }
```

###### Use Another Method to Perform Cloning

The `method` parameter can be utilized to replace the implementation of the `Clone` trait for a field, eliminating the need to implement the `Clone` trait for the type of that field.

```rust
# #[cfg(feature = "Clone")]
# {
use educe::Educe;

fn clone(v: &u8) -> u8 {
    v + 100
}

trait A {
    fn add(&self, rhs: u8) -> Self;
}

fn clone2<T: A>(v: &T) -> T {
    v.add(100)
}

#[derive(Educe)]
#[educe(Clone)]
enum Enum<T: A> {
    V1,
    V2 {
        #[educe(Clone(method(clone)))]
        f1: u8,
    },
    V3(
        #[educe(Clone(method(clone2)))]
        T
    ),
}
# }
```

###### Generic Parameters Bound to the `Clone` Trait or Others

The where predicates of the generated impl are determined from the field types automatically; see the "Trait Bounds" section above for the exact rules.

```rust
# #[cfg(feature = "Clone")]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(Clone)]
enum Enum<T, K> {
    V1,
    V2 {
        f1: K,
    },
    V3(
        T
    ),
}
# }
```

Or you can set the where predicates by yourself.

```rust
# #[cfg(feature = "Clone")]
# {
use educe::Educe;

trait A {
    fn add(&self, rhs: u8) -> Self;
}

fn clone<T: A>(v: &T) -> T {
    v.add(100)
}

#[derive(Educe)]
#[educe(Clone(bound(T: std::clone::Clone)))]
enum Enum<T, K: A> {
    V1,
    V2 {
        #[educe(Clone(method(clone)))]
        f1: K,
    },
    V3(
        T
    ),
}
# }
```

In the above case, `T` is bound to the `Clone` trait, but `K` is not.

###### Union

Refer to the introduction of the `#[educe(Copy)]` attribute.

#### Copy

Use `#[derive(Educe)]` and `#[educe(Copy)]` to implement the `Copy` trait for a struct, an enum, or a union.

###### Basic Usage

```rust
# #[cfg(all(feature = "Clone", feature = "Copy"))]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(Copy, Clone)]
struct Struct {
    f1: u8
}

#[derive(Educe)]
#[educe(Copy, Clone)]
enum Enum {
    V1,
    V2 {
        f1: u8,
    },
    V3(u8),
}
# }
```

###### Generic Parameters Bound to the `Copy` Trait or Others

The where predicates of the generated impl are determined from the field types automatically; see the "Trait Bounds" section above for the exact rules. With automatic bounds, the `Copy` impl additionally inherits the predicates of the `Clone` impl generated by Educe, because `Copy` requires `Clone`.

```rust
# #[cfg(all(feature = "Clone", feature = "Copy"))]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(Copy, Clone)]
enum Enum<T, K> {
    V1,
    V2 {
        f1: K,
    },
    V3(
        T
    ),
}
# }
```

Or you can set the where predicates by yourself.

```rust
# #[cfg(all(feature = "Clone", feature = "Copy"))]
# {
use educe::Educe;

trait A {
    fn add(&self, rhs: u8) -> Self;
}

fn clone<T: A>(v: &T) -> T {
    v.add(100)
}

#[derive(Educe)]
#[educe(Copy, Clone(bound(T: Copy, K: A + Copy)))]
enum Enum<T, K> {
    V1,
    V2 {
        #[educe(Clone(method(clone)))]
        f1: K,
    },
    V3(
        T
    ),
}
# }
```

Note that utilizing custom cloning methods for a type that implements the `Copy` and `Clone` traits may not be entirely appropriate.

###### Union

The `#[educe(Copy, Clone)]` attribute can be used for a union. The fields of a union cannot be cloned with other methods.

```rust
# #[cfg(all(feature = "Clone", feature = "Copy"))]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(Copy, Clone)]
union Union {
    f1: u8,
}
# }
```

#### PartialEq

Use `#[derive(Educe)]` and `#[educe(PartialEq)]` to implement the `PartialEq` trait for a struct, enum, or union. You can also choose to ignore specific fields or set a method to replace the `PartialEq` trait.

###### Basic Usage

```rust
# #[cfg(feature = "PartialEq")]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(PartialEq)]
struct Struct {
    f1: u8
}

#[derive(Educe)]
#[educe(PartialEq)]
enum Enum {
    V1,
    V2 {
        f1: u8,
    },
    V3(u8),
}
# }
```

###### Ignore Fields

The `ignore` parameter can ignore a specific field.

```rust
# #[cfg(feature = "PartialEq")]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(PartialEq)]
struct Struct {
    #[educe(PartialEq(ignore))]
    f1: u8
}

#[derive(Educe)]
#[educe(PartialEq)]
enum Enum {
    V1,
    V2 {
        #[educe(PartialEq(ignore))]
        f1: u8,
    },
    V3(
        #[educe(PartialEq(ignore))]
        u8
    ),
}
# }
```

###### Use Another Method to Perform Comparison

The `method` parameter can be utilized to replace the implementation of the `PartialEq` trait for a field, eliminating the need to implement the `PartialEq` trait for the type of that field.

```rust
# #[cfg(feature = "PartialEq")]
# {
use educe::Educe;

fn eq(a: &u8, b: &u8) -> bool {
    a + 1 == *b
}

trait A {
    fn is_same(&self, other: &Self) -> bool;
}

fn eq2<T: A>(a: &T, b: &T) -> bool {
    a.is_same(b)
}

#[derive(Educe)]
#[educe(PartialEq)]
enum Enum<T: A> {
    V1,
    V2 {
        #[educe(PartialEq(method(eq)))]
        f1: u8,
    },
    V3(
        #[educe(PartialEq(method(eq2)))]
        T
    ),
}
# }
```

###### Generic Parameters Bound to the `PartialEq` Trait or Others

The where predicates of the generated impl are determined from the field types automatically; see the "Trait Bounds" section above for the exact rules.

```rust
# #[cfg(feature = "PartialEq")]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(PartialEq)]
enum Enum<T, K> {
    V1,
    V2 {
        f1: K,
    },
    V3(
        T
    ),
}
# }
```

Or you can set the where predicates by yourself.

```rust
# #[cfg(feature = "PartialEq")]
# {
use educe::Educe;

trait A {
    fn is_same(&self, other: &Self) -> bool;
}

fn eq<T: A>(a: &T, b: &T) -> bool {
    a.is_same(b)
}

#[derive(Educe)]
#[educe(PartialEq(bound(T: std::cmp::PartialEq, K: A)))]
enum Enum<T, K> {
    V1,
    V2 {
        #[educe(PartialEq(method(eq)))]
        f1: K,
    },
    V3(
        T
    ),
}
# }
```

###### Union

The `#[educe(PartialEq(unsafe))]` attribute compares the entire storage of two unions as bytes. Custom field methods are not supported. Every byte must be initialized and readable during each comparison, including padding and bytes outside the active field. The storage must not change during a call. This condition must hold after construction, writes, moves, and copies. Otherwise the comparison has undefined behavior. The `unsafe` attribute requires the user to uphold this contract; it does not make uninitialized reads valid.

```rust
# #[cfg(feature = "PartialEq")]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(PartialEq(unsafe))]
union Union {
    f1: u8,
    f2: i32
}
# }
```

#### Eq

Use `#[derive(Educe)]` and `#[educe(Eq)]` to implement the `Eq` trait for a struct, enum, or union. `Eq` is a marker trait, so it has no field attributes of its own; field-level equality settings such as `ignore` and `method` belong to the `PartialEq` attribute.

With automatic bounds, every ordinary struct or enum field must implement `Eq`, even when its type is concrete, such as `f64`.
This check uses the final impl bounds, adds no runtime calls, and does not put concrete field requirements in the public where clause.
When Educe also derives `PartialEq`, fields with `PartialEq(ignore)` or `PartialEq(method = ...)` are excluded from this check and from automatic `Eq` bounds.
The author must ensure that custom comparison methods form an equivalence relation: reflexive, symmetric, and transitive.

Educe cannot inspect an external `PartialEq` implementation, so all fields are treated as ordinary fields in that case.
If a manual implementation provides an equivalence relation without requiring every field to implement `Eq`, use `Eq(bound(false))`, `Eq(bound(*))`, or custom predicates.
These explicit modes and unions keep their existing behavior and do not perform the automatic field check; the author is responsible for the equality contract.

###### Basic Usage

```rust
# #[cfg(all(feature = "PartialEq", feature = "Eq"))]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(PartialEq, Eq)]
struct Struct {
    f1: u8
}

#[derive(Educe)]
#[educe(PartialEq, Eq)]
enum Enum {
    V1,
    V2 {
        f1: u8,
    },
    V3(u8),
}
# }
```

###### Generic Parameters Bound to the `Eq` Trait or Others

The where predicates of the generated impl are determined from the field types automatically; see the "Trait Bounds" section above for the exact rules. With automatic bounds, the `Eq` impl also inherits the predicates of the `PartialEq` impl generated by Educe.

```rust
# #[cfg(all(feature = "PartialEq", feature = "Eq"))]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(PartialEq, Eq)]
enum Enum<T, K> {
    V1,
    V2 {
        f1: K,
    },
    V3(
        T
    ),
}
# }
```

Or you can set the where predicates by yourself.

```rust
# #[cfg(all(feature = "PartialEq", feature = "Eq"))]
# {
use educe::Educe;

trait A {
    fn is_same(&self, other: &Self) -> bool;
}

fn eq<T: A>(a: &T, b: &T) -> bool {
    a.is_same(b)
}

#[derive(Educe)]
#[educe(
    PartialEq(bound(T: std::cmp::PartialEq, K: A)),
    Eq(bound(T: std::cmp::Eq, K: A))
)]
enum Enum<T, K> {
    V1,
    V2 {
        #[educe(PartialEq(method(eq)))]
        f1: K,
    },
    V3(
        T
    ),
}
# }
```

###### Union

The `#[educe(PartialEq(unsafe), Eq)]` attribute compares the entire storage of two unions as bytes, without tracking their active fields. Custom field methods are not supported. Every byte must be initialized and readable during each comparison, including padding and bytes outside the active field. The storage must not change during a call. This condition must hold after construction, writes, moves, and copies; otherwise the comparison has undefined behavior.

```rust
# #[cfg(all(feature = "PartialEq", feature = "Eq"))]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(PartialEq(unsafe), Eq)]
union Union {
    f1: u8,
    f2: i32
}
# }
```

#### PartialOrd

Use `#[derive(Educe)]` and `#[educe(PartialOrd)]` to implement the `PartialOrd` trait for a struct or enum. You can also choose to ignore specific fields or set a method to replace the `PartialOrd` trait.

###### Basic Usage

```rust
# #[cfg(feature = "PartialOrd")]
# {
use educe::Educe;

#[derive(PartialEq, Educe)]
#[educe(PartialOrd)]
struct Struct {
    f1: u8
}

#[derive(PartialEq, Educe)]
#[educe(PartialOrd)]
enum Enum {
    V1,
    V2 {
        f1: u8,
    },
    V3(u8),
}
# }
```

###### Ignore Fields

The `ignore` parameter can ignore a specific field.

```rust
# #[cfg(feature = "PartialOrd")]
# {
use educe::Educe;

#[derive(PartialEq, Educe)]
#[educe(PartialOrd)]
struct Struct {
    #[educe(PartialOrd(ignore))]
    f1: u8
}

#[derive(PartialEq, Educe)]
#[educe(PartialOrd)]
enum Enum {
    V1,
    V2 {
        #[educe(PartialOrd(ignore))]
        f1: u8,
    },
    V3(
        #[educe(PartialOrd(ignore))]
        u8
    ),
}
# }
```

###### Use Another Method to Perform Comparison

The `method` parameter can be utilized to replace the implementation of the `PartialOrd` trait for a field, eliminating the need to implement the `PartialOrd` trait for the type of that field.

When `Ord` is derived together, a field without its own `PartialOrd` attribute follows the `ignore`, `rank`, and `method` settings of its `Ord` attribute, so `partial_cmp` stays consistent with `cmp`; the result of an `Ord` comparison method is wrapped in `Some` automatically.

```rust
# #[cfg(all(feature = "PartialEq", feature = "PartialOrd"))]
# {
use educe::Educe;

use std::cmp::Ordering;

fn partial_cmp(a: &u8, b: &u8) -> Option<Ordering> {
    if a > b {
        Some(Ordering::Less)
    } else if a < b {
        Some(Ordering::Greater)
    } else {
        Some(Ordering::Equal)
    }
}

trait A {
    fn value(&self) -> u8;
}

fn partial_cmp2<T: A>(a: &T, b: &T) -> Option<Ordering> {
    partial_cmp(&a.value(), &b.value())
}

#[derive(Educe)]
#[educe(PartialEq, PartialOrd)]
enum Enum<T: A> {
    V1,
    V2 {
        #[educe(PartialOrd(method(partial_cmp)))]
        f1: u8,
    },
    V3(
        #[educe(PartialOrd(method(partial_cmp2)))]
        T
    ),
}
# }
```

###### Ranking

Each field can add a `#[educe(PartialOrd(rank = priority_value))]` attribute, where `priority_value` is an integer value indicating its comparison precedence (lower values indicate higher priority). The default `priority_value` for a field depends on its ordinal position (lower towards the front) and starts with `isize::MIN`.

```rust
# #[cfg(feature = "PartialOrd")]
# {
use educe::Educe;

#[derive(PartialEq, Educe)]
#[educe(PartialOrd)]
struct Struct {
    #[educe(PartialOrd(rank = 1))]
    f1: u8,
    #[educe(PartialOrd(rank = 0))]
    f2: u8,
}
# }
```

For variants, the discriminant can be explicitly set for comparison. Constant expressions are evaluated by Rust, and integer representations retain their full signed or unsigned range. Alignment settings do not affect the comparison order.

```rust
# #[cfg(feature = "PartialOrd")]
# {
use educe::Educe;

#[derive(PartialEq, Educe)]
#[educe(PartialOrd)]
#[repr(u8)]
enum Enum {
    Three { f1: u8 } = 3,
    Two(u8) = 2,
    One = 1,
}
# }
```

###### Generic Parameters Bound to the `PartialOrd` Trait or Others

The where predicates of the generated impl are determined from the field types automatically; see the "Trait Bounds" section above for the exact rules. With automatic bounds, the `PartialOrd` impl also inherits the predicates of the `PartialEq` impl generated by Educe.

```rust
# #[cfg(feature = "PartialOrd")]
# {
use educe::Educe;

#[derive(PartialEq, Educe)]
#[educe(PartialOrd)]
enum Enum<T, K> {
    V1,
    V2 {
        f1: K,
    },
    V3(
        T
    ),
}
# }
```

Or you can set the where predicates by yourself.

```rust
# #[cfg(feature = "PartialOrd")]
# {
use educe::Educe;

use std::cmp::Ordering;

trait A {
    fn value(&self) -> u8;
}

fn partial_cmp<T: A>(a: &T, b: &T) -> Option<Ordering> {
    a.value().partial_cmp(&b.value())
}

#[derive(PartialEq, Educe)]
#[educe(PartialOrd(bound(T: std::cmp::PartialOrd, K: std::cmp::PartialOrd + A)))]
enum Enum<T, K> {
    V1,
    V2 {
        #[educe(PartialOrd(method(partial_cmp)))]
        f1: K,
    },
    V3(
        T
    ),
}
# }
```

#### Ord

Use `#[derive(Educe)]` and `#[educe(Ord)]` to implement the `Ord` trait for a struct or enum. You can also choose to ignore specific fields or set a method to replace the `Ord` trait.

###### Basic Usage

```rust
# #[cfg(all(feature = "PartialOrd", feature = "Ord"))]
# {
use educe::Educe;

#[derive(PartialEq, Eq, Educe)]
#[educe(PartialOrd, Ord)]
struct Struct {
    f1: u8
}

#[derive(PartialEq, Eq, Educe)]
#[educe(PartialOrd, Ord)]
enum Enum {
    V1,
    V2 {
        f1: u8,
    },
    V3(u8),
}
# }
```

###### Ignore Fields

The `ignore` parameter can ignore a specific field.

```rust
# #[cfg(all(feature = "PartialOrd", feature = "Ord"))]
# {
use educe::Educe;

#[derive(PartialEq, Eq, Educe)]
#[educe(PartialOrd, Ord)]
struct Struct {
    #[educe(Ord(ignore))]
    f1: u8
}

#[derive(PartialEq, Eq, Educe)]
#[educe(PartialOrd, Ord)]
enum Enum {
    V1,
    V2 {
        #[educe(Ord(ignore))]
        f1: u8,
    },
    V3(
        #[educe(Ord(ignore))]
        u8
    ),
}
# }
```

###### Use Another Method to Perform Comparison

The `method` parameter can be utilized to replace the implementation of the `Ord` trait for a field, eliminating the need to implement the `Ord` trait for the type of that field.

When `PartialOrd` is derived together, a field without its own `Ord` attribute follows the `ignore` and `rank` settings of its `PartialOrd` attribute; a `PartialOrd` comparison method returns an `Option<Ordering>` and cannot be used by `cmp`, so such a field is compared with the built-in comparison.

```rust
# #[cfg(all(feature = "PartialEq", feature = "Eq", feature = "PartialOrd", feature = "Ord"))]
# {
use educe::Educe;

use std::cmp::Ordering;

fn cmp(a: &u8, b: &u8) -> Ordering {
    if a > b {
        Ordering::Less
    } else if a < b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

trait A {
    fn value(&self) -> u8;
}

fn cmp2<T: A>(a: &T, b: &T) -> Ordering {
    cmp(&a.value(), &b.value())
}

#[derive(Educe)]
#[educe(PartialEq, Eq, PartialOrd, Ord)]
enum Enum<T: A> {
    V1,
    V2 {
        #[educe(Ord(method(cmp)))]
        f1: u8,
    },
    V3(
        #[educe(Ord(method(cmp2)))]
        T
    ),
}
# }
```

###### Ranking

Each field can add a `#[educe(Ord(rank = priority_value))]` attribute, where `priority_value` is an integer value indicating its comparison precedence (lower values indicate higher priority). The default `priority_value` for a field depends on its ordinal position (lower towards the front) and starts with `isize::MIN`.

```rust
# #[cfg(all(feature = "PartialOrd", feature = "Ord"))]
# {
use educe::Educe;

#[derive(PartialEq, Eq, Educe)]
#[educe(PartialOrd, Ord)]
struct Struct {
    #[educe(Ord(rank = 1))]
    f1: u8,
    #[educe(Ord(rank = 0))]
    f2: u8,
}
# }
```

For variants, the discriminant can be explicitly set for comparison. Constant expressions are evaluated by Rust, and integer representations retain their full signed or unsigned range. Alignment settings do not affect the comparison order.

```rust
# #[cfg(all(feature = "PartialOrd", feature = "Ord"))]
# {
use educe::Educe;

#[derive(PartialEq, Eq, Educe)]
#[educe(PartialOrd, Ord)]
#[repr(u8)]
enum Enum {
    Three { f1: u8 } = 3,
    Two(u8) = 2,
    One = 1,
}
# }
```

###### Generic Parameters Bound to the `Ord` Trait or Others

The where predicates of the generated impl are determined from the field types automatically; see the "Trait Bounds" section above for the exact rules. With automatic bounds, the `Ord` impl also inherits the predicates of the `Eq` and `PartialOrd` impls generated by Educe.

```rust
# #[cfg(all(feature = "PartialOrd", feature = "Ord"))]
# {
use educe::Educe;

#[derive(PartialEq, Eq, Educe)]
#[educe(PartialOrd, Ord)]
enum Enum<T, K> {
    V1,
    V2 {
        f1: K,
    },
    V3(
        T
    ),
}
# }
```

Or you can set the where predicates by yourself.

```rust
# #[cfg(all(feature = "PartialOrd", feature = "Ord"))]
# {
use educe::Educe;

use std::cmp::Ordering;

trait A {
    fn value(&self) -> u8;
}

fn cmp<T: A>(a: &T, b: &T) -> Ordering {
    a.value().cmp(&b.value())
}

#[derive(PartialEq, Eq, Educe)]
#[educe(
    PartialOrd(bound(T: std::cmp::PartialOrd, K: std::cmp::PartialEq + A)),
    Ord(bound(T: std::cmp::Ord, K: std::cmp::Ord + A))
)]
enum Enum<T, K> {
    V1,
    V2 {
        #[educe(Ord(method(cmp)))]
        f1: K,
    },
    V3(
        T
    ),
}
# }
```

#### Hash

Use `#[derive(Educe)]` and `#[educe(Hash)]` to implement the `Hash` trait for a struct, enum, or union. You can also choose to ignore specific fields or set a method to replace the `Hash` trait.

###### Basic Usage

```rust
# #[cfg(feature = "Hash")]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(Hash)]
struct Struct {
    f1: u8
}

#[derive(Educe)]
#[educe(Hash)]
enum Enum {
    V1,
    V2 {
        f1: u8,
    },
    V3(u8),
}
# }
```

###### Ignore Fields

The `ignore` parameter can ignore a specific field.

```rust
# #[cfg(feature = "Hash")]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(Hash)]
struct Struct {
    #[educe(Hash(ignore))]
    f1: u8
}

#[derive(Educe)]
#[educe(Hash)]
enum Enum {
    V1,
    V2 {
        #[educe(Hash(ignore))]
        f1: u8,
    },
    V3(
        #[educe(Hash(ignore))]
        u8
    ),
}
# }
```

###### Use Another Method for Hashing

The `method` parameter can be utilized to replace the implementation of the `Hash` trait for a field, eliminating the need to implement the `Hash` trait for the type of that field.

```rust
# #[cfg(feature = "Hash")]
# {
use educe::Educe;

use std::hash::{Hash, Hasher};

fn hash<H: Hasher>(_s: &u8, state: &mut H) {
    Hash::hash(&100, state)
}

fn hash2<H: Hasher, T>(_s: &T, state: &mut H) {
    Hash::hash(&100, state)
}

#[derive(Educe)]
#[educe(Hash)]
enum Enum<T> {
    V1,
    V2 {
        #[educe(Hash(method(hash)))]
        f1: u8,
    },
    V3(
        #[educe(Hash(method(hash2)))]
        T
    ),
}
# }
```

###### Generic Parameters Bound to the `Hash` Trait or Others

The where predicates of the generated impl are determined from the field types automatically; see the "Trait Bounds" section above for the exact rules.

```rust
# #[cfg(feature = "Hash")]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(Hash)]
enum Enum<T, K> {
    V1,
    V2 {
        f1: K,
    },
    V3(
        T
    ),
}
# }
```

Or you can set the where predicates by yourself.

```rust
# #[cfg(feature = "Hash")]
# {
use educe::Educe;

use std::hash::{Hash, Hasher};

trait A {
    fn value(&self) -> u8;
}

fn hash<H: Hasher, T: A>(s: &T, state: &mut H) {
    Hash::hash(&s.value(), state)
}

#[derive(Educe)]
#[educe(Hash(bound(T: std::hash::Hash, K: A)))]
enum Enum<T, K> {
    V1,
    V2 {
        #[educe(Hash(method(hash)))]
        f1: K,
    },
    V3(
        T
    ),
}
# }
```

###### Union

The `#[educe(PartialEq(unsafe), Eq, Hash(unsafe))]` attribute compares and hashes the entire storage of a union as bytes, without tracking its active field. Custom field methods are not supported. Every byte must be initialized and readable during each call, including padding and bytes outside the active field. The storage must not change during a call. This condition must hold after construction, writes, moves, and copies; otherwise these operations have undefined behavior.

```rust
# #[cfg(all(feature = "PartialEq", feature = "Eq", feature = "Hash"))]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(PartialEq(unsafe), Eq, Hash(unsafe))]
union Union {
    f1: u8,
    f2: i32
}
# }
```

#### Default

Use `#[derive(Educe)]` and `#[educe(Default)]` to implement the `Default` trait for a struct, enum, or union.
You can set a default expression for the entire type or for individual fields.
For an enum, mark the default variant with `#[educe(Default)]`; for a union, mark the field to initialize.
Fields without a custom value use their type's `Default` implementation.

###### Basic Usage

For enums and unions, it is necessary to designate a default variant (for enums) and a default field (for unions) unless the enum has only one variant or the union has only one field.

```rust
# #[cfg(feature = "Default")]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(Default)]
struct Struct {
    f1: u8
}

#[derive(Educe)]
#[educe(Default)]
enum Enum {
    V1,
    #[educe(Default)]
    V2 {
        f1: u8,
    },
    V3(u8),
}

#[derive(Educe)]
#[educe(Default)]
union Union {
    f1: u8,
    #[educe(Default)]
    f2: f64,
}
# }
```

###### The Default Value for the Entire Type

```rust
# #[cfg(feature = "Default")]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(Default(expression = Struct { f1: 1 }))]
struct Struct {
    f1: u8
}

#[derive(Educe)]
#[educe(Default(expression = Enum::Struct { f1: 1 }))]
enum Enum {
    Unit,
    Struct {
        f1: u8
    },
    Tuple(u8),
}

#[derive(Educe)]
#[educe(Default(expression = Union { f1: 1 }))]
union Union {
    f1: u8,
    f2: f64,
}
# }
```

You may need to activate the `full` feature to enable support for advanced expressions.

Note that the expression is pasted into the generated `default` method verbatim, so for a generic type it has to be valid for every possible instantiation; an expression producing a concrete type does not work for a generic field.

###### The Default Values for Specific Fields

```rust
# #[cfg(feature = "Default")]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(Default)]
struct Struct {
    #[educe(Default = 1)]
    f1: u8,
    #[educe(Default = 11111111111111111111111111111)]
    f2: i128,
    #[educe(Default = 1.1)]
    f3: f64,
    #[educe(Default = true)]
    f4: bool,
    #[educe(Default = "Hi")]
    f5: &'static str,
    #[educe(Default = "Hello")]
    f6: String,
    #[educe(Default = 'M')]
    f7: char,
}

#[derive(Educe)]
#[educe(Default)]
enum Enum {
    Unit,
    #[educe(Default)]
    Tuple(
        #[educe(Default(expression = 0 + 1))]
        u8,
        #[educe(Default(expression = -11111111111111111111111111111 * -1))]
        i128,
        #[educe(Default(expression = 1.0 + 0.1))]
        f64,
        #[educe(Default(expression = !false))]
        bool,
        #[educe(Default(expression = "Hi"))]
        &'static str,
        #[educe(Default(expression = String::from("Hello")))]
        String,
        #[educe(Default(expression = 'M'))]
        char,
    ),
}

#[derive(Educe)]
#[educe(Default)]
union Union {
    f1: u8,
    f2: i128,
    f3: f64,
    f4: bool,
    #[educe(Default = "Hi")]
    f5: &'static str,
    f6: char,
}
# }
```

###### Generic Parameters Bound to the `Default` Trait or Others

The where predicates of the generated impl are determined from the field types automatically; see the "Trait Bounds" section above for the exact rules.

```rust
# #[cfg(feature = "Default")]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(Default)]
enum Enum<T> {
    Unit,
    #[educe(Default)]
    Struct {
        f1: T
    },
    Tuple(T),
}
# }
```

Or you can set the where predicates by yourself.

```rust
# #[cfg(feature = "Default")]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(Default(bound(T: std::default::Default)))]
enum Enum<T> {
    Unit,
    #[educe(Default)]
    Struct {
        f1: T
    },
    Tuple(T),
}
# }
```

###### The `new` Associated Function

With the `#[educe(Default(new))]` attribute, your type will include an additional associated function called `new`. This function can be utilized to invoke the `default` method of the `Default` trait.

```rust
# #[cfg(feature = "Default")]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(Default(new))]
struct Struct {
    f1: u8
}
# }
```

#### Deref

Use `#[derive(Educe)]` and `#[educe(Deref)]` to implement the `Deref` trait for a struct or enum.

###### Basic Usage

You must designate a field as the default for obtaining an immutable reference unless the number of fields is exactly one.

```rust
# #[cfg(feature = "Deref")]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(Deref)]
struct Struct {
    f1: u8,
    #[educe(Deref)]
    f2: u8,
}

#[derive(Educe)]
#[educe(Deref)]
enum Enum {
    Struct {
        f1: u8
    },
    Struct2 {
        f1: u8,
        #[educe(Deref)]
        f2: u8,
    },
    Tuple(u8),
    Tuple2(
        u8,
        #[educe(Deref)]
        u8
    ),
}
# }
```

#### DerefMut

Use `#[derive(Educe)]` and `#[educe(DerefMut)]` to implement the `DerefMut` trait for a struct or enum.

###### Basic Usage

You must designate a field as the default for obtaining an mutable reference unless the number of fields is exactly one.

```rust
# #[cfg(all(feature = "Deref", feature = "DerefMut"))]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(Deref, DerefMut)]
struct Struct {
    f1: u8,
    #[educe(Deref, DerefMut)]
    f2: u8,
}

#[derive(Educe)]
#[educe(Deref, DerefMut)]
enum Enum {
    Struct {
        f1: u8
    },
    Struct2 {
        f1: u8,
        #[educe(Deref, DerefMut)]
        f2: u8,
    },
    Tuple(u8),
    Tuple2(
        #[educe(DerefMut)]
        u8,
        #[educe(Deref)]
        u8
    ),
}
# }
```

The mutable dereferencing fields do not need to be the same as the immutable dereferencing fields, but their types must be consistent.

#### Into

Use `#[derive(Educe)]` and `#[educe(Into(type))]` to make a struct or enum convertible into another type.

Educe generates an `impl From<YourType> for type`, which automatically provides the corresponding `Into` through the standard library's blanket implementation. Use the bare `into` flag — `#[educe(Into(type, into))]` — to generate a direct `impl Into<type>` instead.

###### The `into` Flag

A `From` impl also lets callers write `Target::from(value)`, whereas a direct `Into` impl only supports `value.into()`. Use the `into` flag when you deliberately want the conversion to be one-directional, exposed only as `value.into()`.

###### Basic Usage

You need to designate a field as the default for `Into<type>` conversion unless the number of fields is exactly one. If you don't, educe will automatically try to find a proper one.

Reference targets preserve explicit lifetimes, `mut`, and each reference layer, so `Into(&'a mut T)` and `Into(&'a &'b T)` keep those types in the generated interface.
An omitted lifetime on a target reference defaults to `'static`.
Automatic field matching ignores reference lifetimes, preserves reference depth, and permits an outer mutable reference to become a shared reference when Rust accepts the coercion.

Original generic declarations and `where` clauses keep `Self` tied to the source type in a generated `From` impl.
Custom `bound(...)` predicates are used verbatim: their `Self` means the target type in `From`, and the source type in a direct `Into` impl.
Custom method paths always use the source type for `Self`.

```rust
# #[cfg(feature = "Into")]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(Into(u8), Into(u16))]
struct Struct {
    f1: u8,
    f2: u16,
}

#[derive(Educe)]
#[educe(Into(u8))]
enum Enum {
    V1 {
        f1: u8,
        #[educe(Into(u8))]
        f2: u8,
    },
    V2 (
        u8
    ),
}
# }
```

###### Use Another Method to Perform Into Conversion

The `method` parameter can be utilized to replace the implementation of the `Into` trait for a field, eliminating the need to implement the `Into` trait for the type of that field.

```rust
# #[cfg(feature = "Into")]
# {
use educe::Educe;

fn into(v: u16) -> u8 {
    v as u8
}

#[derive(Educe)]
#[educe(Into(u8))]
enum Enum {
    V1 {
        #[educe(Into(u8, method(into)))]
        f1: u16,
    },
    V2 (
        u8
    ),
}
# }
```

###### Generic Parameters Bound to the `Into` Trait or Others

A generic type parameter is automatically bound to `Into<type>` only when it is itself the type of a field, because a nested parameter cannot meaningfully receive an `Into` bound.

```rust
# #[cfg(feature = "Into")]
# {
use educe::Educe;

#[derive(Educe)]
#[educe(Into(u8))]
enum Enum<T, K> {
    V1 {
        f1: K,
    },
    V2 (
        T
    ),
}
# }
```

Or you can set the where predicates by yourself.

```rust
# #[cfg(feature = "Into")]
# {
use educe::Educe;

fn into<T>(_v: T) -> u8 {
    0
}

#[derive(Educe)]
#[educe(Into(u8, bound(K: Into<u8>)))]
enum Enum<T, K> {
    V1 {
        f1: K,
    },
    V2 (
        #[educe(Into(u8, method(into)))]
        T
    ),
}
# }
```

*/

#![cfg_attr(docsrs, feature(doc_cfg))]

mod common;
#[allow(dead_code)]
mod panic;
mod supported_traits;
mod trait_handlers;

use std::collections::HashMap;

use proc_macro::TokenStream;
use supported_traits::Trait;
use syn::{
    DeriveInput, Meta, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};
#[cfg(any(
    feature = "Clone",
    feature = "Copy",
    feature = "Debug",
    feature = "Default",
    feature = "Deref",
    feature = "DerefMut",
    feature = "Eq",
    feature = "Hash",
    feature = "Ord",
    feature = "PartialEq",
    feature = "PartialOrd"
))]
use trait_handlers::TraitHandler;
use trait_handlers::TraitHandlerContext;
#[cfg(feature = "Into")]
use trait_handlers::TraitHandlerMultiple;

/// The entry point of the expansion: collects the traits requested by the `#[educe(...)]` attributes and dispatches each of them to its handler.
///
/// The handlers run in a fixed order (Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Deref, DerefMut, Into) so that a trait always runs after the traits it may inherit bounds from.
fn derive_input_handler(ast: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let mut token_stream = proc_macro2::TokenStream::new();
    let mut trait_meta_map: HashMap<Trait, Vec<Meta>> = HashMap::new();

    for attr in ast.attrs.iter() {
        let path = attr.path();

        if path.is_ident("educe") {
            if let Meta::List(list) = &attr.meta {
                let result =
                    list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

                for meta in result {
                    let path = meta.path();

                    let t = match Trait::from_path(path) {
                        Some(t) => t,
                        None => return Err(panic::unsupported_trait(meta.path())),
                    };

                    if let Some(v_meta) = trait_meta_map.get_mut(&t) {
                        // except for those traits containing generics types

                        #[cfg(feature = "Into")]
                        if t == Trait::Into {
                            v_meta.push(meta);

                            continue;
                        }

                        // avoid unused warnings
                        let _ = v_meta;

                        return Err(panic::reuse_a_trait(path.get_ident().unwrap()));
                    }

                    trait_meta_map.insert(t, vec![meta]);
                }
            } else {
                return Err(panic::educe_format_incorrect(path.get_ident().unwrap()));
            }
        }
    }

    let traits: Vec<Trait> = trait_meta_map.keys().copied().collect();

    let mut ctx = TraitHandlerContext::default();

    #[cfg(feature = "Debug")]
    {
        if let Some(meta) = trait_meta_map.get(&Trait::Debug) {
            trait_handlers::debug::DebugHandler::trait_meta_handler(
                &ast,
                &mut ctx,
                &mut token_stream,
                &traits,
                &meta[0],
            )?;
        }
    }

    #[cfg(feature = "Clone")]
    {
        if let Some(meta) = trait_meta_map.get(&Trait::Clone) {
            trait_handlers::clone::CloneHandler::trait_meta_handler(
                &ast,
                &mut ctx,
                &mut token_stream,
                &traits,
                &meta[0],
            )?;
        }
    }

    #[cfg(feature = "Copy")]
    {
        if let Some(meta) = trait_meta_map.get(&Trait::Copy) {
            trait_handlers::copy::CopyHandler::trait_meta_handler(
                &ast,
                &mut ctx,
                &mut token_stream,
                &traits,
                &meta[0],
            )?;
        }
    }

    #[cfg(feature = "PartialEq")]
    {
        if let Some(meta) = trait_meta_map.get(&Trait::PartialEq) {
            trait_handlers::partial_eq::PartialEqHandler::trait_meta_handler(
                &ast,
                &mut ctx,
                &mut token_stream,
                &traits,
                &meta[0],
            )?;
        }
    }

    #[cfg(feature = "Eq")]
    {
        if let Some(meta) = trait_meta_map.get(&Trait::Eq) {
            trait_handlers::eq::EqHandler::trait_meta_handler(
                &ast,
                &mut ctx,
                &mut token_stream,
                &traits,
                &meta[0],
            )?;
        }
    }

    #[cfg(feature = "PartialOrd")]
    {
        if let Some(meta) = trait_meta_map.get(&Trait::PartialOrd) {
            trait_handlers::partial_ord::PartialOrdHandler::trait_meta_handler(
                &ast,
                &mut ctx,
                &mut token_stream,
                &traits,
                &meta[0],
            )?;
        }
    }

    #[cfg(feature = "Ord")]
    {
        if let Some(meta) = trait_meta_map.get(&Trait::Ord) {
            trait_handlers::ord::OrdHandler::trait_meta_handler(
                &ast,
                &mut ctx,
                &mut token_stream,
                &traits,
                &meta[0],
            )?;
        }
    }

    #[cfg(feature = "Hash")]
    {
        if let Some(meta) = trait_meta_map.get(&Trait::Hash) {
            trait_handlers::hash::HashHandler::trait_meta_handler(
                &ast,
                &mut ctx,
                &mut token_stream,
                &traits,
                &meta[0],
            )?;
        }
    }

    #[cfg(feature = "Default")]
    {
        if let Some(meta) = trait_meta_map.get(&Trait::Default) {
            trait_handlers::default::DefaultHandler::trait_meta_handler(
                &ast,
                &mut ctx,
                &mut token_stream,
                &traits,
                &meta[0],
            )?;
        }
    }

    #[cfg(feature = "Deref")]
    {
        if let Some(meta) = trait_meta_map.get(&Trait::Deref) {
            trait_handlers::deref::DerefHandler::trait_meta_handler(
                &ast,
                &mut ctx,
                &mut token_stream,
                &traits,
                &meta[0],
            )?;
        }
    }

    #[cfg(feature = "DerefMut")]
    {
        if let Some(meta) = trait_meta_map.get(&Trait::DerefMut) {
            trait_handlers::deref_mut::DerefMutHandler::trait_meta_handler(
                &ast,
                &mut ctx,
                &mut token_stream,
                &traits,
                &meta[0],
            )?;
        }
    }

    #[cfg(feature = "Into")]
    {
        if let Some(meta) = trait_meta_map.get(&Trait::Into) {
            trait_handlers::into::IntoHandler::trait_meta_handler(
                &ast,
                &mut ctx,
                &mut token_stream,
                &traits,
                meta,
            )?;
        }
    }

    if trait_meta_map.contains_key(&Trait::_Nothing) {
        // avoid unused warnings
        let _ = &ast;
        let _ = &mut ctx;
        let _ = &mut token_stream;
        let _ = traits;
        unreachable!();
    }

    if token_stream.is_empty() {
        return Err(panic::derive_attribute_not_set_up_yet());
    }

    Ok(token_stream)
}

#[proc_macro_derive(Educe, attributes(educe))]
pub fn educe_derive(input: TokenStream) -> TokenStream {
    struct MyDeriveInput(proc_macro2::TokenStream);

    impl Parse for MyDeriveInput {
        #[inline]
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let token_stream = derive_input_handler(input.parse::<DeriveInput>()?)?;

            Ok(Self(token_stream))
        }
    }

    // Parse the token stream
    let derive_input = parse_macro_input!(input as MyDeriveInput);

    derive_input.0.into()
}
