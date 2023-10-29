use crate::structs::StructAttrs;
use darling::FromMeta;
use quote::quote;
use syn::{Data, DeriveInput};

use crate::structs::StructField;
use crate::types::{ScalarType, Ty};

#[test]
fn parse_struct_attributes_test() {
    let fragment = quote! {
        #[proto_map(source = "proto::Entity", rename_all = "snake_case")]
        struct Test;
    };

    let input = syn::parse2::<DeriveInput>(fragment.into()).unwrap();

    let meta = &input.attrs.first().unwrap().meta;

    let attrs = StructAttrs::from_meta(meta).unwrap();
    let path = attrs.source;
    assert_eq!(quote! { #path }.to_string(), "proto :: Entity");
    assert_eq!(attrs.rename_all, Some("snake_case".into()));
}

#[test]
fn parse_struct_primitive_fields_test() {
    let fragment = quote! {
        struct Test {
            a: u32,
            b: i32,
            c: bool,
            d: f32,
            e: f64,
            f: u64,
            g: i64,
            h: String,
        }
    };

    let input = syn::parse2::<DeriveInput>(fragment.into()).unwrap();

    let Data::Struct(data) = &input.data else {
        panic!("Expected Data::Struct here");
    };

    let mut fields = data.fields.iter();
    let field = fields.next().unwrap();

    let field = StructField::try_from_field(field).unwrap();

    assert_eq!(field.name, "a".to_string());
    assert_eq!(field.ty, Ty::scalar(ScalarType::U32, false));
    assert!(field.attrs.is_none());

    let field = fields.next().unwrap();

    let field = StructField::try_from_field(field).unwrap();

    assert_eq!(field.name, "b".to_string());
    assert_eq!(field.ty, Ty::scalar(ScalarType::I32, false));
    assert!(!field.is_optional());
    assert!(field.attrs.is_none());

    let field = fields.next().unwrap();

    let field = StructField::try_from_field(field).unwrap();

    assert_eq!(field.name, "c".to_string());
    assert_eq!(field.ty, Ty::scalar(ScalarType::Bool, false));
    assert!(!field.is_optional());
    assert!(field.attrs.is_none());

    let field = fields.next().unwrap();

    let field = StructField::try_from_field(field).unwrap();

    assert_eq!(field.name, "d".to_string());
    assert_eq!(field.ty, Ty::scalar(ScalarType::F32, false));
    assert!(field.attrs.is_none());

    let field = fields.next().unwrap();

    let field = StructField::try_from_field(field).unwrap();

    assert_eq!(field.name, "e".to_string());
    assert_eq!(field.ty, Ty::scalar(ScalarType::F64, false));
    assert!(field.attrs.is_none());

    let field = fields.next().unwrap();

    let field = StructField::try_from_field(field).unwrap();

    assert_eq!(field.name, "f".to_string());
    assert_eq!(field.ty, Ty::scalar(ScalarType::U64, false));
    assert!(!field.is_optional());
    assert!(field.attrs.is_none());

    let field = fields.next().unwrap();

    let field = StructField::try_from_field(field).unwrap();

    assert_eq!(field.name, "g".to_string());
    assert_eq!(field.ty, Ty::scalar(ScalarType::I64, false));
    assert!(!field.is_optional());
    assert!(field.attrs.is_none());

    let field = fields.next().unwrap();

    let field = StructField::try_from_field(field).unwrap();

    assert_eq!(field.name, "h".to_string());
    assert_eq!(field.ty, Ty::scalar(ScalarType::String, false));
    assert!(!field.is_optional());
    assert!(field.attrs.is_none());
}

#[test]
fn parse_nested_types_test() {
    let fragment = quote! {
        #[derive(Debug, PartialEq)]
        #[proto_convert(source = "proto::Entity", rename_all = "snake_case")]
        struct Entity {
            pub opt_id: std::option::Option<u32>,
            pub whatever: Whatever,
            pub opt_whatever: Option<Whatever>,
            pub hash_map: HashMap<u32, String>,
        }
    };

    let input = syn::parse2::<DeriveInput>(fragment.into()).unwrap();

    let Data::Struct(data) = &input.data else {
        panic!("Expected Data::Struct here");
    };

    let mut fields = data.fields.iter();

    let field = fields.next().unwrap();

    let field = StructField::try_from_field(field).unwrap();
    assert_eq!(field.name, "opt_id".to_string());
    assert_eq!(field.ty, Ty::scalar(ScalarType::U32, true));
    assert!(field.attrs.is_none());

    let field = fields.next().unwrap();

    let field = StructField::try_from_field(field).unwrap();
    assert_eq!(field.name, "whatever".to_string());
    assert!(matches!(
        field.ty,
        Ty::Other {
            optional: false,
            ..
        }
    ));
    assert!(field.attrs.is_none());

    let field = fields.next().unwrap();

    let field = StructField::try_from_field(field).unwrap();
    assert_eq!(field.name, "opt_whatever".to_string());
    assert!(matches!(field.ty, Ty::Other { optional: true, .. }));
    assert!(field.attrs.is_none());

    let field = fields.next().unwrap();

    let field = StructField::try_from_field(field).unwrap();
    assert_eq!(field.name, "hash_map".to_string());
    assert!(matches!(
        field.ty,
        Ty::Other {
            optional: false,
            ..
        }
    ));

    assert!(field.attrs.is_none());
}

#[test]
fn unsupported_tuple_primitive_field_test() {
    let fragment = quote! {
        struct Test {
            field: (u32,u32),
        }
    };

    let input = syn::parse2::<DeriveInput>(fragment.into()).unwrap();

    let Data::Struct(data) = &input.data else {
        panic!("Expected Data::Struct here");
    };
    let field = data.fields.iter().next().unwrap();

    let res = StructField::try_from_field(field);
    assert!(res.is_err())
}

#[test]
fn parse_struct_field_attributes_test() {
    let fragment = quote! {
        struct Test {
            a: u32,
            #[proto_map(rename="awesome_b")]
            b: i32,
            #[proto_map(skip)]
            c: bool,
            #[proto_map(enumeration)]
            d: Option<Whatever>
        }
    };

    let input = syn::parse2::<DeriveInput>(fragment.into()).unwrap();

    let Data::Struct(data) = &input.data else {
        panic!("Expected Data::Struct here");
    };

    let mut fields = data.fields.iter();

    let field = fields.next().unwrap();
    let field = StructField::try_from_field(field).unwrap();

    assert!(field.attrs.is_none());

    let field = fields.next().unwrap();
    let field = StructField::try_from_field(field).unwrap();

    assert_eq!(field.name, "b".to_string());
    assert!(field.attrs.is_some());
    let attrs = field.attrs.unwrap();
    assert_eq!(attrs.rename, Some("awesome_b".to_string()));

    let field = fields.next().unwrap();
    let field = StructField::try_from_field(field).unwrap();

    assert_eq!(field.name, "c".to_string());
    assert!(field.attrs.is_some());
    let attrs = field.attrs.unwrap();
    assert!(attrs.skip);

    let field = fields.next().unwrap();
    let field = StructField::try_from_field(field).unwrap();

    assert_eq!(field.name, "d".to_string());
    assert!(field.attrs.is_some());
    let attrs = field.attrs.unwrap();
    assert!(attrs.enumeration);
}
