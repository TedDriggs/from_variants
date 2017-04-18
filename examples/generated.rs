#[macro_use]
extern crate from_variants;

#[derive(Debug, Clone, FromVariants)]
#[from_variants(no_std)]
pub enum Lorem {
    Str(String),
    Num(u16),
}

fn main() {
    println!("{:?}", Lorem::from(10));
}