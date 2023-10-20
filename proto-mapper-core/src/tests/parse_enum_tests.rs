use std::ops::Deref;

use quote::quote;
use syn::{parse_quote, Data, DeriveInput, Path};

use crate::proto_enum::{EnumAttrs, EnumVariant};

#[test]
fn parse_enum_one_of_attributes_test() {
    let fragment = quote! {
        #[derive(Debug, ProtoMap, PartialEq)]
        #[proto_map(
            source = "proto::HierarchyEntity",
            one_of(field = "data"),
            rename_variants = "snake_case"
        )]
        enum HierarchyEntity {
            FirstEntity(Entity),
        }
    };

    let input = syn::parse2::<DeriveInput>(fragment.into()).unwrap();

    let attrs = EnumAttrs::try_from(input.attrs.deref()).unwrap();
    let expected_source: Path = parse_quote! { proto::HierarchyEntity };
    assert_eq!(attrs.source, expected_source);
    assert!(!attrs.is_enumeration());
    if let Some(one_of) = attrs.one_of {
        assert_eq!(one_of.field, "data")
    } else {
        assert!(false, "Missing one of attribute")
    }

    assert_eq!(attrs.rename_variants, Some("snake_case".to_string()))
}

#[test]
fn parse_enumeration_attributes_test() {
    let fragment = quote! {
        #[derive(Debug, ProtoMap, PartialEq)]
        #[proto_map(
            source = "proto::Entity",
            enumeration,
            rename_variants = "STREAMING_SNAKE_CASE"
        )]
        enum Entity {
            One,
            Two,
            Three,
        }
    };

    let input = syn::parse2::<DeriveInput>(fragment.into()).unwrap();

    let attrs = EnumAttrs::try_from(input.attrs.deref()).unwrap();
    let expected_source: Path = parse_quote! { proto::Entity };
    assert_eq!(attrs.source, expected_source);
    assert!(attrs.is_enumeration());
    assert_eq!(attrs.one_of, None);

    assert_eq!(
        attrs.rename_variants,
        Some("STREAMING_SNAKE_CASE".to_string())
    )
}

#[test]
fn parse_enumeration_one_of_attributes_mutual_exclusive_test() {
    let fragment = quote! {
        #[derive(Debug, ProtoConvert, PartialEq)]
        #[proto_map(
            source = "proto::Entity",
            enumeration,
            one_of(field="foo")
        )]
        enum Entity {
            One,
            Two,
            Three,
        }
    };

    let input = syn::parse2::<DeriveInput>(fragment.into()).unwrap();

    if let Ok(_attrs) = EnumAttrs::try_from(input.attrs.deref()) {
        panic!("Expected mutual exclusion error on `enumeration` and `one_of` attributes")
    }
}

#[test]
fn parse_unnamed_variant_success_test() {
    let fragment = quote! {
        #[proto_map(
            source = "proto::HierarchyEntity",
            one_of(field = "data"),
        )]
        enum HierarchyEntity {
            FirstEntity(Entity),
        }
    };

    let input = syn::parse2::<DeriveInput>(fragment.into()).unwrap();
    let Data::Enum(data) = &input.data else {
        panic!("Expected `enum` data item")
    };

    let variant = data.variants.first().unwrap();
    let variant = EnumVariant::try_from_unnamed_variant(variant).unwrap();
    assert_eq!(variant.name, "FirstEntity");
    let expected_field_name: Path = parse_quote! { Entity };
    assert_eq!(variant.field_name, Some(expected_field_name));
}

#[test]
fn parse_enumeration_variant_success_test() {
    let fragment = quote! {
        #[proto_map(
            source = "proto::HierarchyEntity",
            enumeration,
        )]
        enum Foo {
            Bar,
        }
    };

    let input = syn::parse2::<DeriveInput>(fragment.into()).unwrap();
    let Data::Enum(data) = &input.data else {
        panic!("Expected `enum` data item")
    };

    let variant = data.variants.first().unwrap();
    let variant = EnumVariant::try_from_enumeration_variant(variant).unwrap();
    assert_eq!(variant.name, "Bar");
    assert_eq!(variant.field_name, None);
}

#[test]
fn parse_u_variant_error_test() {
    let fragment = quote! {
        #[proto_map(
            source = "proto::HierarchyEntity",
            enumeration,
        )]
        enum Foo {
            First(Inner),
        }
    };

    let input = syn::parse2::<DeriveInput>(fragment.into()).unwrap();
    let Data::Enum(data) = &input.data else {
        panic!("Expected `enum` data item")
    };

    let variant = data.variants.first().unwrap();
    if let Ok(_) = EnumVariant::try_from_enumeration_variant(variant) {
        panic!("Expected unnamed variant error")
    }
}

#[test]
fn parse_unnamed_variant_errors_test() {
    let fragment = quote! {
        #[proto_map(
            source = "proto::HierarchyEntity",
            enumeration,
        )]
        enum Foo {
            Bar,
            Invalid(Inner1, Inner2)
        }
    };

    let input = syn::parse2::<DeriveInput>(fragment.into()).unwrap();
    let Data::Enum(data) = &input.data else {
        panic!("Expected `enum` data item")
    };

    let mut variants = data.variants.iter();
    let variant = variants.next().unwrap();
    if let Ok(_) = EnumVariant::try_from_unnamed_variant(variant) {
        panic!("Expected unnamed variant no inner error")
    }

    let variant = variants.next().unwrap();
    if let Ok(_) = EnumVariant::try_from_unnamed_variant(variant) {
        panic!("Expected unnamed variant more that one inner error")
    }
}
