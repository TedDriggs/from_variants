# Change Log

## 0.6.0 (March 3, 2021)

- Add `#![no_std]` to `from_variants` so the crate works on `no_std` targets.

## 0.5.0 (July 17, 2020)

- Change minimum Rust version from 1.15 to 1.45
- Update Rust to `edition = "2018"`.
- Update dependencies on `syn`, `quote`, `darling` to their latest versions.
- Drop `error-chain` dependency.

## 0.4.0 (May 14, 2018)

- Update dependencies on `syn`, `quote`, `darling`, and `error-chain` to their latest versions.

## 0.3.0 (April 5, 2018)

- Update dependencies on `syn`, `quote`, and `darling` to their latest versions.

## 0.2.4 (January 26, 2018)

- Update dependencies on `syn`, `quote`, and `darling` to their latest versions.

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
