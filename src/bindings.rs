use syn;

/// The bindings mode to use in generated code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Bindings {
    Std,
    NoStd,
}

impl Bindings {
    /// Gets the fully-qualified identifier for the `From` trait based on
    /// whether or not the std library is in use.
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