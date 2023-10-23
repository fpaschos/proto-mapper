use proto_mapper::{ProtoMap, ProtoMapScalar, ProtoScalar};
use crate::proto;

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
            _ => Err(anyhow::anyhow!(format!("Unable to match enumeration value {} to EntityStatus", proto)))
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
            uint32_f: ProtoMapScalar::from_scalar(proto.uint32_f.to_owned())?,
            int32_f: ProtoMapScalar::from_scalar(proto.int32_f.to_owned())?,
            bool_f: ProtoMapScalar::from_scalar(proto.bool_f.to_owned())?,
            string_f: ProtoMapScalar::from_scalar(proto.string_f.to_owned())?,
            bytes_f: ProtoMapScalar::from_scalar(proto.bytes_f.to_owned())?,
            // Special case for enum
            status: ProtoMapScalar::from_scalar(proto.status.to_owned())?,
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

//
// impl ProtoMap for EntityWithOptionals {
//     type ProtoStruct = proto::protobuf::EntityWithOptionals;
//     fn to_proto(&self) -> Self::ProtoStruct {
//         let mut proto = proto::protobuf::EntityWithOptionals::default();
//         proto.set_id(ProtoMapScalar::to_scalar(&self.id));
//         proto.set_nonce(ProtoMapScalar::to_scalar(&self.nonce));
//         proto.set_valid(ProtoMapScalar::to_scalar(&self.valid));
//         proto.set_name(ProtoMapScalar::to_scalar(&self.name));
//
//         // Only if there is value other default
//         if let Some(value) = &self.opt_id {
//             proto.set_opt_id(ProtoMapScalar::to_scalar(value));
//         }
//
//         // Only if there is value other default
//         if let Some(value) = &self.opt_nonce {
//             proto.set_opt_nonce(ProtoMapScalar::to_scalar(value));
//         }
//
//         if let Some(value) = &self.opt_valid {
//             proto.set_opt_valid(ProtoMapScalar::to_scalar(value));
//         }
//
//         if let Some(value) = &self.opt_name {
//             proto.set_opt_name(ProtoMapScalar::to_scalar(value));
//         }
//
//         if let Some(value) = &self.opt_status {
//             proto.set_opt_status(ProtoMap::to_proto(value));
//         }
//         proto
//     }
//     fn from_proto(proto: Self::ProtoStruct) -> Result<Self, anyhow::Error> {
//         let inner = Self {
//             id: ProtoMapScalar::from_scalar(proto.id().to_owned())?,
//             nonce: ProtoMapScalar::from_scalar(proto.nonce().to_owned())?,
//             valid: ProtoMapScalar::from_scalar(proto.valid().to_owned())?,
//             name: ProtoMapScalar::from_scalar(proto.name().to_owned())?,
//             // Special case for options
//             opt_id: {
//                 let v = proto.opt_id().to_owned();
//                 if ProtoScalar::has_value(&v) {
//                     Some(ProtoMapScalar::from_scalar(v)?)
//                 } else {
//                     None
//                 }
//             },
//             opt_nonce: {
//                 let v = proto.opt_nonce().to_owned();
//                 if ProtoScalar::has_value(&v) {
//                     Some(ProtoMapScalar::from_scalar(v)?)
//                 } else {
//                     None
//                 }
//             },
//             opt_valid: {
//                 let v = proto.opt_valid().to_owned();
//                 if ProtoScalar::has_value(&v) {
//                     Some(ProtoMapScalar::from_scalar(v)?)
//                 } else {
//                     None
//                 }
//             },
//             opt_name: {
//                 let v = proto.opt_name().to_owned();
//                 if ProtoScalar::has_value(&v) {
//                     Some(ProtoMapScalar::from_scalar(v)?)
//                 } else {
//                     None
//                 }
//             },
//             // Special case for enumerations
//             opt_status: {
//                 let v = proto.opt_status().to_owned();
//                 // convert enum value to i32 in order to check ProtoPrimitive value
//                 if ProtoScalar::has_value(&v.value()) {
//                     Some(ProtoMap::from_proto(v)?)
//                 } else {
//                     None
//                 }
//             },
//         };
//         Ok(inner)
//     }
// }
#[derive(Debug, Clone, PartialEq)]
pub struct NestedEntity {
    first: ScalarEntity,
    second: Option<ScalarEntity>,
}

impl ProtoMap for NestedEntity {
    type ProtoStruct = proto::prost::NestedEntity;
    fn to_proto(&self) -> Self::ProtoStruct {
        let mut proto = proto::prost::NestedEntity::default();
        proto.first = Some(ProtoMap::to_proto(&self.first).into());
        proto.second = self.second.as_ref().map(|value| ProtoMap::to_proto(value));

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
                if let Some(value) =  proto.second {
                    Some(ProtoMap::from_proto(value)?)
                } else {
                    None
                }
            },
        };
        Ok(inner)
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
//
// #[test]
// fn test_entity_with_optionals_round_trips() {
//     let original = EntityWithOptionals {
//         id: 1,
//         nonce: 10,
//         valid: true,
//         name: "Foo".into(),
//         opt_id: None,
//         opt_nonce: None,
//         opt_valid: None,
//         opt_name: None,
//         opt_status: None,
//     };
//
//     let p = original.to_proto();
//     let tested = EntityWithOptionals::from_proto(p).unwrap();
//
//     assert_eq!(tested, original);
//
//     let original = EntityWithOptionals {
//         id: 1,
//         nonce: 10,
//         valid: true,
//         name: "Foo".into(),
//         opt_id: Some(1),
//         opt_nonce: Some(2),
//         opt_valid: Some(true),
//         opt_name: Some("Foo1".into()),
//         opt_status: Some(EntityStatus::StatusC),
//     };
//
//     let p = original.to_proto();
//     let tested = EntityWithOptionals::from_proto(p).unwrap();
//
//     assert_eq!(tested, original);
// }
//
//
//
// #[test]
// fn nested_entity_test_round_trips() {
//     let entity = Entity {
//         id: 1,
//         nonce: 10,
//         valid: true,
//         name: "Foo".into(),
//         status: EntityStatus::StatusB,
//     };
//
//     let original = NestedEntity {
//         first: entity.clone(),
//         second: None,
//     };
//
//     let p = original.to_proto();
//     let tested = NestedEntity::from_proto(p).unwrap();
//
//     assert_eq!(tested, original);
//
//     let original = NestedEntity {
//         first: entity.clone(),
//         second: Some(entity.clone()),
//     };
//
//     let p = original.to_proto();
//     let tested = NestedEntity::from_proto(p).unwrap();
//
//     assert_eq!(tested, original);
// }
