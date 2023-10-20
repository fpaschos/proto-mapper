pub mod protobuf {
    include!(concat!(env!("OUT_DIR"), "/mod.rs"));
    pub use entities::*;
    pub use timestamps::*;
}

pub mod prost {
    include!(concat!(env!("OUT_DIR"), "/entities.schema.rs"));
}
