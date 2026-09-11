# Changelog: 0.7.6 to 0.8.0

This document compares `v0.7.6` (`8b73bc2`) with `0.8.0` and focuses on changes that affect crate users.
Several correctness fixes change generated bounds, conversion signatures, or method resolution, so they are released as `0.8.0`.

## Breaking and Upgrade Notes

- Automatic `Eq` now requires every ordinary struct or enum field to implement `Eq`, including concrete fields such as `f64` and fields that use const generic parameters.
  A type that previously compiled with an ordinary non-`Eq` field can now fail to compile.
  If an external, manually written `PartialEq` implementation provides an equivalence relation without requiring every field to implement `Eq`, use explicit `Eq(bound(false))`, `Eq(bound(*))`, or custom predicates.
  The author remains responsible for reflexive, symmetric, and transitive equality.
- When Educe derives `PartialEq` and `Eq` together, fields marked `PartialEq(ignore)` or `PartialEq(method = ...)` are excluded from automatic `Eq` bounds and field checks.
  Add explicit bounds if another custom method relied on an automatically added `T: Eq`.
  Educe cannot inspect an external `PartialEq` implementation, so its fields are checked as ordinary fields.
- `Into` targets now preserve explicit reference lifetimes, mutability, and reference depth.
  For example, `Into(&'a mut T)` keeps `&'a mut T` instead of becoming `&'static T`.
  Adjust target declarations or caller types if they relied on the old normalization.
  Omitted lifetimes on target references still default to `'static`.
- `Self` in custom `Into` method paths now refers to the source type, including its generic arguments, even when the generated implementation is `From` for the target type.
  Use an explicit target type path if the method belongs to the target.
- `Self` in the source type's original generic bounds and `where` clause stays tied to the source when those bounds are moved into `From`.
  Custom `bound(...)` predicates remain verbatim: their `Self` refers to the target in `From` and to the source in a direct `Into` implementation.
  Put intended target constraints in an explicit `bound(...)`.
- Automatic bounds for `HashMap` and `HashSet` with an explicit hasher now use the complete container type.
  This preserves the container's actual requirements, such as a default hasher for `Default`, without requiring the hasher to implement `Debug` for formatting.
  Add explicit predicates if custom methods relied on former per-parameter bounds such as `T: Clone`.
- Fields that use const generic parameters now participate in automatic bound inference.
  This can make a generated implementation conditional where it was previously unconditional.
- When `Copy` and `Clone` are derived together and a field uses a const generic parameter, cloning now follows the field methods instead of the bitwise-copy shortcut.
  Field `clone` and `clone_from` side effects can become observable; the performance difference has not been measured.
- Custom method paths now preserve qualified types and avoid capture by generated local variables.
  Code that depended on a dropped qualifier or accidental name capture can resolve to a different method or fail to compile.
  Write the intended function as an explicit path.
- A `#[repr(packed)]` or `#[repr(packed(N))]` type now has each field it reads copied into a temporary before being borrowed, so every field the generated code reads must implement `Copy`.
  Every packing level is treated the same way, matching the built-in derives, because a field's alignment cannot be worked out from the type syntax.
  A packed type with a non-`Copy` field whose alignment happens to fit the packing, such as `#[repr(packed(8))] struct S(Vec<u8>)`, compiled before and no longer does; the built-in derives reject it as well.

Implementation references: [`EqHandler`](src/trait_handlers/eq/mod.rs), [`IntoStructHandler`](src/trait_handlers/into/into_struct.rs), [`IntoEnumHandler`](src/trait_handlers/into/into_enum.rs), [`BoundExceptions` and `type_uses_generic_params`](src/common/type.rs), [`CloneStructHandler`](src/trait_handlers/clone/clone_struct.rs), and [`CloneEnumHandler`](src/trait_handlers/clone/clone_enum.rs).

## Custom Methods and Generated Code

- Custom methods accept complete expression paths, including `<Type as Trait>::method::<T>`, while preserving generic arguments and source spans.
  The existing `method = path`, `method(path)`, `method = "path"`, and `method("path")` forms remain supported.
- Custom `Debug` methods now use the final implementation bounds and keep `Self` in the source type's scope.
  The `Debug` and `Clone` method markers also preserve source `Self` in field types, bounds, and qualified method paths.
