#[cfg(feature = "prost")]
mod prost_enum;
#[cfg(feature = "prost")]
pub(crate) use prost_enum::Enum;
#[cfg(test)]
#[cfg(feature = "prost")]
pub(crate) use prost_enum::EnumVariant;

#[cfg(feature = "protobuf")]
mod protobuf_enum;
#[cfg(feature = "protobuf")]
pub(crate) use protobuf_enum::Enum;
#[cfg(test)]
#[cfg(feature = "protobuf")]
pub(crate) use protobuf_enum::EnumVariant;

mod attrs;
pub(crate) use attrs::*;