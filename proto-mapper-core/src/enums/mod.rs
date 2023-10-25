#[cfg(feature = "prost")]
mod prost_enum;

#[cfg(feature = "prost")]
pub(crate) use prost_enum::{Enum, EnumVariant};

#[cfg(feature = "protobuf")]
mod protobuf_enum;
#[cfg(feature = "protobuf")]
pub(crate) use protobuf_enum::{Enum, EnumVariant};

mod attrs;
pub(crate) use attrs::*;