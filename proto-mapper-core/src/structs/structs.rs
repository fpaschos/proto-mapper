use crate::attributes::StructAttrs;
use crate::find_proto_map_meta;
use crate::structs::StructField;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{Attribute, DataStruct};

/// Macro implementor of `struct` items.
pub(crate) struct Struct {
    pub name: Ident,
    pub attrs: StructAttrs,
    pub fields: Vec<StructField>,
}

impl Struct {
    pub(crate) fn try_from_data(
        name: &Ident,
        data: &DataStruct,
        attrs: &[Attribute],
    ) -> darling::Result<Self> {
        let meta = find_proto_map_meta(attrs).ok_or_else(|| {
            darling::Error::unsupported_shape("Missing required proto attribute `proto_map`")
        })?;

        let attrs = StructAttrs::from_meta(meta)?;

        let fields = data
            .fields
            .iter()
            .map(StructField::try_from_field)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            name: name.clone(),
            fields,
            attrs,
        })
    }

    /// Implementation of proto_map for `struct` items
    pub(crate) fn implement_proto_map(&self) -> TokenStream {
        let struct_name = format_ident!("{}", &self.name);
        let proto_struct = &self.attrs.source;
        let to_proto_impl = {
            let fields = self.fields.iter().map(|f| f.implement_getter(&self.attrs));

            quote! {
                let mut proto = #proto_struct::default();
                #(#fields)*
                proto
            }
        };

        let from_proto_impl = {
            let fields = self.fields.iter().map(|f| f.implement_setter(&self.attrs));

            quote! {
                let inner = Self {
                    #(#fields)*
                };
                Ok(inner)
            }
        };

        quote! {
            impl ProtoMap for #struct_name {
                type ProtoStruct = #proto_struct;
                fn to_proto(&self) -> Self::ProtoStruct {
                    #to_proto_impl
                }

                fn from_proto(proto: Self::ProtoStruct) -> std::result::Result<Self, anyhow::Error> {
                    #from_proto_impl
                }
            }
        }
    }
}
