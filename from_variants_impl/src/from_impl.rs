use syn::parse_quote;
use quote::{ToTokens, Tokens, *};

/// The generic type parameter used when `into` conversions are requested.
const INTO_GENERIC: &'static str = "INTO";

/// A view of data which can generate a `From<T> for Target` impl block.
#[derive(Debug, Clone)]
pub struct FromImpl<'a> {

    /// The generics of the target enum.
    pub generics: &'a syn::Generics,

    /// The identifier of the target enum.
    pub target_ident: &'a syn::Ident,

    /// The identifier of the target variant.
    pub variant_ident: &'a syn::Ident,

    /// The type of the target variant.
    pub variant_ty: &'a syn::Type,

    /// Whether or not this impl should use an `Into` conversion.
    pub into: bool,
}

impl<'a> ToTokens for FromImpl<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let target_ident = &self.target_ident;
        let variant_ident = &self.variant_ident;
        let variant_ty = &self.variant_ty;
        let doc_comment = format!(include_str!("impl_doc.md"), variant = variant_ident);
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        if self.into {
            let mut altered_generics: syn::Generics = self.generics.clone();
            altered_generics.params.push(syn::GenericParam::Type(generate_into_ty_param(variant_ty)));
            let (i, _, _) = altered_generics.split_for_impl();
            let into_variant = syn::Ident::from(INTO_GENERIC);

            tokens.append_all(quote!(
                #[doc = #doc_comment]
                impl #i ::from_variants::export::From<#into_variant> for #target_ident #ty_generics
                    #where_clause {
                    fn from(v: #into_variant) -> Self {
                        #target_ident::#variant_ident(v.into())
                    }
                }
            ))
        } else {
            tokens.append_all(quote!(
                #[doc = #doc_comment]
                impl #impl_generics ::from_variants::export::From<#variant_ty> for #target_ident #ty_generics
                    #where_clause {
                    fn from(v: #variant_ty) -> Self {
                        #target_ident::#variant_ident(v)
                    }
                }
            ))
        }
    }
}

fn generate_into_ty_param(variant_ty: &syn::Type) -> syn::TypeParam {
    let into = syn::Ident::from(INTO_GENERIC);
    parse_quote!(#into: ::from_variants::export::Into<#variant_ty>)
}

#[cfg(test)]
macro_rules! default_from_impl {
    () => (
        {
            use syn;
            FromImpl {
                generics: &Default::default(),
                target_ident: &syn::Ident::from("Foo"),
                variant_ident: &syn::Ident::from("Bar"),
                variant_ty: &syn::Type::Path(parse_quote!(String)),
                into: false,
            }
        }
    )
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;
    use quote::*;
    use super::FromImpl;

    #[test]
    fn simple() {
        let fi = default_from_impl!();
        assert_eq!(quote!(#fi), quote!(
            #[doc = "Convert into [`Bar`](#variant.Bar) variant."]
            impl ::from_variants::export::From<String> for Foo {
                fn from(v: String) -> Self {
                    Foo::Bar(v)
                }
            }
        ));
    }

    #[test]
    fn lifetime() {
        let mut generics = syn::Generics::default();
        generics.params.push(parse_quote!('a));

        let ty = parse_quote!(&'a str);

        let mut fi = default_from_impl!();
        fi.variant_ty = &ty;
        fi.generics = &generics;

        assert_eq!(quote!(#fi), quote!(
            #[doc = "Convert into [`Bar`](#variant.Bar) variant."]
            impl<'a> ::from_variants::export::From<&'a str> for Foo<'a> {
                fn from(v: &'a str) -> Self {
                    Foo::Bar(v)
                }
            }
        ));
    }

    #[test]
    fn into() {
        let mut fi = default_from_impl!();
        fi.into = true;

        assert_eq!(quote!(#fi), quote!(
            #[doc = "Convert into [`Bar`](#variant.Bar) variant."]
            impl<INTO: ::from_variants::export::Into<String> > ::from_variants::export::From<INTO> for Foo {
                fn from(v: INTO) -> Self {
                    Foo::Bar(v.into())
                }
            }
        ));
    }

    #[test]
    fn into_generics() {
        let mut generics: syn::Generics = syn::Generics::default();
        let ty = parse_quote!(Vec<T>);
        generics.params.push(parse_quote!(T));
        let mut fi = default_from_impl!();
        fi.variant_ty = &ty;
        fi.generics = &generics;
        fi.into = true;

        assert_eq!(quote!(#fi), quote!(
            #[doc = "Convert into [`Bar`](#variant.Bar) variant."]
            impl<T, INTO: ::from_variants::export::Into<Vec<T> > > ::from_variants::export::From<INTO> for Foo<T> {
                fn from(v: INTO) -> Self {
                    Foo::Bar(v.into())
                }
            }
        ));
    }
}
