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
//! * **no_std support**: To generate conversions using `core::convert::From`, add `#[from_variants(no_std)]` at the struct level.
#![crate_type = "proc-macro"]

#[macro_use]
extern crate error_chain;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

mod errors;
mod bindings;
mod from_impl;
mod parser;
mod state;
mod util;

mod prelude {
    pub use bindings::Bindings;
    pub use errors::{Error, ErrorKind, Result, ResultExt};
}

use proc_macro::TokenStream;
use prelude::*;

#[doc(hidden)]
#[allow(missing_docs)]
#[proc_macro_derive(FromVariants, attributes(from_variants))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_derive_input(&input.to_string()).expect("Couldn't parse item");
    let result = build_converters(ast).unwrap().to_string();
    
    result.parse().expect(&format!("Couldn't parse `{}` to tokens", result))
}

fn build_converters(ast: syn::DeriveInput) -> Result<quote::Tokens> {
    let context = parser::Context::parse(ast)?;
    let bodies = context.as_impls();
    Ok(quote!(#(#bodies)*))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
