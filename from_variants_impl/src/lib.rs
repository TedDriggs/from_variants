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
mod from_impl;
mod parser;
mod state;
mod util;

mod prelude {
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
