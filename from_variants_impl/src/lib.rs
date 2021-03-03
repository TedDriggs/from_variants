mod from_impl;
mod parser;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

type Result<T> = std::result::Result<T, darling::Error>;

#[doc(hidden)]
#[allow(missing_docs)]
#[proc_macro_derive(FromVariants, attributes(from_variants))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Couldn't parse item");
    build_converters(ast).unwrap()
}

fn build_converters(ast: syn::DeriveInput) -> Result<TokenStream> {
    let context = parser::Container::from_derive_input(&ast).map_err(|e| e.flatten())?;
    let bodies = context.as_impls();
    Ok(quote!(#(#bodies)*).into())
}
