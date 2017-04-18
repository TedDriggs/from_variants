use std::marker::PhantomData;

use syn;

use errors::*;
use bindings::Bindings;
use from_impl::FromImpl;
use state::*;

pub struct Context<S: State> {
    pub bindings: Bindings,
    pub target_ident: syn::Ident,
    type_mapping: Vec<TypeMapping>,
    state: PhantomData<S>,
}

impl Context<Parsing> {
    /// Creates a new parsing context using default bindings.
    pub fn new(target: syn::Ident) -> Self {
        Context {
            bindings: Default::default(),
            target_ident: target,
            type_mapping: vec![],
            state: PhantomData,
        }
    }
    
    pub fn parse_attributes(&mut self, attrs: Vec<syn::Attribute>) -> Result<&mut Self> {
        for attr in attrs {
            self.parse_attribute(attr)?;
        }
        
        Ok(self)
    }
    
    fn parse_attribute(&mut self, attr: syn::Attribute) -> Result<&mut Self> {
        const ATTR_NAME: &'static str = "from_variants";
        if attr.name() == ATTR_NAME && attr.style == syn::AttrStyle::Outer {
            if let syn::MetaItem::List(ref _ident, ref nested_attrs) = attr.value {
                for item in nested_attrs {
                    self.parse_meta_item(item)?;
                }
            } else {
                bail!("Expected MetaItem::List, found `{:?}`", attr.value);
            }
        }
        
        Ok(self)
    }
    
    fn parse_meta_item(&mut self, nested: &syn::NestedMetaItem) -> Result<&mut Self> {
        use syn::{NestedMetaItem, MetaItem};
        match *nested {
            NestedMetaItem::MetaItem(MetaItem::Word(ref ident)) => match ident.as_ref() {
                "no_std" => {
                    self.bindings = Bindings::NoStd;
                    Ok(self)
                },
                wd => bail!("Unknown attribute word `{}`", wd)
            },
            ref n => bail!("Unsupported attribute `{:?}`", n)
        }
    }
    
    pub fn parse_body(&mut self, body: syn::Body) -> Result<&mut Self> {
        match body {
            syn::Body::Struct(_) => bail!(ErrorKind::StructsUnsupported),
            syn::Body::Enum(variants) => {
                let mut impls = Vec::with_capacity(variants.len());
                for parse_result in variants.into_iter().map(TypeMapping::parse) {
                    if let Some(fi) = parse_result? {
                        impls.push(fi);
                    }
                }
                
                self.type_mapping = impls;
                
                Ok(self)
            }
        }
    }
    
    pub fn finish(&self) -> Context<Generating> {
        Context {
            bindings: self.bindings.clone(),
            target_ident: self.target_ident.clone(),
            type_mapping: self.type_mapping.clone(),
            state: PhantomData,
        }
    }
}

impl Context<Generating> {
    /// Generates a list of `From` implementations.
    pub fn as_impls<'a>(&'a self) -> Vec<FromImpl<'a>> {
        self.type_mapping.iter().map(|item| {
            FromImpl {
                bindings: self.bindings.clone(),
                variant_ident: &item.variant,
                variant_ty: &item.source,
                target_ident: &self.target_ident,
            }
        }).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeMapping {
    pub source: syn::Ty,
    pub variant: syn::Ident,
}

impl TypeMapping {
    /// Create a new type mapping from a source type to a variant.
    pub fn new(source: syn::Ty, variant: syn::Ident) -> Self {
        TypeMapping {
            source: source,
            variant: variant,
        }
    }
    
    /// Generate a TypeMapping from a variant, if one is appropriate.
    ///
    /// 1. Unit variants are supported, but produce nothing.
    /// 1. Newtype variants are supported, and produce a conversion.
    /// 1. Tuple variants with multiple parts are not currently supported.
    /// 1. Struct variants are not supported.
    pub fn parse(variant: syn::Variant) -> Result<Option<Self>> {
        use syn::VariantData;
        match variant.data {
            VariantData::Unit => Ok(None),
            VariantData::Struct(_) => bail!(ErrorKind::StructVariantsUnsupported),
            VariantData::Tuple(fields) => {
                Ok(Some(TypeMapping::new(Self::parse_source_ty(fields)?, variant.ident)))
            }
        }
    }
    
    fn parse_source_ty(fields: Vec<syn::Field>) -> Result<syn::Ty> {
        let field_count = fields.len();
        let mut field_ty = fields.into_iter().map(|field| field.ty);
        match field_count {
            0 => bail!(ErrorKind::TupleTooShort),
            1 => Ok(field_ty.next().expect("Known to have 1 field")),
            _ => bail!(ErrorKind::TupleTooLong),
        }
    }
}