- Generated parameters, local variables, and enum pattern bindings use macro hygiene to avoid shadowing custom methods named `f`, `builder`, `arg`, `source`, `other`, `state`, or `value`.
- Generated `Hash` methods avoid generic parameter names already used by the input type, including collisions with type or const parameters named `H`, `_H`, or `__H`.
- Helper items for `Debug`, `Clone`, and automatic `Eq` inherit the input type's `allow`, `warn`, and `deny` attributes.
  An `expect` is copied as `allow` to avoid introducing an extra lint expectation.
  Method markers remain visible to dead-code analysis, so custom methods used only by a derive are still counted as used.

Implementation references: [`meta_2_path`](src/common/path.rs), [`ReplaceSelf` and `unused_ident`](src/common/generics.rs), [`quote_mixed`](src/common/mod.rs), [`debug::common`](src/trait_handlers/debug/common.rs), and [`generated_lint_attributes`](src/common/attributes.rs).

## Trait Fixes

- Ordinary `Debug` fields support dynamically sized types such as `T: ?Sized + Debug`, including named, tuple, and map-style struct output.
- `Into` field matching ignores reference lifetimes while preserving mutability and reference depth.
  It also permits an outer mutable reference to become a shared reference when Rust accepts the coercion.
- `Ord` and `PartialOrd` leave enum discriminant expressions to the compiler instead of parsing only integer literals.
  This supports constant expressions, `Self` references, large unsigned values such as `u128::MAX`, and combined representation attributes such as `repr(C, align(8))`.
- Automatic `Eq` checks complete field types under the final implementation bounds without adding runtime calls or concrete field predicates to the public `where` clause.
  Explicit bound modes and unions keep their existing behavior.
- `Debug`, `Clone`, `PartialEq`, `PartialOrd`, `Ord`, and `Hash` support `#[repr(packed)]` and `#[repr(packed(N))]` structs, which previously failed to compile with `error[E0793]: reference to field of packed struct is unaligned`.
  Fields are copied before being read and the automatic bound adds the `Copy` predicates they need, so a packed type whose fields need no predicate at all, such as `#[repr(packed)] struct S<T>(i32, PhantomData<T>)`, gets an implementation without any bound.
  `Clone::clone_from` assigns the cloned value back instead of cloning in place, because a packed field cannot be borrowed mutably either.
  `Deref` and `DerefMut` are unchanged: they return a reference to a field, so they still require the target field to fit the packing.

Implementation references: [`DebugStructHandler`](src/trait_handlers/debug/debug_struct.rs), [`field_matches_target`](src/trait_handlers/into/common.rs), [`DiscriminantType::from_ast`](src/common/tools/discriminant_type.rs), [`EqHandler`](src/trait_handlers/eq/mod.rs), and [`is_packed` and `borrow_field`](src/common/attributes.rs).

## Documentation and Tests

- Clarified the existing safety contract for union `Debug`, `PartialEq`, and `Hash`: every byte read must be initialized and readable, including padding and bytes outside the active field, and storage must not change during a call.
  The contract must hold after construction, writes, moves, and copies.
  This documents existing undefined-behavior risks; the byte-reading algorithms are unchanged.
- Corrected the `Default` documentation to describe default expressions and the selected enum variant or union field.
- Migration guidance for reference targets, `Self`, automatic bounds, and custom equality lives in this changelog; the README and the crate documentation describe only the current behavior.
- Restored the missing test attributes on the enum `Into` method tests and made the generic union test require `Eq`.
- Added regression coverage for qualified method paths, name clashes, reference conversions, generic bounds, dynamically sized fields, discriminants, lint propagation, dead-code analysis, and packed types.
  Two compile-fail doctests verify that automatic `Eq` rejects ordinary non-`Eq` fields in structs and enums.

References: [`README.md`](README.md), [crate documentation](src/lib.rs), [`into_enum::method_1` and `method_2`](tests/into_enum.rs), [`eq_union::bound`](tests/eq_union.rs), and [regression tests](tests).

## Package Compatibility

- The minimum supported Rust version remains Rust 1.89, and the crate continues to use the Rust 2024 edition.
- The default trait features and the optional `full` feature are unchanged.
- Dependency version requirements are unchanged; `syn` now enables its `visit` and `visit-mut` features.

Reference: [`Cargo.toml`](Cargo.toml).
