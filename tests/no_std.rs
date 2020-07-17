//! Compile and correctness test for using `core` instead of `std`.
#![no_std]

use from_variants::FromVariants;

#[derive(Debug, PartialEq, Eq, FromVariants)]
pub enum Lorem {
    Ipsum(u8),
    Dolor(i32),
}

#[test]
fn from_u8() {
    assert_eq!(Lorem::Ipsum(1), Lorem::from(1u8));
}

#[test]
fn from_i32() {
    assert_eq!(Lorem::Dolor(-1), Lorem::from(-1));
}
