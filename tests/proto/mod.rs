#[cfg(feature="protobuf")]
pub mod protobuf {
    include!(concat!(env!("OUT_DIR"), "/mod.rs"));
    pub use entities::*;
}

#[cfg(feature="prost")]
pub mod prost {
    include!(concat!(env!("OUT_DIR"), "/entities.schema.rs"));
}
