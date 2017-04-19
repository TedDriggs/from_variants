use syn;

/// The bindings mode to use in generated code. Defaults to using
/// the standard library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Bindings {
    /// Link to standard library.
    Std,
    
    /// Link to `core`.
    NoStd,
}

impl Bindings {
    /// Gets the path for the `From` trait.
    pub fn from_trait(&self) -> syn::Path {
        syn::parse_path(match *self {
            Bindings::Std => "::std::convert::From",
            Bindings::NoStd => "::core::convert::From",
        }).expect("Static `From` trait paths should be well-formed")
    }
}

impl Default for Bindings {
    fn default() -> Self {
        Bindings::Std
    }
}

#[cfg(test)]
mod tests {
    use syn;
    use super::Bindings;
    
    #[test]
    fn default() {
        assert_eq!(syn::parse_path("::std::convert::From").unwrap(), Bindings::default().from_trait());
    }
    
    #[test]
    fn std() {
        assert_eq!(syn::parse_path("::std::convert::From").unwrap(), Bindings::Std.from_trait());
    }
    
    #[test]
    fn no_std() {
        assert_eq!(syn::parse_path("::core::convert::From").unwrap(), Bindings::NoStd.from_trait());
    }
}