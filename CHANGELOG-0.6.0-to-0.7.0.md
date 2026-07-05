# Changelog: 0.6.0 to 0.7.0

This document compares `v0.6.0` with the current `0.7.0` worktree. It focuses on changes that can affect users of the crate. Internal refactoring details are not listed.

## Breaking and Upgrade Notes

- The minimum supported Rust version was raised from Rust 1.60 to Rust 1.89.
- The crate now uses the Rust 2024 edition.
- Automatic trait bounds were redesigned again. The generated `where` clauses may differ from 0.6.0, especially for wrapper types, recursive types, and traits with prerequisite traits.
- Precise generated bounds can show private field types in docs, compiler errors, or public `where` clauses. Check public types whose generated impl bounds are part of your API.
- `Eq` is now treated as a marker trait. Field-level equality settings such as `ignore` and `method` should be written on `PartialEq`, not on `Eq`.
- `Into` derives now generate `impl From<YourType> for Target` by default. Calling `value.into()` still works through Rust's blanket impl, and `Target::from(value)` also works. Use `#[educe(Into(Target, into))]` if you need a direct one-way `Into` impl instead.
- If you already wrote your own `From<YourType> for Target` impl, check for conflicts with the new default `Into` behavior.

## Trait Bounds

- Automatic bounds are now documented with one shared rule set.
- Ignored fields and fields handled by a custom `method` do not add automatic bounds for that trait.
- Field types without generic parameters do not add constant bounds.
- Common standard library wrapper types are handled more carefully. For example, `Option<T>` can add `T: Trait`, and `Vec<Box<T>>` can add only the needed inner bound.
- Types that always implement a trait, such as `PhantomData`, raw pointers, function pointers, and selected standard library types, can avoid unnecessary bounds.
- Other field types use precise bounds such as `Wrapper<T>: Clone` instead of always falling back to `T: Clone`.
- Self-recursive generic types are handled better. For example, a type like `List<T>` containing `Box<List<T>>` can avoid trait solver overflow by adding bounds on the type parameters.
- Mutually recursive generic types are still a known limitation. Use `bound(*)` or a custom `bound(...)` for those cases.
- Related traits can inherit generated predicates when they are derived together by Educe: `Copy` from `Clone`, `Eq` and `PartialOrd` from `PartialEq`, and `Ord` from `Eq` and `PartialOrd`.
- Explicit `bound(...)` values are used as written. If an explicit bound does not satisfy a required supertrait, the compiler will ask for the missing bound.

## Trait Behavior

- `Debug` output handles field names as strings. Tuple field names and custom names are printed more correctly, including tuple indexes such as `0`.
- `Copy`, `Clone`, `Eq`, `PartialOrd`, and `Ord` derive behavior is more consistent with their real Rust trait relationships.
- `Copy` can work with an Educe-generated or built-in `Clone` impl, while still respecting the bounds that `Copy` really needs.
- `PartialOrd` and `Ord` work together more consistently. When both are derived, `PartialOrd` can follow `Ord` field settings such as `ignore`, `rank`, and `method` so that `partial_cmp` and `cmp` stay consistent.
- When both comparison traits are derived, `Ord` can follow `PartialOrd` field settings for `ignore` and `rank`. A `PartialOrd` method cannot be reused for `Ord` because it returns `Option<Ordering>`.
- `Into` supports the new default `From` generation for concrete target types and for generic target types whose type parameters are covered by the target type.

## Generated Code and Lints

- Generated impls are marked with `#[automatically_derived]`.
- Lint-level attributes from the input type, such as `allow`, `expect`, `warn`, and `deny`, are copied to generated impls. This makes generated code behave more like built-in derives under strict lint settings.
- Diagnostics for invalid attributes and helper parsing were improved.

## Documentation and Safety Notes

- The README now has a dedicated Trait Bounds section that explains the automatic bound rules, bound inheritance, `bound(*)`, and known limitations.
- Documentation for unsafe union derives is clearer. It now states that `Debug`, `PartialEq`, and `Hash` union impls may read, compare, or hash the whole union memory, including padding bytes.
- The trait documentation was updated so each trait points to the shared bound rules instead of repeating older simplified rules.

## Dependency and Package Changes

- `quote` now has a minimum version of `1.0.44`.
- `proc-macro2` now has a minimum version of `1.0.80`.
- `enum-ordinalize` was updated from `4.2` to `4.4`.
- The `rustversion` dev-dependency was removed.