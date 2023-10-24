use crate::tests::{assert_tokens_eq, from_derive_input_struct};
use quote::quote;
use syn::DeriveInput;

#[test]
fn implement_struct_scalar_types_test() {
    let fragment = quote! {
        #[proto_map(source = "proto::Test")]
        struct Test {
            id: u32,
            valid: bool,
            bytes: Vec<u8>,
            opt_name: Option<String>,
            opt_bytes: Option<Vec<u8>>,
        }
    };

    let input = syn::parse2::<DeriveInput>(fragment.into()).unwrap();

    let s = from_derive_input_struct(&input).unwrap();

    let expected = quote! {
        impl ProtoMap for Test {
            type ProtoStruct = proto::Test;
            fn to_proto(&self) -> Self::ProtoStruct {
                let mut proto = proto::Test::default();

                proto.id = ProtoMapScalar::to_scalar(&self.id);
                proto.valid = ProtoMapScalar::to_scalar(&self.valid);
                proto.bytes = ProtoMapScalar::to_scalar(&self.bytes);

                if let Some(value) = &self.opt_name {
                    proto.opt_name = ProtoMapScalar::to_scalar(value);
                }

                if let Some(value) = &self.opt_bytes {
                    proto.opt_bytes = ProtoMapScalar::to_scalar(value);
                }

                proto
            }

            fn from_proto(proto: Self::ProtoStruct) -> std::result::Result<Self, anyhow::Error> {
                let inner = Self {
                    id: ProtoMapScalar::from_scalar(proto.id)?,
                    valid: ProtoMapScalar::from_scalar(proto.valid)?,
                    bytes: ProtoMapScalar::from_scalar(proto.bytes)?,
                    opt_name: {
                        let value = proto.opt_name;
                        if ProtoScalar::has_value(&value) {
                            Some(ProtoMapScalar::from_scalar(value)?)
                        } else {
                            None
                        }
                    },
                    opt_bytes: {
                        let value = proto.opt_bytes;
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
    };

    let actual = s.implement_proto_map();
    assert_tokens_eq(&expected, &actual);
}

#[test]
fn implement_struct_non_scalar_types_test() {
    let fragment = quote! {
        #[proto_map(source = "proto::Test")]
        struct Test {
            first: Entity,
            second: Option<Entity>,
        }
    };

    let input = syn::parse2::<DeriveInput>(fragment.into()).unwrap();

    let s = from_derive_input_struct(&input).unwrap();

    let expected = quote! {
        impl ProtoMap for Test {
            type ProtoStruct = proto::Test;
            fn to_proto(&self) -> Self::ProtoStruct {
                let mut proto = proto::Test::default();

                proto.first = Some(ProtoMap::to_proto(&self.first));

                if let Some(value) = &self.second {
                    proto.second = Some(ProtoMap::to_proto(value));
                }

                proto
            }

            fn from_proto(proto: Self::ProtoStruct) -> std::result::Result<Self, anyhow::Error> {
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
    };

    let actual = s.implement_proto_map();
    assert_tokens_eq(&expected, &actual);
}
//
// #[test]
// fn implement_struct_rename_attributes_test() {
//     let fragment = quote! {
//         #[proto_map(source = "proto::Test")]
//         struct Test {
//             #[proto_map(rename = "type_")]
//             r#type: Entity,
//             #[proto_map(rename = "other_name")]
//             opt_entity: Option<Entity>,
//         }
//     };
//
//     let input = syn::parse2::<DeriveInput>(fragment.into()).unwrap();
//
//     let s = from_derive_input_struct(&input).unwrap();
//
//     let expected = quote! {
//         impl ProtoMap for Test {
//             type ProtoStruct = proto::Test;
//             fn to_proto(&self) -> Self::ProtoStruct {
//                 let mut proto = proto::Test::default();
//
//                 proto.set_type(ProtoMap::to_proto(&self.r#type).into());
//
//                 if let Some(value) = &self.opt_entity {
//                     proto.set_other_name(ProtoMap::to_proto(value).into());
//                 }
//
//                 proto
//             }
//
//             fn from_proto(proto: Self::ProtoStruct) -> std::result::Result<Self, anyhow::Error> {
//                 let inner = Self {
//                     r#type: ProtoMap::from_proto(proto.type_().to_owned())?,
//                     opt_entity: {
//                         let value = proto.other_name().to_owned();
//                         if proto.has_other_name() {
//                             Some(ProtoMap::from_proto(value)?)
//                         } else {
//                             None
//                         }
//                     },
//                 };
//                 Ok(inner)
//             }
//         }
//     };
//
//     let actual = s.implement_proto_map();
//     assert_tokens_eq(&expected, &actual);
// }
//
// #[test]
// fn implement_struct_scalars_with_attribute_overrides_test() {
//     let fragment = quote! {
//         #[proto_map(source = "proto::Test")]
//         struct Test {
//             // Map Uuid as scalar string
//             #[proto_map(scalar, with="uuid_as_string")]
//             field_1: Uuid,
//             // Map Option Uuid as scalar bytes
//             #[proto_map(scalar, with="uuid_as_bytes")]
//             field_2: Option<Uuid>,
//             // An already scalar type marked as scalar behaves the same
//             #[proto_map(scalar)]
//             field_3: u32,
//             // An already optional scalar type marked as scalar behaves the same
//             #[proto_map(scalar)]
//             field_4: Option<u32>,
//         }
//     };
//
//     let input = syn::parse2::<DeriveInput>(fragment.into()).unwrap();
//
//     let s = from_derive_input_struct(&input).unwrap();
//
//     let expected = quote! {
//         impl ProtoMap for Test {
//             type ProtoStruct = proto::Test;
//             fn to_proto(&self) -> Self::ProtoStruct {
//                 let mut proto = proto::Test::default();
//
//                 proto.set_field_1(uuid_as_string::to_scalar(&self.field_1).into());
//
//                 if let Some(value) = &self.field_2 {
//                     proto.set_field_2(uuid_as_bytes::to_scalar(value).into());
//                 }
//
//                 proto.set_field_3(ProtoMapScalar::to_scalar(&self.field_3).into());
//
//                 if let Some(value) = &self.field_4 {
//                     proto.set_field_4(ProtoMapScalar::to_scalar(value).into());
//                 }
//                 proto
//             }
//
//             fn from_proto(proto: Self::ProtoStruct) -> std::result::Result<Self, anyhow::Error> {
//                 let inner = Self {
//                     field_1: uuid_as_string::from_scalar(proto.field_1().to_owned())?,
//                     field_2: {
//                         let value = proto.field_2().to_owned();
//                         if ProtoScalar::has_value(&value) {
//                             Some(uuid_as_bytes::from_scalar(value)?)
//                         } else {
//                             None
//                         }
//                     },
//                     field_3: ProtoMapScalar::from_scalar(proto.field_3().to_owned())?,
//                     field_4: {
//                         let value = proto.field_4().to_owned();
//                         if ProtoScalar::has_value(&value) {
//                             Some(ProtoMapScalar::from_scalar(value)?)
//                         } else {
//                             None
//                         }
//                     },
//                 };
//                 Ok(inner)
//             }
//         }
//     };
//
//     let actual = s.implement_proto_map();
//     assert_tokens_eq(&expected, &actual);
// }
