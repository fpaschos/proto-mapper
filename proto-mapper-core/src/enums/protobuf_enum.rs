use crate::rename_item;
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Attribute, DataEnum, Fields, Path, Type, Variant};
use crate::enums::EnumAttrs;

#[derive(Debug)]
pub(crate) struct Enum {
    pub name: Ident,
    pub attrs: EnumAttrs,
    pub variants: Vec<EnumVariant>,
}

impl Enum {
    pub(crate) fn try_from_data(
        name: &Ident,
        data: &DataEnum,
        attrs: &[Attribute],
    ) -> darling::Result<Self> {
        let attrs = EnumAttrs::try_from(attrs)?;

        let variants: Vec<EnumVariant> = data
            .variants
            .iter()
            .map(|variant| {
                if attrs.is_enumeration() {
                    EnumVariant::try_from_enumeration_variant(variant)
                } else {
                    EnumVariant::try_from_unnamed_variant(variant)
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            name: name.clone(),
            attrs,
            variants,
        })
    }

    /// Implementation of (`to_proto_impl`, `from_proto_impl`) for `enumeration` variant cases.
    fn implement_enumeration(&self) -> (TokenStream, TokenStream) {
        // Proto struct name
        let proto_struct = &self.attrs.source;

        let to_proto_impl = {
            let match_arms = self.variants.iter().map(|variant| {
                let variant_name = &variant.name;
                let proto_variant_name = self.get_proto_variant_name(variant);
                let proto_variant_name = Ident::new(&proto_variant_name, Span::call_site());
                quote! {
                    Self::#variant_name =>  #proto_struct::#proto_variant_name,
                }
            });
            quote! {
                match self {
                 #( #match_arms )*
                }
            }
        };

        let from_proto_impl = {
            let match_arms = self.variants.iter().map(|variant| {
                let variant_name = &variant.name;
                let proto_variant_name = self.get_proto_variant_name(variant);
                let proto_variant_name = Ident::new(&proto_variant_name, Span::call_site());

                quote! {
                    #proto_struct::#proto_variant_name => Ok(Self::#variant_name),
                }
            });

            // We map to a protobuf entity enumeration
            quote! {
                match proto {
                     #( #match_arms )*
                }
            }
        };

        (to_proto_impl, from_proto_impl)
    }

    /// Implementation of (`to_proto_impl`, `from_proto_impl`) for `one_of` variant cases.
    fn implement_one_of(&self) -> (TokenStream, TokenStream) {
        // Variant outer name
        let name = &self.name;

        let to_proto_impl = {
            let match_arms = self.variants.iter().map(|variant| {
                let variant_name = &variant.name;
                let proto_variant_name = self.get_proto_variant_name(variant);

                let setter = Ident::new(&format!("set_{}", proto_variant_name), Span::call_site());
                quote! {
                     Self::#variant_name(value) => inner.#setter(value.to_proto()),
                }
            });

            quote! {
                let mut inner = Self::ProtoStruct::new();
                match self {
                    #( #match_arms )*
                }
                inner
            }
        };

        let from_proto_impl = {
            // We map to a protobuf entity with oneof field

            // One of field as defined in the .proto file
            // Unwrap here never fails
            let one_of_field = &self.attrs.one_of.as_ref().unwrap().field;

            // Enumeration inner type and module generated by 'protobuf'
            let proto_one_of_enum = {
                let mut source_module_name = self.attrs.source.clone();

                let one_of = source_module_name
                    .segments
                    .pop()
                    .unwrap()
                    .value()
                    .ident
                    .clone();
                let one_of_enum =
                    Ident::new(&one_of.to_string().to_snake_case(), Span::call_site());
                let variant = Ident::new(
                    &one_of_field.to_string().to_upper_camel_case(),
                    Span::call_site(),
                );

                quote! { #source_module_name #one_of_enum::#variant }
            };

            let match_arms = self.variants.iter().map(|variant| {
                let variant_name = &variant.name;
                let field_name = &variant.field_name;
                quote! {
                    Some(#proto_one_of_enum::#variant_name(value)) => {
                        #field_name::from_proto(value).map(Self::#variant_name)
                    }
                }
            });

            quote! {
                match proto.#one_of_field {
                     #( #match_arms )*
                     _ => Err(anyhow::anyhow!(stringify!(Failed to decode enum #name from proto entity)))
                }
            }
        };
        (to_proto_impl, from_proto_impl)
    }

    /// Implementation of proto_map for `enum` items
    pub(crate) fn implement_proto_map(&self) -> TokenStream {
        // Variant outer name
        let name = &self.name;
        // Proto struct name
        let proto_struct = &self.attrs.source;

        let (to_proto_impl, from_proto_impl) = if self.attrs.is_enumeration() {
            self.implement_enumeration()
        } else {
            self.implement_one_of()
        };

        quote! {
            impl ProtoMap for #name {
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
    fn get_proto_variant_name(&self, variant: &EnumVariant) -> String {
        if let Some(rename_variants) = self.attrs.rename_variants.as_ref() {
            rename_item(&variant.name.to_string(), rename_variants).unwrap()
        } else {
            variant.name.to_string()
        }
    }
}

#[derive(Debug)]
pub(crate) struct EnumVariant {
    pub name: Ident,
    pub field_name: Option<Path>,
}

impl EnumVariant {
    pub(crate) fn try_from_enumeration_variant(variant: &Variant) -> darling::Result<Self> {
        if let Fields::Unnamed(fields) = &variant.fields {
            if !fields.unnamed.is_empty() {
                return Err(darling::Error::unsupported_shape(
                    "Only unnamed variants with no inner field e.g. `Foo, Bar` are supported for `enumeration` attributed enums.",
                ));
            }
        }

        let name = variant.ident.clone();

        Ok(Self {
            name,
            field_name: None,
        })
    }
    pub(crate) fn try_from_unnamed_variant(variant: &Variant) -> darling::Result<Self> {
        let field_name = if let Fields::Unnamed(fields) = &variant.fields {
            if fields.unnamed.len() != 1 {
                return Err(darling::Error::unsupported_shape(
                    "Only unnamed variants with only one inner field e.g. `Foo(Bar)` are supported for `one_of` attributed enums.",
                ));
            }

            // Note: .first() here never fails
            match &fields.unnamed.first().unwrap().ty {
                Type::Path(type_path) => Ok(type_path.path.clone()),
                _ => Err(darling::Error::unsupported_shape(
                    "Only unnamed variants with only one inner field e.g. `Foo(Bar)` are supported for `one_of` attributed enums.",
                )),
            }
        } else {
            Err(darling::Error::unsupported_shape(
                "Only unnamed variants with only one inner field e.g. `Foo(Bar)` are supported for `one_of` attributed enums.",
            ))
        }?;

        let name = variant.ident.clone();

        Ok(Self {
            name,
            field_name: Some(field_name),
        })
    }
}