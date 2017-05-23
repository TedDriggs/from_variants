use darling::util::{Body, VariantData};
use syn;

use from_impl::FromImpl;

/// A parsing context which houses information read from the input until it
/// can be used to construct the appropriate token stream.
///
/// The `Container` is the workhorse of the macro; it is responsible for traversing
/// the input to populate itself, and then generating a set of `FromImpl` objects
/// which are responsible for the eventual rendering of the conversion implementations.
#[derive(FromDeriveInput)]
#[darling(from_ident, attributes(from_variants))]
pub struct Container {
    pub into: bool,
    pub ident: syn::Ident,
    generics: syn::Generics,
    body: Body<Variant, Field>,
}

impl Container {
    /// Generates a list of `From` implementations.
    pub fn as_impls<'a>(&'a self) -> Vec<FromImpl<'a>> {
        if let Some(variants) = self.body.as_ref().take_enum() {
            variants.into_iter().filter(|v| v.is_enabled()).map(|item| {
                FromImpl {
                    generics: &self.generics,
                    variant_ident: &item.ident,
                    variant_ty: item.ty().unwrap(),
                    target_ident: &self.ident,
                    into: item.into.unwrap_or(self.into),
                }
            }).collect()
        } else {
            Vec::new()
        }
    }
}

impl From<syn::Ident> for Container {
    fn from(ident: syn::Ident) -> Self {
        Container {
            ident,
            into: false,
            generics: Default::default(),
            body: Body::Enum(vec![]),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, FromVariant)]
#[darling(from_ident, attributes(from_variants), map = "Self::validate")]
pub struct Variant {
    ident: syn::Ident,
    skip: Option<bool>,
    into: Option<bool>,
    data: VariantData<syn::Ty>,
}

impl Variant {
    fn validate(self) -> Self {
        if self.is_enabled() && !self.data.is_newtype() {
            panic!("Variants must be newtype or unit");
        }

        self
    }
    
    /// Check if this variant will emit a converter.
    pub fn is_enabled(&self) -> bool {
        !(self.data.is_unit() || self.skip.unwrap_or(false))
    }

    pub fn ty(&self) -> Option<&syn::Ty> {
        if let VariantData::Tuple(ref fields) = self.data {
            fields.get(0)
        } else {
            None
        }
    }
}

impl From<syn::Ident> for Variant {
    fn from(ident: syn::Ident) -> Self {
        Variant {
            ident,
            skip: Default::default(),
            into: Default::default(),
            data: VariantData::Unit,
        }
    }
}

#[derive(Debug, FromField)]
struct Field;