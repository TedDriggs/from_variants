#![crate_type = "proc-macro"]

#[macro_use]
extern crate error_chain;

extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

mod errors;
mod bindings;
mod from_impl;
mod parser;
mod state;

mod prelude {
    pub use bindings::Bindings;
    pub use errors::{Error, ErrorKind, Result, ResultExt};
}

use proc_macro::TokenStream;
use prelude::*;

#[doc(hidden)]
#[proc_macro_derive(FromVariants, attributes(from_variants))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_derive_input(&input.to_string()).expect("Couldn't parse item");
    let result = build_converters(ast).unwrap().to_string();
    
    result.parse().expect(&format!("Couldn't parse `{}` to tokens", result))
}

fn build_converters(ast: syn::DeriveInput) -> Result<quote::Tokens> {
    let target_ident = ast.ident;
    let context = parser::Context::new(target_ident)
        .parse_attributes(ast.attrs)?
        .parse_body(ast.body)?
        .finish();
    let bodies = context.as_impls();
    Ok(quote!(#(#bodies)*))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
