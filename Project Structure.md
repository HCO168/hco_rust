# Project Structure

This document gives a quick map of the current project layout for `hco_rust`.

The crate is centered around four public top-level modules declared in [src/lib.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/lib.rs):

- `math`: math types, numeric helpers, and trait experiments.
- `datas`: data-structure and container-related utilities.
- `ntr_lang`: number-to-language conversion code.
- `tool`: debug-print helper macros.

## High-Level Tree

```text
hco_rust/
├── Cargo.toml
├── README.md
├── Project Structure.md
└── src/
    ├── lib.rs
    ├── tool.rs
    ├── math/
    │   ├── mod.rs
    │   ├── fraction.rs
    │   ├── digits.rs
    │   ├── num_theory.rs
    │   ├── vector.rs
    │   ├── matrix.rs
    │   ├── complex.rs
    │   └── traits/
    │       ├── mod.rs
    │       ├── additive.rs
    │       ├── multiplicative.rs
    │       ├── categories.rs
    │       ├── cmp.rs
    │       ├── constants.rs
    │       ├── sign.rs
    │       ├── nan.rs
    │       └── trait structure.md
    ├── datas/
    │   ├── mod.rs
    │   ├── array.rs
    │   ├── interval.rs
    │   ├── interval_set.rs
    │   ├── sort.rs
    │   ├── ordered.rs
    │   ├── ord_wrap.rs
    │   ├── map.rs
    │   ├── btree.rs
    │   ├── bound_protect.rs
    │   └── traits/
    │       ├── mod.rs
    │       ├── set.rs
    │       └── bitmap.rs
    └── ntr_lang/
        ├── mod.rs
        ├── lang.rs
        └── chinese.rs
```

## Top-Level Modules

### `src/lib.rs`

This is the crate entry point.

- Exposes `pub mod math;`
- Exposes `pub mod datas;`
- Exposes `pub mod ntr_lang;`
- Exposes `pub mod tool;`
- Gates nightly specialization support with `cfg_attr(feature = "specialization", feature(min_specialization))`

### `src/tool.rs`

Contains exported debug macros:

- `dprintln!`
- `dprint!`

These only print in debug builds.

## `math` Module

Declared in [src/math/mod.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/math/mod.rs).

Primary purpose: reusable math-oriented types and trait systems.

### Main files

- [src/math/fraction.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/math/fraction.rs)
  Generic `Fraction<T>` type with arithmetic trait implementations.
- [src/math/digits.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/math/digits.rs)
  Base-aware digit container plus conversion helpers such as `arabic_num_to_char`.
- [src/math/num_theory.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/math/num_theory.rs)
  GCD implementations and number-theory helpers.
- [src/math/vector.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/math/vector.rs)
  Fixed-size generic vector type.
- [src/math/matrix.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/math/matrix.rs)
  Experimental matrix-related code.
- [src/math/complex.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/math/complex.rs)
  Experimental complex-number-related code.

### `math::traits`

Declared in [src/math/traits/mod.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/math/traits/mod.rs).

This folder acts as the project's numeric trait toolbox.

- [src/math/traits/additive.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/math/traits/additive.rs)
  Additive identity and related additive helpers.
- [src/math/traits/multiplicative.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/math/traits/multiplicative.rs)
  Multiplicative identity and reciprocal-related traits.
- [src/math/traits/categories.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/math/traits/categories.rs)
  Numeric category markers such as integer-style traits.
- [src/math/traits/cmp.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/math/traits/cmp.rs)
  Comparison-related traits including non-comparable detection and min/max helpers.
- [src/math/traits/constants.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/math/traits/constants.rs)
  Numeric constant traits such as finite bounds and infinities.
- [src/math/traits/sign.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/math/traits/sign.rs)
  Sign-related enums and traits.
- [src/math/traits/nan.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/math/traits/nan.rs)
  NaN marker and inspection traits.
- [src/math/traits/trait structure.md](/Users/wenhanhu/Documents/GitHub/hco_rust/src/math/traits/trait%20structure.md)
  Internal design notes for the trait system.

## `datas` Module

Declared in [src/datas/mod.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/datas/mod.rs).

Primary purpose: data-structure experiments and collection helpers.

One notable detail: `datas/mod.rs` currently both declares modules and re-exports many items with `pub use ...::*;`, so this namespace is flatter than `math`.

### Main files

- [src/datas/array.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/datas/array.rs)
  Array wrapper utilities.
- [src/datas/interval.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/datas/interval.rs)
  Interval primitives including `Interval<T>`, `IntervalFlag`, and `ClosedRange<T>`.
- [src/datas/interval_set.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/datas/interval_set.rs)
  Set-like interval storage built on ordered collections.
- [src/datas/sort.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/datas/sort.rs)
  Sorting helpers such as merge sort and insertion sort.
- [src/datas/ordered.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/datas/ordered.rs)
  Traits for ordered set/map access patterns.
- [src/datas/ord_wrap.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/datas/ord_wrap.rs)
  Ordering wrapper utilities.
- [src/datas/map.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/datas/map.rs)
  Map-related traits and implementations.
- [src/datas/btree.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/datas/btree.rs)
  B-tree-related structures.
- [src/datas/bound_protect.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/datas/bound_protect.rs)
  Safe indexing strategy abstractions such as `ProtectStrategy` and `BoundProtect`.

### `datas::traits`

Declared in [src/datas/traits/mod.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/datas/traits/mod.rs).

- [src/datas/traits/set.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/datas/traits/set.rs)
  Basic set-like traits: `Contains`, `Insert`, and `Remove`.
- [src/datas/traits/bitmap.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/datas/traits/bitmap.rs)
  Bitmap-related trait experiments.

## `ntr_lang` Module

Declared in [src/ntr_lang/mod.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/ntr_lang/mod.rs).

Primary purpose: converting numbers into language-specific text output.

This module currently re-exports its submodules and their public items, so consumers can use either:

- `lib::ntr_lang::lang::NumToLangParser`
- `lib::ntr_lang::NumToLangParser`

### Files

- [src/ntr_lang/lang.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/ntr_lang/lang.rs)
  Core traits and error types for number-to-language conversion.
- [src/ntr_lang/chinese.rs](/Users/wenhanhu/Documents/GitHub/hco_rust/src/ntr_lang/chinese.rs)
  Chinese number rendering logic, including configurable parser behavior and tests.

## Export Pattern Notes

The project currently uses a mixed export style:

- `lib.rs` exports top-level modules only.
- `math` mostly stays hierarchical by module path.
- `datas` re-exports many internal items at the module root.
- `ntr_lang` re-exports public items from its submodules.

This works, but it means API shape is not fully uniform across the crate yet.

## Feature Flags

Defined in [Cargo.toml](/Users/wenhanhu/Documents/GitHub/hco_rust/Cargo.toml):

- `default = []`
- `specialization = []`

Current intent:

- Stable Rust should work by default.
- The `specialization` feature is reserved for nightly-only specialized behavior.

## Maintenance Notes

- `src/math/traits/trait structure.md` is a design note, not a Rust module.
- Some modules are clearly more mature than others; the project mixes reusable code with experiments.
- `datas/mod.rs` currently flattens part of the namespace via re-exports, which may affect future API cleanup.
- The published crate name is currently `lib`, even though the repository is `hco_rust`.

## Suggested Future Cleanup

- Unify the export strategy across `math`, `datas`, and `ntr_lang`.
- Decide which modules are stable public API and which are still experimental.
- Consider renaming the published crate from `lib` to a less generic public name in a future breaking release.
