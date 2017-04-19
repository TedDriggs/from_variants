use syn;
use quote::{ToTokens, Tokens};

use prelude::Bindings;

/// A view of data which can generate a `From<T> for Target` impl block.
#[derive(Debug, Clone)]
pub struct FromImpl<'a> {
    /// The set of library bindings to generate against (core or std).
    pub bindings: Bindings,
    
    /// The generics of the target enum.
    pub generics: &'a syn::Generics,
    
    /// The identifier of the target enum.
    pub target_ident: &'a syn::Ident,
    
    /// The identifier of the target variant.
    pub variant_ident: &'a syn::Ident,
    
    /// The type of the target variant.
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
    use syn;
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
    
    #[test]
    fn lifetime() {
        let mut generics = syn::Generics::default();
        generics.lifetimes.push(syn::LifetimeDef::new("'a"));
        
        let ty = syn::parse_type("&'a str").unwrap();
        
        let mut fi = default_from_impl!();
        fi.variant_ty = &ty;
        fi.generics = &generics;
        
        assert_eq!(quote!(#fi), quote!(
            #[doc = "Convert into a `Bar` variant."]
            impl<'a> ::std::convert::From<&'a str> for Foo<'a> {
                fn from(v: &'a str) -> Self {
                    Foo::Bar(v)
                }
            }
        ));
    }
}