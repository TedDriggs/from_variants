use syn;

/// Trait for accessing an object's contents as a word.
pub trait AsWord {
    /// Peers into an object's contents and return them as a string if available.
    fn as_word(&self) -> Option<&str>;
}

/// Peer into a nested meta item and determine if it contains exactly one word.
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