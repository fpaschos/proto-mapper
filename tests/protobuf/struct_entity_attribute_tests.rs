use crate::proto;
use proto_mapper::{derive::ProtoMap, ProtoMap, ProtoMapScalar};
use std::default::Default;

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
#[proto_map(source = "proto::protobuf::ScalarEntity")]
struct ScalarEntity {
    pub uint32_f: u32,
    pub int32_f: i32,
    #[proto_map(rename = "bool_f")]
    pub boolean_f: bool,
    #[proto_map(skip)]
    pub string_f: String,
    pub status: EntityStatus,
    #[proto_map(rename = "type_")]
    pub r#type: EntityType,
}

#[test]
fn entity_round_trip() {
    let mut original = ScalarEntity {
        uint32_f: 10,
        int32_f: -10,
        boolean_f: true,
        string_f: "Bar".to_string(),
        status: EntityStatus::StatusA,
        r#type: EntityType::TypeC,
    };

    let p = original.to_proto();
    let tested = ScalarEntity::from_proto(p).unwrap();

    original.string_f = Default::default(); // Name is skipped so default
    assert_eq!(tested, original);
}

#[test]
fn proto_entity_round_trip() {
    let mut original = proto::protobuf::ScalarEntity {
        uint32_f: 10,
        int32_f: -10,
        bool_f: true,
        ..Default::default()
    };

    let e = ScalarEntity::from_proto(original.clone()).unwrap();
    let tested = e.to_proto();

    original.string_f = Default::default(); // string field is skipped so default
    assert_eq!(tested, original);
}
