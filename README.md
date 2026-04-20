# hco_rust

`hco_rust` is a small Rust utility library focused on math helpers, container/data-structure experiments, and simple number-to-language conversion tools.

The crate is still early-stage and intended primarily for study and exploration rather than production use. APIs may change between releases.

## Highlights

- Math utilities such as fractions, digit representations, number-theory helpers, vectors, and numeric traits.
- Data-structure utilities such as interval sets, sorting helpers, ordered-search traits, and bounds-protection helpers.
- Natural-language number conversion experiments, currently including Chinese number rendering.
- Stable by default: nightly-only specialization behavior is behind an explicit `specialization` feature.

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
lib = "0.0.2"
```

If you want the specialization-based implementations, enable the feature explicitly:

```toml
[dependencies]
lib = { version = "0.0.2", features = ["specialization"] }
```

The `specialization` feature requires a nightly compiler.

## Module Overview

The public top-level modules are:

- `lib::math`: numeric data types, algorithms, and math-related traits.
- `lib::datas`: container and interval-related data structures and helpers.
- `lib::ntr_lang`: number-to-language conversion traits and implementations.
- `lib::tool`: lightweight debug-print macros.

### `math`

Notable submodules:

- `fraction`: generic `Fraction<T>`.
- `digits`: a base-aware `Digits` representation.
- `num_theory`: GCD implementations and related helpers.
- `traits`: numeric traits such as sign, additive/multiplicative identities, constants, and comparison helpers.
- `vector`, `matrix`, `complex`: experimental math structures.

### `datas`

Notable submodules:

- `interval` and `interval_set`: interval types and set-like interval storage.
- `sort`: merge sort and insertion sort helpers.
- `ordered`: traits for ordered collections.
- `map`, `btree`, `bound_protect`, `array`, `ord_wrap`: additional experimental utilities.
- `traits`: collection traits such as `Contains`, `Insert`, and `Remove`.

### `ntr_lang`

Notable submodules:

- `lang`: core traits such as `NumToLangParser` and `LanguageParser`.
- `chinese`: Chinese number conversion support.

## Examples

### Digits

```rust
use lib::math::digits::{Digits, arabic_num_to_char};

let digits = Digits::from_u64(114514, 10);
assert_eq!(digits.to_string(arabic_num_to_char).unwrap(), "114514");
```

### Fractions

```rust
use lib::math::fraction::Fraction;

let a = Fraction::new(1_i64, 2);
let b = Fraction::new(1_i64, 3);
let c = a + b;

assert_eq!(c.to_f64(), 5.0 / 6.0);
```

### Chinese Number Rendering

```rust
use lib::math::digits::Digits;
use lib::ntr_lang::lang::NumToLangParser;
use lib::ntr_lang::chinese::NumberToChineseParser;

let parser = NumberToChineseParser::default();
let text = parser.number_to_text(Digits::from_u64(10001, 10)).unwrap();

assert_eq!(text, "一万零一");
```

## Features

### `specialization`

Enables nightly-only specialization-based implementations for part of the comparison trait system.

- Default: disabled
- Compiler requirement when enabled: nightly Rust
- Intended use: experimentation with specialized trait behavior

Without this feature, the crate is intended to compile on stable Rust.

## Project Notes

- This library currently favors clarity and experimentation over aggressive optimization.
- Some modules are more mature than others; behavior and naming may still evolve.
- The crate name published on crates.io is currently `lib`, so imports use paths such as `lib::math::digits::Digits`.

## Development

Useful local commands:

```bash
cargo check
cargo test
cargo publish --dry-run --allow-dirty
```

## License

Licensed under the Apache License, Version 2.0. See [`LICENSE`](LICENSE).
