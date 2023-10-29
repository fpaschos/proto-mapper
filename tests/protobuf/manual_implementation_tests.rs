use crate::proto;
use proto_mapper::{ProtoMap, ProtoMapScalar, ProtoScalar};
use protobuf::Enum;

/// Fully expanded and manual experiments (these used to build the macros and the library traits synergy)

#[derive(Debug, Clone, Copy, PartialEq)]
enum EntityStatus {
    StatusA,
    StatusB,
    StatusC,
}

// Example of manual implementation for enumeration to primitive
impl ProtoMap for EntityStatus {
    type ProtoStruct = proto::protobuf::EntityStatus;
    fn to_proto(&self) -> Self::ProtoStruct {
        match self {
            Self::StatusA => proto::protobuf::EntityStatus::STATUS_A,
            Self::StatusB => proto::protobuf::EntityStatus::STATUS_B,
            Self::StatusC => proto::protobuf::EntityStatus::STATUS_C,
        }
    }

    fn from_proto(proto: Self::ProtoStruct) -> Result<Self, anyhow::Error> {
        match proto {
            proto::protobuf::EntityStatus::STATUS_A => Ok(Self::StatusA),
            proto::protobuf::EntityStatus::STATUS_B => Ok(Self::StatusB),
            proto::protobuf::EntityStatus::STATUS_C => Ok(Self::StatusC),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct ScalarEntity {
    pub uint32_f: u32,
    pub int32_f: i32,
    pub bool_f: bool,
    pub string_f: String,
    pub bytes_f: Vec<u8>,
    pub status: EntityStatus,
}

impl ProtoMap for ScalarEntity {
    type ProtoStruct = proto::protobuf::ScalarEntity;
    fn to_proto(&self) -> Self::ProtoStruct {
        let mut proto = proto::protobuf::ScalarEntity::default();
        proto.set_uint32_f(ProtoMapScalar::to_scalar(&self.uint32_f));
        proto.set_int32_f(ProtoMapScalar::to_scalar(&self.int32_f));
        proto.set_bool_f(ProtoMapScalar::to_scalar(&self.bool_f));
        proto.set_string_f(ProtoMapScalar::to_scalar(&self.string_f));
        proto.set_bytes_f(ProtoMapScalar::to_scalar(&self.bytes_f));
        // Special case for enum
        proto.set_status(ProtoMap::to_proto(&self.status));
        proto
    }
    fn from_proto(proto: Self::ProtoStruct) -> Result<Self, anyhow::Error> {
        let inner = Self {
            uint32_f: ProtoMapScalar::from_scalar(proto.uint32_f().to_owned())?,
            int32_f: ProtoMapScalar::from_scalar(proto.int32_f().to_owned())?,
            bool_f: ProtoMapScalar::from_scalar(proto.bool_f().to_owned())?,
            string_f: ProtoMapScalar::from_scalar(proto.string_f().to_owned())?,
            bytes_f: ProtoMapScalar::from_scalar(proto.bytes_f().to_owned())?,
            // Special case for enum
            status: ProtoMap::from_proto(proto.status().to_owned())?,
        };
        Ok(inner)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct ScalarEntityOptions {
    pub uint32_f: Option<u32>,
    pub int32_f: Option<i32>,
    pub bool_f: Option<bool>,
    pub string_f: Option<String>,
    pub bytes_f: Option<Vec<u8>>,
    pub status: Option<EntityStatus>,
}

impl ProtoMap for ScalarEntityOptions {
    type ProtoStruct = proto::protobuf::ScalarEntity;
    fn to_proto(&self) -> Self::ProtoStruct {
        let mut proto = proto::protobuf::ScalarEntity::default();

        // Only if there is value other default
        if let Some(value) = &self.uint32_f {
            proto.set_uint32_f(ProtoMapScalar::to_scalar(value));
        }

        // Only if there is value other default
        if let Some(value) = &self.int32_f {
            proto.set_int32_f(ProtoMapScalar::to_scalar(value));
        }

        if let Some(value) = &self.bool_f {
            proto.set_bool_f(ProtoMapScalar::to_scalar(value));
        }

        if let Some(value) = &self.string_f {
            proto.set_string_f(ProtoMapScalar::to_scalar(value));
        }

        if let Some(value) = &self.bytes_f {
            proto.set_bytes_f(ProtoMapScalar::to_scalar(value));
        }

        if let Some(value) = &self.status {
            proto.set_status(ProtoMap::to_proto(value));
        }

        proto
    }
    fn from_proto(proto: Self::ProtoStruct) -> Result<Self, anyhow::Error> {
        let inner = Self {
            // Special case for options
            uint32_f: {
                let v = proto.uint32_f().to_owned();
                if ProtoScalar::has_value(&v) {
                    Some(ProtoMapScalar::from_scalar(v)?)
                } else {
                    None
                }
            },
            int32_f: {
                let v = proto.int32_f().to_owned();
                if ProtoScalar::has_value(&v) {
                    Some(ProtoMapScalar::from_scalar(v)?)
                } else {
                    None
                }
            },
            bool_f: {
                let v = proto.bool_f().to_owned();
                if ProtoScalar::has_value(&v) {
                    Some(ProtoMapScalar::from_scalar(v)?)
                } else {
                    None
                }
            },
            string_f: {
                let v = proto.string_f().to_owned();
                if ProtoScalar::has_value(&v) {
                    Some(ProtoMapScalar::from_scalar(v)?)
                } else {
                    None
                }
            },
            bytes_f: {
                let v = proto.bytes_f().to_owned();
                if ProtoScalar::has_value(&v) {
                    Some(ProtoMapScalar::from_scalar(v)?)
                } else {
                    None
                }
            },
            // Special case for enumerations
            status: {
                let v = proto.status().to_owned();
                // convert enum value to i32 in order to check ProtoPrimitive value
                if ProtoScalar::has_value(&v.value()) {
                    Some(ProtoMap::from_proto(v)?)
                } else {
                    None
                }
            },
        };
        Ok(inner)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct NestedEntity {
    first: ScalarEntity,
    second: Option<ScalarEntity>,
}

impl ProtoMap for NestedEntity {
    type ProtoStruct = proto::protobuf::NestedEntity;
    fn to_proto(&self) -> Self::ProtoStruct {
        let mut proto = proto::protobuf::NestedEntity::default();
        proto.set_first(ProtoMap::to_proto(&self.first).into());
        // Only if there is value other default
        if let Some(value) = &self.second {
            proto.set_second(ProtoMap::to_proto(value).into());
        }
        proto
    }
    fn from_proto(proto: Self::ProtoStruct) -> Result<Self, anyhow::Error> {
        let inner = Self {
            first: ProtoMap::from_proto(proto.first().to_owned())?,
            second: {
                if proto.has_second() {
                    Some(ProtoMap::from_proto(proto.second().to_owned())?)
                } else {
                    None
                }
            },
        };
        Ok(inner)
    }
}

// Just for reference purposes implement the interface manually
#[derive(Debug, PartialEq)]
enum HierarchyEntityManual {
    FirstEntity(ScalarEntity),
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

    fn from_proto(proto: proto::protobuf::HierarchyEntity) -> Result<Self, anyhow::Error> {
        match proto.data {
            Some(proto::protobuf::hierarchy_entity::Data::FirstEntity(v)) => {
                ScalarEntity::from_proto(v).map(HierarchyEntityManual::FirstEntity)
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
    let entity = ScalarEntity {
        uint32_f: 1,
        int32_f: 10,
        bool_f: true,
        string_f: "Foo".into(),
        bytes_f: "Foo".as_bytes().to_vec(),
        status: EntityStatus::StatusC,
    };

    let original = HierarchyEntityManual::FirstEntity(entity);

    let p = original.to_proto();
    let tested = HierarchyEntityManual::from_proto(p).unwrap();

    assert_eq!(tested, original);
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
