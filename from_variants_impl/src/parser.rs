use darling::{
    ast::{Data, Fields, Style},
    util::Ignored,
    FromDeriveInput, FromVariant, Result,
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
    pub fn validate(self) -> Result<Self> {
        let variant_errors = self
            .data
            .as_ref()
            .take_enum()
            .expect("FromVariants is not valid on structs")
            .into_iter()
            .filter_map(|v| v.validate().err())
            .collect::<Vec<_>>();

        if !variant_errors.is_empty() {
            return Err(darling::Error::multiple(variant_errors));
        }

        Ok(self)
    }

    /// Generates a list of `From` implementations.
    pub fn as_impls(&self) -> Vec<FromImpl<'_>> {
        self.data
            .as_ref()
            .take_enum()
            .expect("FromVariants is not valid on structs")
            .into_iter()
            .filter(|v| v.is_enabled())
            .map(|item| FromImpl {
                generics: &self.generics,
                variant_ident: &item.ident,
                variant_ty: item.ty().unwrap(),
                target_ident: &self.ident,
                into: item.into.unwrap_or(self.into),
            })
            .collect()
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
#[darling(from_ident, attributes(from_variants))]
pub struct Variant {
    ident: syn::Ident,
    skip: Option<bool>,
    into: Option<bool>,
    fields: Fields<syn::Type>,
}

impl Variant {
    fn validate(&self) -> Result<()> {
        if self.is_enabled() && !self.fields.is_newtype() {
            Err(darling::Error::custom(format!("FromVariants only supports newtype variants; other variants must be skipped with `#[from_variants(skip)]`")).with_span(&self.ident))
        } else {
            Ok(())
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
            fields: Fields {
                style: Style::Unit,
                fields: Vec::new(),
            },
        }
    }
}
