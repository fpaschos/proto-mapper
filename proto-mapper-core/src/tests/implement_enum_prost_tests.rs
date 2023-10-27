use quote::quote;
use syn::DeriveInput;

use crate::tests::{assert_tokens_eq, from_derive_input_enum};

#[test]
fn implement_enumeration_test() {
    let fragment = quote! {
        #[proto_map(
            source = "proto::prost::EntityStatus",
            enumeration,
        )]
        enum EntityStatus {
            StatusA,
            StatusB,
            StatusC,
        }
    };

    let input = syn::parse2::<DeriveInput>(fragment.into()).unwrap();
    let e = from_derive_input_enum(&input).unwrap();

    let expected = quote! {
        impl ProtoMapScalar<i32> for EntityStatus {

            fn to_scalar(&self) -> i32 {
                 match self {
                    Self::StatusA => proto::prost::EntityStatus::StatusA.into(),
                    Self::StatusB => proto::prost::EntityStatus::StatusB.into(),
                    Self::StatusC => proto::prost::EntityStatus::StatusC.into(),
                }
            }

            fn from_scalar(proto: i32) -> std::result::Result<Self, anyhow::Error> {
                 match proto {
                    _ if proto == proto::prost::EntityStatus::StatusA as i32 => Ok(Self::StatusA),
                    _ if proto == proto::prost::EntityStatus::StatusB as i32 => Ok(Self::StatusB),
                    _ if proto == proto::prost::EntityStatus::StatusC as i32 => Ok(Self::StatusC),
                    _ => Err(anyhow::anyhow!(format!(stringify!(Failed to match enum value {} to proto entity EntityStatus) ,proto)))
                }
            }
        }
    };

    let actual = e.implement_proto_map();
    assert_tokens_eq(&expected, &actual)
}

#[test]
fn implement_non_enumeration_test() {
    let fragment = quote! {
        #[proto_map(
            source = "proto::HierarchyEntity",
            one_of(field = "data"),
            rename_variants = "snake_case"
        )]
        enum HierarchyEntity {
            FirstEntity(Entity),
            SecondEntity(NestedEntity),
        }
    };

    let input = syn::parse2::<DeriveInput>(fragment.into()).unwrap();
    let e = from_derive_input_enum(&input).unwrap();

    let expected = quote! {
        impl ProtoMap for HierarchyEntity {
            type ProtoStruct = proto::HierarchyEntity;

            fn to_proto(&self) -> Self::ProtoStruct {
                let mut inner = Self::ProtoStruct::default();
                match self {
                    Self::FirstEntity(value) => inner.data = Some(proto::hierarchy_entity::Data::FirstEntity(value.to_proto())),
                    Self::SecondEntity(value) => inner.data = Some(proto::hierarchy_entity::Data::SecondEntity(value.to_proto())),
                }
                inner
            }

            fn from_proto(proto: Self::ProtoStruct) -> std::result::Result<Self, anyhow::Error> {
                match proto.data {
                   Some(proto::hierarchy_entity::Data::FirstEntity(value)) => { Entity::from_proto(value)
                    .map(Self::FirstEntity) }
                    Some(proto::hierarchy_entity::Data::SecondEntity(value)) => { NestedEntity::from_proto(value)
                    .map(Self::SecondEntity) }
                    _ => Err(anyhow::anyhow!(stringify!(Failed to decode enum HierarchyEntity from proto entity)))
                }
            }
        }
    };

    let actual = e.implement_proto_map();
    assert_tokens_eq(&expected, &actual)
}
