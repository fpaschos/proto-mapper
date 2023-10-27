use crate::proto;
use proto_mapper::{ProtoMap, ProtoMapScalar, ProtoScalar};
use crate::proto::prost::hierarchy_entity::Data;

/// Fully expanded and manual experiments (these used to build the macros and the library traits synergy)
#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum EntityStatus {
    #[default]
    StatusA,
    StatusB,
    StatusC,
}

// Example of manual implementation for enumeration to primitive
impl ProtoMapScalar<i32> for EntityStatus {
    fn to_scalar(&self) -> i32 {
        match self {
            Self::StatusA => proto::prost::EntityStatus::StatusA.into(),
            Self::StatusB => proto::prost::EntityStatus::StatusB.into(),
            Self::StatusC => proto::prost::EntityStatus::StatusC.into(),
        }
    }

    fn from_scalar(proto: i32) -> Result<Self, anyhow::Error> {
        match proto {
            _ if proto == proto::prost::EntityStatus::StatusA as i32 => Ok(Self::StatusA),
            _ if proto == proto::prost::EntityStatus::StatusB as i32 => Ok(Self::StatusB),
            _ if proto == proto::prost::EntityStatus::StatusC as i32 => Ok(Self::StatusC),
            _ => Err(anyhow::anyhow!(format!(
                "Unable to match enumeration value {} to EntityStatus",
                proto
            ))),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
struct ScalarEntity {
    pub uint32_f: u32,
    pub int32_f: i32,
    pub bool_f: bool,
    pub string_f: String,
    pub bytes_f: Vec<u8>,
    pub status: EntityStatus,
}

// Example of manual implementation of entity with scalars and enumerations
impl ProtoMap for ScalarEntity {
    type ProtoStruct = proto::prost::ScalarEntity;
    fn to_proto(&self) -> Self::ProtoStruct {
        let mut proto = proto::prost::ScalarEntity::default();
        proto.uint32_f = ProtoMapScalar::to_scalar(&self.uint32_f);
        proto.int32_f = ProtoMapScalar::to_scalar(&self.int32_f);
        proto.bool_f = ProtoMapScalar::to_scalar(&self.bool_f);
        proto.string_f = ProtoMapScalar::to_scalar(&self.string_f);
        proto.bytes_f = ProtoMapScalar::to_scalar(&self.bytes_f);

        // Special case for enum
        proto.status = ProtoMapScalar::to_scalar(&self.status);
        proto
    }
    fn from_proto(proto: Self::ProtoStruct) -> Result<Self, anyhow::Error> {
        let inner = Self {
            uint32_f: ProtoMapScalar::from_scalar(proto.uint32_f)?,
            int32_f: ProtoMapScalar::from_scalar(proto.int32_f)?,
            bool_f: ProtoMapScalar::from_scalar(proto.bool_f)?,
            string_f: ProtoMapScalar::from_scalar(proto.string_f)?,
            bytes_f: ProtoMapScalar::from_scalar(proto.bytes_f)?,
            // Special case for enum
            status: ProtoMapScalar::from_scalar(proto.status)?,
        };
        Ok(inner)
    }
}

#[derive(Debug, PartialEq)]
struct ScalarEntityOptions {
    pub uint32_f: Option<u32>,
    pub int32_f: Option<i32>,
    pub bool_f: Option<bool>,
    pub string_f: Option<String>,
    pub bytes_f: Option<Vec<u8>>,
    pub status: Option<EntityStatus>,
}

impl ProtoMap for ScalarEntityOptions {
    type ProtoStruct = proto::prost::ScalarEntity;
    fn to_proto(&self) -> Self::ProtoStruct {
        let mut proto = proto::prost::ScalarEntity::default();

        // Only if there is value other default
        if let Some(value) = &self.uint32_f {
            proto.uint32_f = ProtoMapScalar::to_scalar(value);
        }

        // Only if there is value other default
        if let Some(value) = &self.int32_f {
            proto.int32_f = ProtoMapScalar::to_scalar(value);
        }

        if let Some(value) = &self.bool_f {
            proto.bool_f = ProtoMapScalar::to_scalar(value);
        }

        if let Some(value) = &self.string_f {
            proto.string_f = ProtoMapScalar::to_scalar(value);
        }

        if let Some(value) = &self.bytes_f {
            proto.bytes_f = ProtoMapScalar::to_scalar(value);
        }

        if let Some(value) = &self.status {
            proto.status = ProtoMapScalar::to_scalar(value);
        }

        proto
    }
    fn from_proto(proto: Self::ProtoStruct) -> Result<Self, anyhow::Error> {
        let inner = Self {
            // Special case for options
            uint32_f: {
                let value = proto.uint32_f;
                if ProtoScalar::has_value(&value) {
                    Some(ProtoMapScalar::from_scalar(value)?)
                } else {
                    None
                }
            },
            int32_f: {
                let value = proto.int32_f;
                if ProtoScalar::has_value(&value) {
                    Some(ProtoMapScalar::from_scalar(value)?)
                } else {
                    None
                }
            },
            bool_f: {
                let value = proto.bool_f;
                if ProtoScalar::has_value(&value) {
                    Some(ProtoMapScalar::from_scalar(value)?)
                } else {
                    None
                }
            },
            string_f: {
                let value = proto.string_f;
                if ProtoScalar::has_value(&value) {
                    Some(ProtoMapScalar::from_scalar(value)?)
                } else {
                    None
                }
            },
            bytes_f: {
                let value = proto.bytes_f;
                if ProtoScalar::has_value(&value) {
                    Some(ProtoMapScalar::from_scalar(value)?)
                } else {
                    None
                }
            },

            // Special case for enumerations
            status: {
                let value = proto.status;
                if ProtoScalar::has_value(&value) {
                    Some(ProtoMapScalar::from_scalar(value)?)
                } else {
                    None
                }
            },
        };
        Ok(inner)
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NestedEntity {
    first: ScalarEntity,
    second: Option<ScalarEntity>,
}

impl ProtoMap for NestedEntity {
    type ProtoStruct = proto::prost::NestedEntity;
    fn to_proto(&self) -> Self::ProtoStruct {
        let mut proto = proto::prost::NestedEntity::default();
        // Only if there is value other default
        proto.first = Some(ProtoMap::to_proto(&self.first));

        if let Some(value) = &self.second {
            proto.second = Some(ProtoMap::to_proto(value));
        }

        proto
    }
    fn from_proto(proto: Self::ProtoStruct) -> Result<Self, anyhow::Error> {
        let inner = Self {
            first: {
                if let Some(value) = proto.first {
                    ProtoMap::from_proto(value)?
                } else {
                    Default::default()
                }
            },
            second: {
                if let Some(value) = proto.second {
                    Some(ProtoMap::from_proto(value)?)
                } else {
                    None
                }
            },
        };
        Ok(inner)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum HierarchyEntity {
    FirstEntity(ScalarEntity),
    SecondEntity(NestedEntity),
}

impl ProtoMap for HierarchyEntity {
    type ProtoStruct = proto::prost::HierarchyEntity;

    fn to_proto(&self) -> Self::ProtoStruct {
        let mut inner = Self::ProtoStruct::default();
        match self {
            Self::FirstEntity(value) => inner.data = Some(proto::prost::hierarchy_entity::Data::FirstEntity(value.to_proto())),
            Self::SecondEntity(value) => inner.data = Some(proto::prost::hierarchy_entity::Data::SecondEntity(value.to_proto())),
        }
        inner
    }
    fn from_proto(proto: Self::ProtoStruct) -> std::result::Result<Self, anyhow::Error> {
        match proto.data {
            Some(proto::prost::hierarchy_entity::Data::FirstEntity(value)) => { ScalarEntity::from_proto(value)
                .map(Self::FirstEntity) }
            Some(proto::prost::hierarchy_entity::Data::SecondEntity(value)) => { NestedEntity::from_proto(value)
                .map(Self::SecondEntity) }
            _ => Err(anyhow::anyhow!(stringify!(Failed to decode enum HierarchyEntity from proto entity)))
        }
    }
}

#[test]
fn entity_test_round_trip() {
    let original = ScalarEntity {
        uint32_f: 1,
        int32_f: 10,
        bool_f: true,
        string_f: "Foo".into(),
        bytes_f: "Foo".as_bytes().to_vec(),
        status: EntityStatus::StatusC,
    };

    let p = original.to_proto();
    let tested = ScalarEntity::from_proto(p).unwrap();

    assert_eq!(tested, original);
}

#[test]
fn proto_entity_test_round_trip() {
    let original = proto::prost::ScalarEntity {
        uint32_f: 1,
        int32_f: -10,
        bool_f: true,
        string_f: "Foo".into(),
        bytes_f: "Foo".as_bytes().to_vec(),
        status: proto::prost::EntityStatus::StatusA as i32,
        ..Default::default()
    };

    let e = ScalarEntity::from_proto(original.clone()).unwrap();
    let tested = e.to_proto();

    assert_eq!(tested, original);
}

#[test]
fn test_entity_with_options_round_trips() {
    let original = ScalarEntityOptions {
        uint32_f: Some(1),
        int32_f: Some(-10),
        bool_f: Some(true),
        string_f: Some("Foo".into()),
        bytes_f: Some("Foo".as_bytes().to_vec()),
        status: Some(EntityStatus::StatusC),
    };

    let p = original.to_proto();
    let tested = ScalarEntityOptions::from_proto(p).unwrap();

    assert_eq!(tested, original);

    let original = ScalarEntityOptions {
        uint32_f: None,
        int32_f: None,
        bool_f: None,
        string_f: None,
        bytes_f: None,
        status: None,
    };

    let p = original.to_proto();
    let tested = ScalarEntityOptions::from_proto(p).unwrap();

    assert_eq!(tested, original);

    assert_eq!(tested, original);
}

#[test]
fn nested_entity_test_round_trips() {
    let entity = ScalarEntity {
        uint32_f: 1,
        int32_f: 10,
        bool_f: true,
        string_f: "Foo".into(),
        bytes_f: "Foo".as_bytes().to_vec(),
        status: EntityStatus::StatusC,
    };

    let original = NestedEntity {
        first: entity.clone(),
        second: None,
    };

    let p = original.to_proto();
    let tested = NestedEntity::from_proto(p).unwrap();

    assert_eq!(tested, original);

    let original = NestedEntity {
        first: entity.clone(),
        second: Some(entity.clone()),
    };

    let p = original.to_proto();
    let tested = NestedEntity::from_proto(p).unwrap();

    assert_eq!(tested, original);
}

#[test]
fn hierarchy_entity_round_trip() {
    let entity = ScalarEntity {
        uint32_f: 1,
        int32_f: 10,
        bool_f: true,
        string_f: "Foo".into(),
        bytes_f: "Foo".as_bytes().to_vec(),
        status: EntityStatus::StatusC,
    };

    let nested = NestedEntity {
        first: entity.clone(),
        second: Some(entity.clone()),
    };

    let original = HierarchyEntity::FirstEntity(entity);
    let p = original.to_proto();
    let tested = HierarchyEntity::from_proto(p).unwrap();

    assert_eq!(tested, original);

    let original = HierarchyEntity::SecondEntity(nested);
    let p = original.to_proto();
    let tested = HierarchyEntity::from_proto(p).unwrap();
    assert_eq!(tested, original);
}
