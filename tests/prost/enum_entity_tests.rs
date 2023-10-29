use crate::proto;
use proto_mapper::{derive::ProtoMap, ProtoMap, ProtoMapScalar};

#[derive(Debug, Clone, Default, ProtoMap, PartialEq)]
#[proto_map(source = "proto::prost::ScalarEntity")]
struct ScalarEntity {
    pub uint32_f: u32,
    pub int32_f: i32,
    pub bool_f: bool,
    pub string_f: String,
    pub bytes_f: Vec<u8>,
    #[proto_map(enumeration)]
    pub status: EntityStatus,
}

#[derive(Debug, ProtoMap, PartialEq)]
#[proto_map(source = "proto::prost::NestedEntity")]
struct NestedEntity {
    pub first: ScalarEntity,
    pub second: ScalarEntity,
}

#[derive(Debug, ProtoMap, PartialEq)]
#[proto_map(
    source = "proto::prost::HierarchyEntity",
    one_of(field = "data"),
    rename_variants = "snake_case"
)]
enum HierarchyEntity {
    FirstEntity(ScalarEntity),
    SecondEntity(NestedEntity),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, ProtoMap)]
#[proto_map(
    source = "proto::prost::EntityStatus",
    enumeration,
)]
enum EntityStatus {
    #[default]
    StatusA,
    StatusB,
    StatusC,
}

#[test]
fn enumeration_round_trips() {
    let original = EntityStatus::StatusA;

    let p = original.to_scalar();
    let tested = EntityStatus::from_scalar(p).unwrap();
    assert_eq!(tested, original);
}

#[test]
fn hierarchy_entity_round_trips() {
    let entity = ScalarEntity {
        uint32_f: 1,
        int32_f: 10,
        bool_f: true,
        string_f: "Foo".into(),
        bytes_f: "Foo".as_bytes().to_vec(),
        status: EntityStatus::StatusC,
    };

    let original = HierarchyEntity::FirstEntity(entity);

    let p = original.to_proto();
    let tested = HierarchyEntity::from_proto(p).unwrap();
    assert_eq!(tested, original);

    let first = ScalarEntity {
        uint32_f: 1,
        int32_f: -10,
        bool_f: true,
        string_f: "Foo1".into(),
        bytes_f: "Foo1".as_bytes().to_vec(),
        status: EntityStatus::StatusC,
    };

    let second = ScalarEntity {
        uint32_f: 2,
        int32_f: -20,
        bool_f: false,
        string_f: "Foo2".into(),
        bytes_f: "Foo2".as_bytes().to_vec(),
        status: EntityStatus::StatusA,
    };

    let nested = NestedEntity { first, second };

    let original = HierarchyEntity::SecondEntity(nested);

    let p: proto::prost::HierarchyEntity = original.to_proto();
    let tested = HierarchyEntity::from_proto(p).unwrap();

    assert_eq!(tested, original);
}