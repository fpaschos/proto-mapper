use crate::proto;
use proto_mapper::{derive::ProtoMap, ProtoMap, ProtoMapScalar, ProtoScalar};
use protobuf::Enum;

#[derive(Debug, Clone, Copy, PartialEq, ProtoMap)]
#[proto_map(
    source = "proto::protobuf::EntityStatus",
    enumeration,
    rename_variants = "STREAMING_SNAKE_CASE"
)]
enum EntityStatus {
    StatusA,
    StatusB,
    StatusC,
}

#[derive(Debug, ProtoMap, PartialEq, Default)]
#[proto_map(source = "proto::protobuf::ScalarEntity")]
struct ScalarEntityOptions {
    pub uint32_f: Option<u32>,
    pub int32_f: Option<i32>,
    pub bool_f: Option<bool>,
    pub string_f: Option<String>,
    pub bytes_f: Option<Vec<u8>>,
    #[proto_map(enumeration)]
    pub status: Option<EntityStatus>,
}

#[test]
fn default_entity_round_trip() {
    let original = ScalarEntityOptions::default();

    let p = original.to_proto();
    let tested = ScalarEntityOptions::from_proto(p).unwrap();

    assert_eq!(tested, original);
}

#[test]
fn default_proto_entity_round_trip() {
    let original = proto::protobuf::ScalarEntity::default();

    let e = ScalarEntityOptions::from_proto(original.clone()).unwrap();
    let tested = e.to_proto();

    assert_eq!(tested, original);
}

#[test]
fn filled_entity_round_trip() {
    // LIMITATION NOTE that false , empty string and zeros and IN GENERAL default values  deserialize (from_proto) to None
    // That means that if you want to discriminate between absence of value and default you should not choose an option.
    let original = ScalarEntityOptions {
        uint32_f: Some(100),
        int32_f: Some(-1000),
        bool_f: Some(true),
        string_f: Some("Foo".into()),
        bytes_f: Some("Foo".as_bytes().to_vec()),
        status: Some(EntityStatus::StatusB),
    };

    let p = original.to_proto();
    let tested = ScalarEntityOptions::from_proto(p).unwrap();

    assert_eq!(tested, original);
}
#[test]
fn filled_proto_entity_round_trip() {
    // LIMITATION NOTE that false , empty string and zeros and IN GENERAL default values  deserialize (from_proto) to None
    // That means that if you want to discriminate between absence of value and default you should not choose an option.
    let original = proto::protobuf::ScalarEntity {
        uint32_f: 0,
        int32_f: 0,
        bool_f: false,
        string_f: "Foo".into(),
        bytes_f: "Bytes".as_bytes().to_vec(),
        status: proto::protobuf::EntityStatus::STATUS_B.into(),
        ..Default::default()
    };

    let e = ScalarEntityOptions::from_proto(original.clone()).unwrap();
    let tested = e.to_proto();

    assert_eq!(tested, original);
}
