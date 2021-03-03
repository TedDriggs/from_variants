//! Example
#![warn(missing_docs)]

#[macro_use]
extern crate from_variants;

/// A sample struct.
#[derive(Debug, Clone, FromVariants)]
pub enum Lorem {
    /// Hello world
    #[from_variants(skip)]
    Str(String),

    /// Hello world
    Num(u16),
}

fn main() {
    println!("{:?}", Lorem::from(10));
}
