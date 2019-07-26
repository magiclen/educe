Educe
====================

[![Build Status](https://travis-ci.org/magiclen/educe.svg?branch=master)](https://travis-ci.org/magiclen/educe)
[![Build status](https://ci.appveyor.com/api/projects/status/nkxmmk08xegs28pr/branch/master?svg=true)](https://ci.appveyor.com/project/magiclen/educe/branch/master)

This crate provides procedural macros to help you implement Rust-build-in traits quickly.

## Debug

Use `#[derive(Educe)]` and `#[educe(Debug)]` to implement the `Debug` trait for a struct, an enum, or a union. It supports to change the name of your types, variants and fields. You can also ignore some fields, or set a trait and/or a method to replace the `Debug` trait used as default. Also, you can even format a struct to a tuple, and vice versa.

#### Basic Usage

```rust
#[macro_use] extern crate educe;

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
```

#### Union

A union will be formatted to a `u8` slice, because we don't know it's field at runtime.

```rust
#[macro_use] extern crate educe;

#[derive(Educe)]
#[educe(Debug)]
struct Union {
    f1: u8,
    f2: i32,
}
```

#### Change the Name of a Type, a Variant or a Field

The `name` attribute can help you rename a type, a variant or a field.

```rust
#[macro_use] extern crate educe;

#[derive(Educe)]
#[educe(Debug(name = "Struct2"))]
struct Struct {
    #[educe(Debug(name = "f"))]
    f1: u8
}

#[derive(Educe)]
#[educe(Debug(name = true))]
enum Enum {
    #[educe(Debug(name = false))]
    V1,
    #[educe(Debug(name = "V"))]
    V2 {
        #[educe(Debug(name = "f"))]
        f1: u8,
    },
    #[educe(Debug(name = false))]
    V3(u8),
}
```

#### Ignore Fields

The `ignore` attribute can ignore specific fields.

```rust
#[macro_use] extern crate educe;

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
```

#### Fake Structs and Tuples

With the `named_field` attribute, structs can be formatted as tuples and tuples can be formatted as structs.

```rust
#[macro_use] extern crate educe;

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
        #[educe(Debug(name = "value"))]
        i32
    ),
}
```

#### Use Another Method or Trait to Do the Format Thing

The `format` attribute has two parameters: `trait` and `method`. They can be used to replace the `Debug` trait on fields. If you only set the `trait` parameter, the `method` will be set to `fmt` automatically as default.

```rust
#[macro_use] extern crate educe;

use std::fmt::{self, Formatter};

fn fmt(_s: &u8, f: &mut Formatter) -> fmt::Result {
    f.write_str("Hi")
}

trait A {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("Hi")
    }
}

impl A for i32 {};
impl A for u64 {};

#[derive(Educe)]
#[educe(Debug)]
enum Enum<T: A> {
    V1,
    V2 {
        #[educe(Debug(format(method = "fmt")))]
        f1: u8,
    },
    V3(
        u8,
        #[educe(Debug(format(trait = "A")))]
        T
    ),
}
```

## TODO

There is a lot of work to be done. Unimplemented traits are listed below:

1. `Default`
1. `Clone`
1. `Copy`
1. `Hash`
1. `PartialEq`
1. `Eq`
1. `PartialOrd`
1. `Ord`
1. `From`
1. `Into`
1. `FromStr`
1. `TryFrom`
1. `Deref`
1. `DerefMut`

## Crates.io

https://crates.io/crates/educe

## Documentation

https://docs.rs/educe

## License

[MIT](LICENSE)
