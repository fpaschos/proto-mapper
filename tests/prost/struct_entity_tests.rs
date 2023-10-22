use proto_mapper::{derive::ProtoMap, ProtoMap, ProtoMapScalar};
use crate::proto;


#[derive(Debug, ProtoMap, Eq, PartialEq)]
#[proto_map(source = "proto::prost::Entity")]
struct Entity {
    pub id: u32,
    pub nonce: i32,
    pub valid: bool,
    pub name: String,
}

#[test]
fn entity_round_trip() {
    let original = Entity {
        id: 1,
        nonce: 10,
        valid: true,
        name: "Foo".into(),
    };

    let p = original.to_proto();
    let tested = Entity::from_proto(p).unwrap();

    assert_eq!(tested, original);
}

#[test]
fn proto_entity_round_trip() {
    let original = proto::prost::Entity {
        id: 1,
        nonce: 10,
        name: "Foo".to_string(),
        valid: true,
        ..Default::default()
    };

    let e = Entity::from_proto(original.clone()).unwrap();
    let tested = e.to_proto();

    assert_eq!(tested, original);
}
