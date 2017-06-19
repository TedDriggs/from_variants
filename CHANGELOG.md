# Change Log

## 0.2.3

### Improvements
- errors will now be returned all at once, rather than stopping assessment on first error.

## 0.2.2

### Improvements
- attempting to derive `FromVariants` on a struct now causes a compilation error instead of failing silently.

## 0.2.1 - 2017-05-17

### Internal Changes
- switched to [`darling`](https://crates.io/crates/darling) for attribute parsing.

## 0.2.0 - 2017-04-26

### Breaking Changes
- removed `#[from_variants(no_std)]`; generated code will now always refer to `core`. Projects using `std` will not see any adverse effects.