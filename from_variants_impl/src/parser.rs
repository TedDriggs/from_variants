use darling::{
    ast::{Data, Fields, Style},
    util::Ignored,
    FromDeriveInput, FromVariant,
};

use crate::from_impl::FromImpl;

/// A parsing context which houses information read from the input until it
/// can be used to construct the appropriate token stream.
///
/// The `Container` is the workhorse of the macro; it is responsible for traversing
/// the input to populate itself, and then generating a set of `FromImpl` objects
/// which are responsible for the eventual rendering of the conversion implementations.
#[derive(FromDeriveInput)]
#[darling(from_ident, attributes(from_variants), supports(enum_any))]
pub struct Container {
    pub into: bool,
    pub ident: syn::Ident,
    generics: syn::Generics,
    data: Data<Variant, Ignored>,
}

impl Container {
    /// Generates a list of `From` implementations.
    pub fn as_impls(&self) -> Vec<FromImpl<'_>> {
        if let Some(variants) = self.data.as_ref().take_enum() {
            variants
                .into_iter()
                .filter(|v| v.is_enabled())
                .map(|item| FromImpl {
                    generics: &self.generics,
                    variant_ident: &item.ident,
                    variant_ty: item.ty().expect("Shape validation already took place"),
                    target_ident: &self.ident,
                    into: item.into.unwrap_or(self.into),
                })
                .collect()
        } else {
            panic!("FromVariants is not supported on structs");
        }
    }
}

impl From<syn::Ident> for Container {
    fn from(ident: syn::Ident) -> Self {
        Container {
            ident,
            into: false,
            generics: Default::default(),
            data: Data::Enum(vec![]),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, FromVariant)]
#[darling(from_ident, attributes(from_variants), and_then = "Self::validate")]
pub struct Variant {
    ident: syn::Ident,
    skip: Option<bool>,
    into: Option<bool>,
    fields: Fields<syn::Type>,
}

impl Variant {
    fn validate(self) -> darling::Result<Self> {
        if self.is_enabled() && !self.fields.is_newtype() {
            let shape = if self.fields.is_tuple() {
                "tuple"
            } else if self.fields.is_struct() {
                "struct"
            } else if self.fields.is_unit() {
                "unit"
            } else {
                "unknown"
            };
            Err(darling::Error::unsupported_shape(shape).with_span(&self.fields))
        } else {
            Ok(self)
        }
    }

    /// Check if this variant will emit a converter.
    pub fn is_enabled(&self) -> bool {
        !(self.fields.is_unit() || self.skip.unwrap_or(false))
    }

    pub fn ty(&self) -> Option<&syn::Type> {
        if let Fields {
            style: Style::Tuple,
            ref fields,
            ..
        } = self.fields
        {
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
            fields: Fields::new(Style::Unit, Vec::new()),
        }
    }
}
