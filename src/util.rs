use syn;

pub trait AsWord {
    fn as_word(&self) -> Option<&str>;
}

impl AsWord for syn::NestedMetaItem {
    fn as_word(&self) -> Option<&str> {
        use syn::{NestedMetaItem, MetaItem};
        if let NestedMetaItem::MetaItem(MetaItem::Word(ref ident)) = *self {
            Some(ident.as_ref())
        } else {
            None
        }
    }
}

impl AsWord for syn::MetaItem {
    fn as_word(&self) -> Option<&str> {
        if let syn::MetaItem::Word(ref ident) = *self {
            Some(ident.as_ref())
        } else {
            None
        }
    }
}