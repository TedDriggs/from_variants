//! Rust macro crate to automatically generate conversions from variant types into the target enum.
//!
//! This crate requires Rust 1.15 or above to compile on stable.
//!
//! # Examples
//!
//! ```rust
//! #[macro_use]
//! extern crate from_variants;
//!
//! #[derive(Debug, Clone, PartialEq, Eq, FromVariants)]
//! pub enum Lorem {
//!     Str(String),
//!     Num(u16),
//! }
//!
//! fn main() {
//!     assert_eq!(Lorem::Num(10), Lorem::from(10));
//! }
//! ```
//!
//! You can skip variants to avoid type collisions:
//!
//! ```rust
//! #[macro_use]
//! extern crate from_variants;
//!
//! #[derive(Debug, Clone, PartialEq, Eq, FromVariants)]
//! pub enum Ipsum {
//!     Hello(String),
//!
//!     #[from_variants(skip)]
//!     Goodbye(String),
//! }
//!
//! fn main() {
//!     assert_eq!(Ipsum::Hello("John".to_string()), Ipsum::from("John".to_string()));
//! }
//! ```
//!
//! # Features
//! * **Variant opt-out**: To skip a variant, add `#[from_variants(skip)]` to that variant.
//! * **Conversion opt-in**: Use `#[from_variants(into)]` on an enum or variant to generate conversions
//!   that will automatically convert - for example, accepting a `&str` for a `String` variant.
//!   This must be used sparingly to avoid generating conflicting impls.
extern crate core;

#[allow(unused_imports)]
#[macro_use]
extern crate from_variants_impl;
pub use from_variants_impl::*;

#[doc(hidden)]
pub mod export {
    pub use ::core::convert::From;
    pub use ::core::convert::Into;
}