use proto_mapper::{derive::ProtoMap, ProtoMap, ProtoMapScalar};
use crate::proto;

// TODO support enumeration
// #[derive(Debug, Clone, Copy, PartialEq, ProtoMap)]
// #[proto_map(
// source = "proto::prost::EntityStatus",
// enumeration,
// rename_variants = "STREAMING_SNAKE_CASE"
// )]
// enum EntityStatus {
//     StatusA,
//     StatusB,
//     StatusC,
// }

#[derive(Debug, ProtoMap, PartialEq)]
#[proto_map(source = "proto::prost::ScalarEntity")]
struct ScalarEntity {
    pub uint32_f: u32,
    pub int32_f: i32,
    pub bool_f: bool,
    pub string_f: String,
    pub bytes_f: Vec<u8>,
    // pub status: EntityStatus,
}
#[test]
fn entity_round_trip() {
    let original = ScalarEntity {
        uint32_f: 1,
        int32_f: -10,
        bool_f: true,
        string_f: "Foo".into(),
        bytes_f: "Foo".as_bytes().to_vec(),
        // status: EntityStatus::StatusC,
    };

    let p = original.to_proto();
    let tested = ScalarEntity::from_proto(p).unwrap();

    assert_eq!(tested, original);
}

#[test]
fn proto_entity_round_trip() {
    let original = proto::prost::ScalarEntity {
        uint32_f: 1,
        int32_f: -10,
        bool_f: true,
        string_f: "Foo".into(),
        bytes_f: "Foo".as_bytes().to_vec(),
        // status: proto::prost::EntityStatus::STATUS_C.into(),
        ..Default::default()
    };

    let e = ScalarEntity::from_proto(original.clone()).unwrap();
    let tested = e.to_proto();

    assert_eq!(tested, original);
}
