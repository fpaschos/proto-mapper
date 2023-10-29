use darling::FromMeta;
use proc_macro2::Ident;
use syn::{Attribute, Path};
use crate::find_proto_map_meta;

/// Meta attribute used in `enum` items to mark one_of field name
#[derive(Debug, FromMeta, PartialEq)]
pub(crate) struct OneOf {
    pub field: Ident,
}

/// Meta attributes for `enum` items.
#[derive(Debug, FromMeta)]
pub(crate) struct EnumAttrs {
    /// The source proto entity that we map to
    pub source: Path,

    /// Indicates that the proto entity that we map to has an `one_of` enumeration structure.
    /// Mutually exclusive with `enumeration`.
    pub one_of: Option<OneOf>,

    /// Indicates that the proto entity is a simple enumeration.
    /// Mutually exclusive with `one_of`
    pub enumeration: Option<bool>,

    /// Optional renaming of the variant fields before mapping to the proto entity.
    pub rename_variants: Option<String>,
}

impl EnumAttrs {
    pub(crate) fn is_enumeration(&self) -> bool {
        self.enumeration.is_some_and(|e| e)
    }

    fn validate(self) -> darling::Result<Self> {
        if self.is_enumeration() && self.one_of.is_some() {
            return Err(darling::Error::unsupported_shape("Enum attributes `enumeration` and `one_of` are mutually excluded (use only one of them)"));
        }
        Ok(self)
    }
}

impl TryFrom<&[Attribute]> for EnumAttrs {
    type Error = darling::Error;

    fn try_from(attrs: &[Attribute]) -> Result<Self, Self::Error> {
        let meta = find_proto_map_meta(attrs).ok_or_else(|| {
            darling::Error::unsupported_shape("Missing meta attribute `proto_map`")
        })?;
        let attrs = Self::from_meta(meta)?;
        attrs.validate()
    }
}
