use syn;
use quote::{ToTokens, Tokens};

use prelude::Bindings;

#[derive(Debug, Clone)]
pub struct FromImpl<'a> {
    pub bindings: Bindings,
    pub target_ident: &'a syn::Ident,
    pub variant_ident: &'a syn::Ident,
    pub variant_ty: &'a syn::Ty,
}

impl<'a> ToTokens for FromImpl<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let from_trait = self.bindings.from_trait();
        let target_ident = &self.target_ident;
        let variant_ident = &self.variant_ident;
        let variant_ty = &self.variant_ty;
        
        tokens.append(quote!(
            impl #from_trait<#variant_ty> for #target_ident {
                fn from(v: #variant_ty) -> Self {
                    #target_ident::#variant_ident(v)
                }
            }
        ))
    }
}


macro_rules! default_from_impl {
    () => (
        {
            use syn;
            FromImpl {
                bindings: Default::default(),
                target_ident: &syn::Ident::new("Foo"),
                variant_ident: &syn::Ident::new("Bar"),
                variant_ty: &syn::parse_type("String").expect("default_from_impl should produce valid type"),
            }
        }
    )
}

#[cfg(test)]
mod tests {
    use super::FromImpl;
    
    #[test]
    fn simple() {
        let fi = default_from_impl!();
        assert_eq!(quote!(#fi), quote!(
            impl ::std::convert::From<String> for Foo {
                fn from(v: String) -> Self {
                    Foo::Bar(v)
                }
            }
        ));
    }
}