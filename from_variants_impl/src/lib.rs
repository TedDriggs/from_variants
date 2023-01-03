mod from_impl;
mod parser;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

#[doc(hidden)]
#[allow(missing_docs)]
#[proc_macro_derive(FromVariants, attributes(from_variants))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Couldn't parse item");
    build_converters(ast)
}

fn build_converters(ast: syn::DeriveInput) -> TokenStream {
    let context = match parser::Container::from_derive_input(&ast) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors().into();
        }
    };
    let bodies = context.as_impls();
    quote!(#(#bodies)*).into()
}
