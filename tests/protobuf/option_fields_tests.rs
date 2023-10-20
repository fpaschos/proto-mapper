use proto_mapper::{derive::ProtoMap, ProtoMap, ProtoMapScalar, ProtoScalar};
use crate::proto;

#[derive(Debug, ProtoMap, Eq, PartialEq, Default)]
#[proto_map(source = "proto::protobuf::Entity")]
struct Entity {
    pub id: Option<u32>,
    pub nonce: Option<i32>,
    pub valid: Option<bool>,
    pub name: Option<String>,
}

#[test]
fn default_entity_round_trip() {
    let original = Entity::default();

    let p = original.to_proto();
    let tested = Entity::from_proto(p).unwrap();

    assert_eq!(tested, original);
}

#[test]
fn default_proto_entity_round_trip() {
    let original = proto::protobuf::Entity::default();

    let e = Entity::from_proto(original.clone()).unwrap();
    let tested = e.to_proto();

    assert_eq!(tested, original);
}

#[test]
fn filled_entity_round_trip() {
    // LIMITATION NOTE that false , empty string and zeros and IN GENERAL default values  deserialize (from_proto) to None
    // That means that if you want to discriminate between absence of value and default you should not choose an option.
    let original = Entity {
        id: Some(100),
        nonce: Some(1000),
        valid: Some(true),
        name: Some("Foo".into()),
    };

    let p = original.to_proto();
    let tested = Entity::from_proto(p).unwrap();

    assert_eq!(tested, original);
}
#[test]
fn filled_proto_entity_round_trip() {
    // LIMITATION NOTE that false , empty string and zeros and IN GENERAL default values  deserialize (from_proto) to None
    // That means that if you want to discriminate between absence of value and default you should not choose an option.
    let original = proto::protobuf::Entity {
        id: 100,
        nonce: 100,
        valid: true,
        name: "BAR".to_string(),
        ..Default::default()
    };

    let e = Entity::from_proto(original.clone()).unwrap();
    let tested = e.to_proto();

    assert_eq!(tested, original);
}
