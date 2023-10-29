use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use super::{FieldAttrs, StructAttrs};
use crate::types::Ty;
use crate::{find_proto_map_meta, get_proto_field_name};


/// [`StructField`] describes a struct field capable of generating `getter` `setter` implementations
/// for `ProtoMap` and `ProtoMapScalar` traits
///
/// This is the implementation variant for `prost` library support
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
    /// Checks of the field should be treated as a scalar.
    ///
    /// Note: this is not the same as checking `self.ty.is_scalar()` as it takes into account
    /// attribute overrides.
    ///
    /// For example in `prost` (this) implementation proto `message enum ... ` is treated as scalar.
    #[inline]
    pub fn is_scalar_like(&self) -> bool {
        self.ty.is_scalar()
            || matches!(self.attrs, Some(FieldAttrs{enumeration: true, ..}))
            || matches!(self.attrs, Some(FieldAttrs{scalar: true, ..}))
    }
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
                None if attrs.scalar || attrs.enumeration => {
                    return quote! { ProtoMapScalar::to_scalar };
                }
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
    /// Specific `prost` feature implementation of struct filed getter method.
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
            format_ident!("{}", field_name)
        } else {
            self.name.clone()
        };

        let struct_field = &self.name;

        let to_proto_method = self.determine_to_proto_method();

        match (self.is_scalar_like(), self.ty.is_optional()) {
            // scalar - non optional
            (true, false) => {
                quote! {
                    proto.#proto_field_setter = #to_proto_method(&self.#struct_field);
               }
            }

            // scalar - optional
            (true, true) => {
                quote! {
                    if let Some(value) = &self.#struct_field {
                        proto.#proto_field_setter = #to_proto_method(value);
                    }
                }
            }

            // non scalar - non optional
            (false, false) => {
                quote! {
                    proto.#proto_field_setter = Some(#to_proto_method(&self.#struct_field));
                }
            }
            // non scalar - optional
            (false, true) => {
                quote! {
                    if let Some(value) = &self.#struct_field {
                        proto.#proto_field_setter = Some(#to_proto_method(value));
                    }
                }
            }
        }
    }

    pub fn determine_from_proto_method(&self) -> TokenStream {
        // First consult field attributes that override struct type
        if let Some(attrs) = &self.attrs {
            match &attrs.with {
                // Override self.ty for scalar, enumeration properties
                None if attrs.scalar || attrs.enumeration => {
                    return quote! { ProtoMapScalar::from_scalar };
                }
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

    // TODO use struct attrs for rename_all
    /// Specific `prost` feature implementation of struct filed setter method.
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

        let proto_field_getter = &proto_field;

        match (self.is_scalar_like(), self.ty.is_optional()) {
            // scalar - non optional
            (true, false) => {
                quote! {
                    #struct_field: #from_proto_method(proto.#proto_field_getter)?,
                }
            }

            // scalar - optional
            (true, true) => {

                quote! {
                    #struct_field: {
                        let value = proto.#proto_field_getter;
                        if ProtoScalar::has_value(&value) {
                            Some(#from_proto_method(value)?)
                        } else {
                            None
                        }
                    },
                }
            }

            // non scalar - non optional
            (false, false) => {
                quote! {
                    #struct_field: {
                        if let Some(value) = proto.#proto_field_getter {
                            #from_proto_method(value)?
                        } else {
                            Default::default()
                        }
                    },
                }
            }
            // non scalar - optional
            (false, true) => {
                quote! {
                    #struct_field: {
                        if let Some(value) = proto.#proto_field_getter {
                            Some(#from_proto_method(value)?)
                        } else {
                            None
                        }
                    },
                }
            }
        }
    }
}
