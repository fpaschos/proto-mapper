## Proto Mapper
[![CI/main](https://github.com/fpaschos/proto-mapper/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/fpaschos/proto-mapper/actions/workflows/ci.yml)

Macro implementation library for mapping between custom models and protobuf generated code

### Notice
This library is an (almost) complete rewrite of the [protobuf-convert](https://github.com/aleksuss/protobuf-convert/blob/master/README.md) library.
The purpose of the rewrite is to adapt it to specific needs of our projects.
The main concept and the idea remains the same, so the credit goes to the original authors of the `protobuf-convert` library.

### What changed
This library:
- changes the main name of the macro to `ProtoMap`
- changes the main way the macro is used and is interfaced with external traits
- avoids the use of re implementing ProtoMap trait to client modules
- is restructured to different crates
- contains excessive testing for edge cases
- introduces ProtoScalar types
- introduces ProtoScalarMap trait for protobuf scalar types
- handles enumeration protobuf generation code automatically
- handles option values via scanning the types of the applied struct and chooses different implementation paths
- supports prost

### Install

First, add the dependency in `Cargo.toml`:

```toml
proto-mapper = {version = "0.1.2", features = ["protobuf"] } 
```

or 

```toml
proto-mapper = {version = "0.1.2", features = ["prost"] } 
```

__NOTE__: Features `prost` or `protobuf` are __mutually exclusive and required__.
Use one of them according to targeted generated code proto framework that you use

### Usage

A proof of concept that demonstrates the use of this library can be found [here](https://github.com/fpaschos/rust-kafka-debezium-demo/blob/main/claims-model/src/model/mod.rs). 
Keep in mind that the PoC is still work in progress.

##### Mapping scalar values and enumerations
Given the protobuf enumeration and message
```protobuf
syntax = "proto3";

enum EntityStatus {
  STATUS_A = 0;
  STATUS_B = 1;
  STATUS_C = 2;
}

message ScalarEntity {
  uint32 uint32_f = 1;
  int32 int32_f= 2;
  bool bool_f = 4;
  string string_f = 5;
  int64  int64_f = 6;
  uint64 uint64_f  = 7;
  bytes bytes_f = 8;
  float float_f = 9;
  double double_f = 10;

  EntityStatus status = 11;
}
```

After using `prost` or `rust-protobuf` library to generate code, you can map your custom model to the generated structs as follows:

```rust

#[derive(Debug, Clone, Copy, Default, PartialEq, ProtoMap)]
#[proto_map(
source = "proto::EntityStatus",
enumeration,
)]
enum EntityStatus {
    #[default]
    StatusA,
    StatusB,
    StatusC,
}

#[derive(Debug, ProtoMap, Default)]
#[proto_map(source = "proto::ScalarEntity")]
struct ScalarEntity {
    pub uint32_f: u32,
    pub int32_f: i32,
    pub bool_f: bool,
    pub string_f: String,
    pub bytes_f: Vec<u8>,
    pub int64_f: i64,
    pub uint64_f: u64,
    pub float_f: f32,
    pub double_f: f64,
    #[proto_map(enumeration)]
    pub status: EntityStatus,
}
```

Then you can convert between your defined struct and the generated code as follows:

```rust
let e = ScalarEntity::default();
let p = e.to_proto();
```

You can also convert a proto instance to your custom struct.
```rust
let p = proto::ScalarEntity::default();
let e = ScalarEntity::from_proto(p)?;
```

Note that the mapping code for the enumeration requires `#[proto_map(..., enumeration)]` attribute on the rust enumeration
and also needs to mark the field inside the `ScalarEntity` as well.

##### Mapping optional scalar values and enumerations
Given the same proto file. Out of the box you can map to optional values

That is:

```rust
#[derive(Debug, ProtoMap, PartialEq, Default)]
#[proto_map(source = "proto::prost::ScalarEntity")]
struct ScalarEntityOptions {
    pub uint32_f: Option<u32>,
    pub int32_f: Option<i32>,
    pub bool_f: Option<bool>,
    pub string_f: Option<String>,
    pub bytes_f: Option<Vec<u8>>,
    pub int64_f: Option<i64>,
    pub uint64_f: Option<u64>,
    pub float_f: Option<f32>,
    pub double_f: Option<f64>,
    #[proto_map(enumeration)]
    pub status: Option<EntityStatus>,
}
```
The macro scans  the types of the custom struct that annotates and chooses different implementation paths for the conversion code.

##### Mapping non scalar values

Given the proto file 
```protobuf
syntax = "proto3";

// ... definitions of ScalarEntity

message NestedEntity {
  ScalarEntity first = 1;
  ScalarEntity second = 2;
}
```

You can map non scalar values as follows

```rust 
#[derive(Debug, ProtoMap, PartialEq)]
#[proto_map(source = "proto::NestedEntity")]
struct NestedEntity {
    pub first: ScalarEntity,
    pub second: Option<ScalarEntity>,
}
```

##### Mapping non scalar `oneof` field to rust enumeration
You can map top level `oneof` protobuf fields as follows

Given the proto file
```protobuf
syntax = "proto3";

// ... definitions of ScalarEntity

message HierarchyEntity {
  oneof data {
    ScalarEntity first_entity = 1;
    NestedEntity second_entity = 2;
  }
}
```

Then one implementation of the custom struct may be
```rust 
#[derive(Debug, ProtoMap, PartialEq)]
#[proto_map(
    source = "proto::HierarchyEntity",
    one_of(field = "data"),
    rename_variants = "snake_case"
)]
enum HierarchyEntity {
    FirstEntity(ScalarEntity),
    SecondEntity(NestedEntity),
}
```

Note that the `rename_variants` attribute may take two values `snake_case` and `STREAMING_SNAKE_CASE` according to the target generated struct.
##### Custom mapping scalar values
TODO

### Differences between `prost` and `protobuf` usage
TODO

### How it works
TODO

### Limitations
TODO




### Related Projects
- [Github: protobuf-convert](https://github.com/aleksuss/protobuf-convert/blob/master/README.md)

#### Resources
- [The little book of Rust Macros](https://veykril.github.io/tlborm/introduction.html)
- [The Rust reference](https://doc.rust-lang.org/reference/introduction.html)
- [How to write hygienic macros](https://gist.github.com/Kestrer/8c05ebd4e0e9347eb05f265dfb7252e1)
- [Medium: Nine rules for creating procedural macros in rust](https://towardsdatascience.com/nine-rules-for-creating-procedural-macros-in-rust-595aa476a7ff)