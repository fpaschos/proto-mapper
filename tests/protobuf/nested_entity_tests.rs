use crate::proto;
use proto_mapper::derive::ProtoMap;
use proto_mapper::{ProtoMap, ProtoMapScalar};

#[derive(Debug, Clone, ProtoMap, Eq, PartialEq)]
#[proto_map(source = "proto::protobuf::ScalarEntity")]
struct ScalarEntity {
    pub uint32_f: u32,
    pub int32_f: i32,
    pub bool_f: bool,
    pub string_f: String,
}

#[derive(Debug, ProtoMap, Eq, PartialEq)]
#[proto_map(source = "proto::protobuf::NestedEntity")]
struct NestedEntity {
    pub first: ScalarEntity,
    pub second: ScalarEntity,
}

#[test]
fn nested_entity_round_trip() {
    let entity = ScalarEntity {
        uint32_f: 1,
        int32_f: 10,
        bool_f: true,
        string_f: "Foo".into(),
    };

    let original = NestedEntity {
        first: entity.clone(),
        second: entity,
    };

    let p = original.to_proto();
    let tested = NestedEntity::from_proto(p).unwrap();

    assert_eq!(tested, original);
}
