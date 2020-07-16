mod errors;
mod from_impl;
mod parser;

mod prelude {
    pub use crate::errors::{Error, ErrorKind, Result, ResultExt};
}

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::*; // TODO: upgrate quote version because it requires all macros in scope
use prelude::*;

#[doc(hidden)]
#[allow(missing_docs)]
#[proc_macro_derive(FromVariants, attributes(from_variants))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Couldn't parse item");
    let result = build_converters(ast).unwrap().to_string();

    result.parse().expect(&format!("Couldn't parse `{}` to tokens", result))
}

fn build_converters(ast: syn::DeriveInput) -> Result<quote::Tokens> {
    let context = parser::Container::from_derive_input(&ast).map_err(|e| e.flatten())?;
    let bodies = context.as_impls();
    Ok(quote!(#(#bodies)*))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
