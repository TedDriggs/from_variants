mod from_impl;
mod parser;

use darling::{FromDeriveInput, Result};
use proc_macro::TokenStream;
use quote::quote;

#[doc(hidden)]
#[allow(missing_docs)]
#[proc_macro_derive(FromVariants, attributes(from_variants))]
pub fn derive(input: TokenStream) -> TokenStream {
    match build_converters(syn::parse_macro_input!(input)).map_err(|e| e.write_errors()) {
        Ok(ts) => ts.into(),
        Err(e) => e.into(),
    }
}

fn build_converters(ast: syn::DeriveInput) -> Result<TokenStream> {
    let context =
        parser::Container::from_derive_input(&ast).and_then(parser::Container::validate)?;
    let bodies = context.as_impls();
    Ok(quote!(#(#bodies)*).into())
}
