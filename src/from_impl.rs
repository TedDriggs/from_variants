use syn;
use quote::{ToTokens, Tokens};

use prelude::Bindings;

#[derive(Debug, Clone)]
pub struct FromImpl<'a> {
    pub bindings: Bindings,
    pub generics: &'a syn::Generics,
    pub target_ident: &'a syn::Ident,
    pub variant_ident: &'a syn::Ident,
    pub variant_ty: &'a syn::Ty,
}

impl<'a> ToTokens for FromImpl<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let from_trait = self.bindings.from_trait();
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let target_ident = &self.target_ident;
        let variant_ident = &self.variant_ident;
        let variant_ty = &self.variant_ty;
        let doc_comment = format!(include_str!("impl_doc.md"), variant = variant_ident);
        
        tokens.append(quote!(
            #[doc = #doc_comment]
            impl #impl_generics #from_trait<#variant_ty> for #target_ident #ty_generics
                #where_clause {
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
                generics: &Default::default(),
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
            #[doc = "Convert into a `Bar` variant."]
            impl ::std::convert::From<String> for Foo {
                fn from(v: String) -> Self {
                    Foo::Bar(v)
                }
            }
        ));
    }
}