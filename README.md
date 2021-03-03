[![Build Status](https://github.com/TedDriggs/from_variants/workflows/CI/badge.svg)](https://github.com/TedDriggs/from_variants/actions)
[![Latest Version](https://img.shields.io/crates/v/from_variants.svg)](https://crates.io/crates/from_variants)

# Newtype Variant Conversions

Rust macro crate to automatically generate conversions from variant types into the target enum.

This crate requires Rust 1.45 or above to compile on stable.

## Examples

```rust
use from_variants::FromVariants;

#[derive(Debug, Clone, PartialEq, Eq, FromVariants)]
pub enum Lorem {
    Str(String),
    Num(u16),
}

fn main() {
    assert_eq!(Lorem::Num(10), Lorem::from(10));
}
```

You can skip variants to avoid type collisions:

```rust
use from_variants::FromVariants;

#[derive(Debug, Clone, PartialEq, Eq, FromVariants)]
pub enum Ipsum {
    Hello(String),

    #[from_variants(skip)]
    Goodbye(String),
}

fn main() {
    assert_eq!(Ipsum::Hello("John".to_string()), Ipsum::from("John".to_string()));
}
```

## Features

- **Variant opt-out**: To skip a variant, add `#[from_variants(skip)]` to that variant.
- **Conversion opt-in**: Use `#[from_variants(into)]` on an enum or variant to generate conversions
  that will automatically convert - for example, accepting a `&str` for a `String` variant.
  This must be used sparingly to avoid generating conflicting impls.
- **no_std support**: Generated conversions do not depend on the standard library.
