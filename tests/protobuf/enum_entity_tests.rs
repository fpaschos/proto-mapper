use anyhow::Error;
use proto_mapper::{derive::ProtoMap, ProtoMap, ProtoMapScalar};
use crate::proto;


#[derive(Debug, Clone, ProtoMap, PartialEq)]
#[proto_map(source = "proto::protobuf::Entity")]
struct Entity {
    pub id: u32,
    pub nonce: i32,
    pub valid: bool,
    pub name: String,
    pub status: EntityStatus,
}

#[derive(Debug, ProtoMap, PartialEq)]
#[proto_map(source = "proto::protobuf::NestedEntity")]
struct NestedEntity {
    pub first: Entity,
    pub second: Entity,
}

#[derive(Debug, ProtoMap, PartialEq)]
#[proto_map(
    source = "proto::protobuf::HierarchyEntity",
    one_of(field = "data"),
    rename_variants = "snake_case"
)]
enum HierarchyEntity {
    FirstEntity(Entity),
    SecondEntity(NestedEntity),
}

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

#[test]
fn enumeration_round_trips() {
    let original = EntityStatus::StatusA;

    let p = original.to_proto();
    let tested = EntityStatus::from_proto(p).unwrap();
    assert_eq!(tested, original);
}

#[test]
fn hierarchy_entity_round_trips() {

    let entity = Entity {
        id: 1,
        nonce: 10,
        valid: true,
        name: "Foo".into(),
        status: EntityStatus::StatusC,
    };

    let original = HierarchyEntity::FirstEntity(entity);

    let p = original.to_proto();
    let tested = HierarchyEntity::from_proto(p).unwrap();
    assert_eq!(tested, original);

    let first = Entity {
        id: 2,
        nonce: 20,
        valid: false,
        name: "Entity2".into(),
        status: EntityStatus::StatusB,
    };

    let second = Entity {
        id: 2,
        nonce: 30,
        valid: true,
        name: "Entity3".into(),
        status: EntityStatus::StatusA,
    };

    let nested = NestedEntity { first, second };

    let original = HierarchyEntity::SecondEntity(nested);

    let p = original.to_proto();
    let tested = HierarchyEntity::from_proto(p).unwrap();

    assert_eq!(tested, original);
}

// TODO move to manual_implementation_tests
// Just for reference purposes implement the interface manually
#[derive(Debug, PartialEq)]
enum HierarchyEntityManual {
    FirstEntity(Entity),
    SecondEntity(NestedEntity),
}
impl ProtoMap for HierarchyEntityManual {
    type ProtoStruct = proto::protobuf::HierarchyEntity;
    fn to_proto(&self) -> proto::protobuf::HierarchyEntity {
        let mut inner = proto::protobuf::HierarchyEntity::default();
        match self {
            HierarchyEntityManual::FirstEntity(value) => inner.set_first_entity(value.to_proto()),
            HierarchyEntityManual::SecondEntity(value) => inner.set_second_entity(value.to_proto()),
        }
        inner
    }

    fn from_proto(proto: proto::protobuf::HierarchyEntity) -> Result<Self, Error> {
        match proto.data {
            Some(proto::protobuf::hierarchy_entity::Data::FirstEntity(v)) => {
                Entity::from_proto(v).map(HierarchyEntityManual::FirstEntity)
            }
            Some(proto::protobuf::hierarchy_entity::Data::SecondEntity(v)) => {
                NestedEntity::from_proto(v).map(HierarchyEntityManual::SecondEntity)
            }

            None => Err(anyhow::anyhow!(
                "Failed to convert HierarchyEntityManual from protobuf"
            )),
        }
    }
}

#[test]
fn manual_hierarchy_entity_round_trip() {
    let entity = Entity {
        id: 1,
        nonce: 10,
        valid: true,
        name: "Foo".into(),
        status: EntityStatus::StatusB,
    };

    let original = HierarchyEntityManual::FirstEntity(entity);

    let p = original.to_proto();
    let tested = HierarchyEntityManual::from_proto(p).unwrap();

    assert_eq!(tested, original);
}
