use darling::FromMeta;
use syn::Path;

/// Meta attributes for `struct` items
#[derive(Debug, darling::FromMeta)]
pub(crate) struct StructAttrs {
    pub source: Path,
    /// Optional renaming of the struct fields before mapping to the proto entity.
    pub rename_all: Option<String>,
}

/// Meta attributes for `struct field` items
#[derive(Debug, darling::FromMeta, Default)]
#[darling(default)]
pub(crate) struct FieldAttrs {
    /// Optional skipping struct field from proto serialization.
    pub skip: bool,
    /// Optional mark the field as an scalar type mapping.
    pub scalar: bool,
    /// Optional mark the field as an enumeration mapping (used only for optional getter/setter mapping).
    pub enumeration: bool,
    /// Optional module with implementation of override mappings (implementation depends on scalar, enumeration or other proto destination type)
    pub with: Option<Path>,
    /// Optional renaming of a single struct field before mapping to the proto entity.
    pub rename: Option<String>,
}

impl FieldAttrs {
    pub(crate) fn try_from_meta(meta: &syn::Meta) -> darling::Result<Self> {
        let attrs = FieldAttrs::from_meta(meta)?;
        attrs.validate()
    }

    fn validate(self) -> darling::Result<Self> {
        if self.enumeration && self.scalar {
            return Err(darling::Error::unsupported_shape("Struct attributes `enumeration` and `scalar` are mutually excluded (use only one of them)"));
        }
        Ok(self)
    }
}
