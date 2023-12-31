#[cfg(feature = "prost")]
mod prost_field;
#[cfg(feature = "prost")]
pub(crate) use prost_field::StructField;

#[cfg(feature = "protobuf")]
mod protobuf_field;
#[cfg(feature = "protobuf")]
pub(crate) use protobuf_field::StructField;

mod r#struct;

pub(crate) use r#struct::Struct;

mod attrs;

pub(crate) use attrs::*;
