use quote::quote;
use syn::DeriveInput;

use crate::tests::{assert_tokens_eq, from_derive_input_enum};

#[test]
fn implement_enumeration_test() {
    let fragment = quote! {
        #[proto_map(
            source = "proto::EntityStatus",
            enumeration,
            rename_variants = "STREAMING_SNAKE_CASE"
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
        impl ProtoMap for EntityStatus {
            type ProtoStruct = proto::EntityStatus;

            fn to_proto(&self) -> Self::ProtoStruct {
                 match self {
                    Self::StatusA => proto::EntityStatus::STATUS_A,
                    Self::StatusB => proto::EntityStatus::STATUS_B,
                    Self::StatusC => proto::EntityStatus::STATUS_C,

                }
            }

            fn from_proto(proto: Self::ProtoStruct) -> std::result::Result<Self, anyhow::Error> {
                 match proto {
                        proto::EntityStatus::STATUS_A => Ok(Self::StatusA),
                        proto::EntityStatus::STATUS_B => Ok(Self::StatusB),
                        proto::EntityStatus::STATUS_C => Ok(Self::StatusC),
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
                let mut inner = Self::ProtoStruct::new();
                match self {
                    Self::FirstEntity(value) => inner.set_first_entity(value.to_proto()),
                    Self::SecondEntity(value) => inner.set_second_entity(value.to_proto()),
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
