use proto_mapper::derive::ProtoMap;
use proto_mapper::{ProtoMap, ProtoMapScalar};
use crate::proto;

#[derive(Debug, Clone, ProtoMap, Eq, PartialEq)]
#[proto_map(source = "proto::protobuf::Entity")]
struct Entity {
    pub id: u32,
    pub nonce: i32,
    pub valid: bool,
    pub name: String,
}

#[derive(Debug, ProtoMap, Eq, PartialEq)]
#[proto_map(source = "proto::protobuf::NestedEntity")]
struct NestedEntity {
    pub first: Entity,
    pub second: Entity,
}

#[test]
fn nested_entity_round_trip() {
    let entity = Entity {
        id: 1,
        nonce: 10,
        valid: true,
        name: "Foo".into(),
    };

    let original = NestedEntity {
        first: entity.clone(),
        second: entity,
    };

    let p = original.to_proto();
    let tested = NestedEntity::from_proto(p).unwrap();

    assert_eq!(tested, original);
}
