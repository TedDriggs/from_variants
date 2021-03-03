use from_variants::FromVariants;

#[derive(Debug, PartialEq, Eq, FromVariants)]
pub enum Demo<T> {
    Hello,
    #[from_variants(skip)]
    World(T),
    Lorem(String),
    Dolor(u32),
    #[from_variants(skip)]
    NoGood {
        has: bool,
        fields: bool,
    },
}

#[test]
fn from_string() {
    assert_eq!(
        Demo::<()>::from("Hello".to_string()),
        Demo::Lorem("Hello".to_string())
    );
}
