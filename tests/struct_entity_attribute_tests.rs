use proto_mapper::{
    derive::ProtoMap,
    ProtoMap,
    ProtoMapScalar
};
use std::default::Default;
mod proto;

#[derive(Debug, Clone, Copy, Eq, PartialEq, ProtoMap)]
#[proto_map(
    source = "proto::protobuf::EntityStatus",
    enumeration,
    rename_variants = "STREAMING_SNAKE_CASE"
)]
pub enum EntityStatus {
    StatusA,
    StatusB,
    StatusC,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, ProtoMap)]
#[proto_map(
    source = "proto::protobuf::EntityType",
    enumeration,
    rename_variants = "STREAMING_SNAKE_CASE"
)]
pub enum EntityType {
    TypeA,
    TypeB,
    TypeC,
}

#[derive(Debug, ProtoMap, Eq, PartialEq)]
#[proto_map(source = "proto::protobuf::Entity")]
struct Entity {
    pub id: u32,
    pub nonce: i32,
    pub valid: bool,
    #[proto_map(skip)]
    pub name: String,
    pub status: EntityStatus,
    #[proto_map(rename = "type_")]
    pub r#type: EntityType,
}

#[test]
fn entity_round_trip() {
    let mut original = Entity {
        id: 1,
        nonce: 10,
        valid: true,
        name: "Bar".to_string(),
        status: EntityStatus::StatusA,
        r#type: EntityType::TypeC,
    };

    let p = original.to_proto();
    let tested = Entity::from_proto(p).unwrap();

    original.name = Default::default(); // Name is skipped so default
    assert_eq!(tested, original);
}

#[test]
fn proto_entity_round_trip() {
    let mut original = proto::protobuf::Entity {
        id: 1,
        nonce: 10,
        name: "Foo".to_string(),
        valid: true,
        ..Default::default()
    };

    let e = Entity::from_proto(original.clone()).unwrap();
    let tested = e.to_proto();

    original.name = Default::default(); // Name is skipped so default
    assert_eq!(tested, original);
}
