use crate::enums::Enum;
use crate::structs::Struct;
use heck::ToSnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{Data, DeriveInput};

pub(crate) enum ProtoMap {
    Struct(Struct),
    Enum(Enum),
}

impl ProtoMap {
    fn name(&self) -> &Ident {
        match self {
            Self::Struct(inner) => &inner.name,
            Self::Enum(inner) => &inner.name,
        }
    }
    fn implement_proto_map(&self) -> TokenStream {
        match self {
            Self::Struct(data) => data.implement_proto_map(),
            Self::Enum(data) => data.implement_proto_map(),
        }
    }
}

impl ToTokens for ProtoMap {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mod_name = format_ident!("proto_map_impl_{}", self.name().to_string().to_snake_case());

        let proto_convert = self.implement_proto_map();

        let expanded = quote! {
            mod #mod_name {
                use super::*;
                #proto_convert
            }
        };

        tokens.extend(expanded)
    }
}

impl darling::FromDeriveInput for ProtoMap {
    fn from_derive_input(input: &DeriveInput) -> darling::Result<Self> {
        match &input.data {
            Data::Struct(data) => {
                let s = Struct::try_from_data(&input.ident, data, &input.attrs)?;
                Ok(ProtoMap::Struct(s))
            }
            Data::Enum(data) => Ok(ProtoMap::Enum(Enum::try_from_data(
                &input.ident,
                data,
                input.attrs.as_ref(),
            )?)),
            _ => Err(darling::Error::unsupported_shape(
                "Macro supports only `struct` and `enum` items",
            )),
        }
    }
}
