use std::marker::PhantomData;

use syn;

use errors::*;
use bindings::Bindings;
use from_impl::FromImpl;
use state::*;
use util::AsWord;

const ATTR_NAME: &'static str = "from_variants";

/// A parsing context which houses information read from the input until it
/// can be used to construct the appropriate token stream.
///
/// The `Context` is the workhorse of the macro; it is responsible for traversing
/// the input to populate itself, and then generating a set of `FromImpl` objects
/// which are responsible for the eventual rendering of the conversion implementations.
pub struct Context<S: State> {
    pub bindings: Bindings,
    pub target_ident: syn::Ident,
    generics: syn::Generics,
    type_mapping: Vec<TypeMapping>,
    state: PhantomData<S>,
}

impl Context<Generating> {
    /// Generates a list of `From` implementations.
    pub fn as_impls<'a>(&'a self) -> Vec<FromImpl<'a>> {
        self.type_mapping.iter().map(|item| {
            FromImpl {
                bindings: self.bindings.clone(),
                generics: &self.generics,
                variant_ident: &item.variant,
                variant_ty: &item.source,
                target_ident: &self.target_ident,
            }
        }).collect()
    }
    
    /// Read the input enum and generate a parsing context or return an error.
    pub fn parse(input: syn::DeriveInput) -> Result<Self> {
        let mut ctx = Context::new(input.ident, input.generics);
        Ok(ctx.parse_attributes(input.attrs)?.parse_body(input.body)?.finish())
    }
}

impl Context<Parsing> {
    /// Creates a new parsing context using default bindings.
    fn new(target: syn::Ident, generics: syn::Generics) -> Self {
        Context {
            bindings: Default::default(),
            target_ident: target,
            generics: generics,
            type_mapping: vec![],
            state: PhantomData,
        }
    }
    
    /// Read attributes off the target enum and update corresponding context properties.
    fn parse_attributes(&mut self, attrs: Vec<syn::Attribute>) -> Result<&mut Self> {
        for attr in attrs.into_iter().filter(is_attr_relevant) {
            self.parse_attribute(attr)?;
        }
        
        Ok(self)
    }
    
    /// Parse an individual `#[from_variants(...)]` attribute at the enum level.
    fn parse_attribute(&mut self, attr: syn::Attribute) -> Result<()> {
        if let syn::MetaItem::List(ref _ident, ref nested_attrs) = attr.value {
            for item in nested_attrs {
                self.parse_meta_item(item)?;
            }
            
            Ok(())
            
        } else {
            bail!("Expected MetaItem::List, found `{:?}`", attr.value);
        }
    }
    
    /// Parse a meta item from the enum-level attribute.
    /// 
    /// # Errors
    /// * Returns an error for unsupported attribute words.
    /// * Returns an error for non-word meta-items.
    fn parse_meta_item(&mut self, nested: &syn::NestedMetaItem) -> Result<&mut Self> {
        match nested.as_word() {
            Some("no_std") => {
                self.bindings = Bindings::NoStd;
                Ok(self)
            },
            Some(wd) => bail!("Unknown attribute word `{}`", wd),
            None => bail!("Unknown attribute `{:?}`", nested),
        }
    }
    
    /// Parse the body of an enum, generating `TypeMapping` instances for
    /// each non-skipped, non-unit variant. Returns an error if any non-skipped
    /// variants are unsupported by the crate.
    fn parse_body(&mut self, body: syn::Body) -> Result<&mut Self> {
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
    
    /// Finish parsing the enum and update the context to be ready to generate
    /// a list of `From` implementations.
    fn finish(&self) -> Context<Generating> {
        Context {
            bindings: self.bindings.clone(),
            generics: self.generics.clone(),
            target_ident: self.target_ident.clone(),
            type_mapping: self.type_mapping.clone(),
            state: PhantomData,
        }
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
    /// * Passing `#[from_variants(skip)]` as an attribute will produce `None`.
    /// * Unit variants are supported and produce `None`.
    /// * Struct variants are not supported.
    /// * Tuple variants are handled by `TypeMapping::parse_source_ty`.
    pub fn parse(variant: syn::Variant) -> Result<Option<Self>> {
        use syn::VariantData;
        
        if !Self::parse_attributes(variant.attrs) {
            return Ok(None);
        }
        
        match variant.data {
            VariantData::Unit => Ok(None),
            VariantData::Struct(_) => bail!(ErrorKind::StructVariantsUnsupported),
            VariantData::Tuple(fields) => {
                Ok(Some(TypeMapping::new(Self::parse_source_ty(fields)?, variant.ident)))
            }
        }
    }
    
    /// Parse an individual `#[from_variants(...)]` attribute at the variant level, and 
    /// returns `true` if a TypeMapping should be generated.
    fn parse_attributes(attributes: Vec<syn::Attribute>) -> bool {
        
        // TODO fix the return type of this method to adhere to others.
        for attr in attributes.into_iter().filter(is_attr_relevant) {
            if let syn::MetaItem::List(ref _ident, ref nested_attrs) = attr.value {
                for item in nested_attrs {
                    return Self::parse_meta_item(item).unwrap();
                }
            } else {
                // TODO switch this to use the `Result` pattern elsewhere in the library.
                panic!("Expected MetaItem::List, found `{:?}`", attr.value);
            }
        }
        
        return true;
    }
    
    fn parse_meta_item(item: &syn::NestedMetaItem) -> Result<bool> {
        match item.as_word() {
            Some("skip") => Ok(false),
            _ => bail!("Unknown option: `{:?}`", item)
        }
    }
    
    /// Extract the conversion source type for a tuple variant. This produces
    /// an error unless the tuple variant has exactly 1 field; this is referred
    /// to as a "newtype" variant.
    fn parse_source_ty(fields: Vec<syn::Field>) -> Result<syn::Ty> {
        let field_count = fields.len();
        let mut field_ty = fields.into_iter().map(|field| field.ty);
        match field_count {
            0 => bail!(ErrorKind::TupleTooShort),
            1 => Ok(field_ty.next().expect("Known to have 1 field")),
            // TODO add support for tuples.
            _ => bail!(ErrorKind::TupleTooLong),
        }
    }
}

/// Checks if an attribute is relevant to the `from_variants` macro.
fn is_attr_relevant(attr: &syn::Attribute) -> bool {
    !attr.is_sugared_doc && attr.style == syn::AttrStyle::Outer && attr.name() == ATTR_NAME
}