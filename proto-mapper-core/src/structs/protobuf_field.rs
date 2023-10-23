use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::attributes::{FieldAttrs, StructAttrs};
use crate::types::Ty;
use crate::{find_proto_map_meta, get_proto_field_name};

/// [`StructField`] describes a struct field capable of generating `getter` `setter` implementations
/// for `ProtoMap` and `ProtoMapScalar` traits
///
/// This is the implementation variant for `protobuf` library support
pub(crate) struct StructField {
    pub name: Ident,
    pub ty: Ty,
    pub attrs: Option<FieldAttrs>,
}

#[cfg(test)]
impl StructField {
    #[inline]
    pub(crate) fn is_optional(&self) -> bool {
        self.ty.is_optional()
    }
}

impl StructField {
    pub(crate) fn try_from_field(field: &syn::Field) -> darling::Result<Self> {
        let name = field.ident.as_ref().ok_or_else(|| {
            darling::Error::unsupported_shape("Macro supports only structs with named fields")
        })?;

        let ty = Ty::try_from_field(field)?;

        let meta = find_proto_map_meta(&field.attrs);
        let attrs = if let Some(meta) = meta {
            Some(FieldAttrs::try_from_meta(meta)?)
        } else {
            None
        };

        Ok(Self {
            name: name.clone(),
            ty,
            attrs,
        })
    }

    pub fn determine_to_proto_method(&self) -> TokenStream {
        // First consult field attributes that override struct type
        if let Some(attrs) = &self.attrs {
            match &attrs.with {
                // Override self.ty for scalar, enumeration properties
                None if attrs.scalar => {
                    return quote! { ProtoMapScalar::to_scalar };
                }
                None if attrs.enumeration => {
                    return quote! { ProtoMap::to_proto };
                }

                // TODO protobuf enumeration with override should be done via #with::to_proto interface
                // Override implementation for with module  scalar
                Some(with) if attrs.scalar || attrs.enumeration || self.ty.is_scalar() => {
                    return quote! { #with::to_scalar };
                }
                // Override implementation for with module  non scalar
                Some(with) => {
                    return quote! { #with::to_proto };
                }
                // For all other possibly invalid combinations proceed to defaults (consult self.ty)
                _ => {}
            };
        }

        // If no related attributes found return defaults
        if self.ty.is_scalar() {
            quote! { ProtoMapScalar::to_scalar }
        } else {
            quote! { ProtoMap::to_proto }
        }
    }

    // TODO use struct attrs for rename_all
    /// Specific `protobuf` feature implementation of struct filed getter method.
    pub(crate) fn implement_getter(&self, _struct_attrs: &StructAttrs) -> TokenStream {
        // Fast handle skip attribute
        if let Some(FieldAttrs { skip: true, .. }) = &self.attrs {
            return quote! {};
        }

        //Check field rename
        let proto_field_setter = if let Some(FieldAttrs {
            rename: Some(new_name),
            ..
        }) = &self.attrs
        {
            let field_name = get_proto_field_name(new_name.as_str(), Some('_'));
            format_ident!("set_{}", field_name)
        } else {
            format_ident!("set_{}", &self.name)
        };

        let struct_field = &self.name;

        let to_proto_method = self.determine_to_proto_method();

        if self.ty.is_optional() {
            // Optional field setter
            quote! {
                if let Some(value) = &self.#struct_field {
                    proto.#proto_field_setter(#to_proto_method(value).into());
                }
            }
        } else {
            // Non optional field just a setter
            quote! {
                proto.#proto_field_setter(#to_proto_method(&self.#struct_field).into());
            }
        }
    }
    pub fn determine_from_proto_method(&self) -> TokenStream {
        // First consult field attributes that override struct type
        if let Some(attrs) = &self.attrs {
            match &attrs.with {
                // Override self.ty for scalar, enumeration properties
                None if attrs.scalar => {
                    return quote! { ProtoMapScalar::from_scalar };
                }
                None if attrs.enumeration => return quote! {ProtoMap::from_proto},
                // Override implementation for with module  scalar
                Some(with) if attrs.scalar || attrs.enumeration || self.ty.is_scalar() => {
                    return quote! { #with::from_scalar };
                }
                // Override implementation for with module  non scalar
                Some(with) => {
                    return quote! { #with::from_proto };
                }
                // For all other possibly invalid combinations proceed to defaults (consult self.ty)
                _ => {}
            };
        }

        if self.ty.is_scalar() {
            quote! { ProtoMapScalar::from_scalar }
        } else {
            quote! { ProtoMap::from_proto }
        }
    }

    pub fn determine_has_value_method(&self, proto_field: &Ident) -> TokenStream {
        // First consult field attributes that override struct type
        if let Some(attrs) = &self.attrs {
            // Override has_value for scalar types
            if attrs.scalar {
                return quote! { ProtoScalar::has_value(&value) };
            }
            // Override has_value for enumeration types
            if attrs.enumeration {
                return quote! { ProtoScalar::has_value(&value.value()) };
            }
        }

        if self.ty.is_scalar() {
            quote! { ProtoScalar::has_value(&value) }
        } else {
            let has_field = format_ident!("has_{}", proto_field);
            quote! { proto.#has_field() }
        }
    }

    // TODO use struct attrs for rename_all
    /// Specific `protobuf` feature implementation of struct filed setter method.
    pub(crate) fn implement_setter(&self, _struct_attrs: &StructAttrs) -> TokenStream {
        let struct_field = &self.name;

        // Fast fail skip attribute
        if let Some(FieldAttrs { skip: true, .. }) = &self.attrs {
            // Default struct setter for the skipped fields.
            return quote! { #struct_field: Default::default(), };
        }

        //Check field rename
        let proto_field = if let Some(FieldAttrs {
            rename: Some(new_name),
            ..
        }) = &self.attrs
        {
            let field_name = get_proto_field_name(new_name.as_str(), None);
            format_ident!("{}", field_name)
        } else {
            struct_field.clone() // Here proto and struct field are the same
        };

        let from_proto_method = self.determine_from_proto_method();

        let proto_field_getter = format_ident!("{}", proto_field);

        if self.ty.is_optional() {
            // Determine the appropriate has_value method
            let has_value_method = self.determine_has_value_method(&proto_field);

            // In case of optional check value is empty via `has_value_method`
            quote! {
                #struct_field: {
                    let value = proto.#proto_field_getter().to_owned();
                    if #has_value_method {
                        Some(#from_proto_method(value)?)
                    } else {
                        None
                    }
                },
            }
        } else {
            // Non optional field just a setter
            quote! {
                #struct_field: #from_proto_method(proto.#proto_field_getter().to_owned())?,
            }
        }
    }
}
