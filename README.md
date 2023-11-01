# Proto Mapper
[<img alt="github" src="https://img.shields.io/badge/github-fpaschos/proto-mapper?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/fpaschos/proto-mapper)
[![CI/main](https://github.com/fpaschos/proto-mapper/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/fpaschos/proto-mapper/actions/workflows/ci.yml)
[<img alt="crates.io" src="https://img.shields.io/crates/v/proto-mapper.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/proto-mapper)

Macro implementation library for mapping between custom models and protobuf generated code

## Notice
This library is an (almost) complete rewrite of the [protobuf-convert](https://github.com/aleksuss/protobuf-convert/blob/master/README.md) library.
The purpose of the rewrite is to adapt it to specific needs of our projects.
The main concept and the idea remains the same, so the credit goes to the original authors of the `protobuf-convert` library.

## What changed
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

## Install

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

## Usage

A proof of concept that demonstrates the use of this library can be found [here](https://github.com/fpaschos/rust-kafka-debezium-demo/blob/main/claims-model/src/model/mod.rs). 
Keep in mind that the PoC is still work in progress.

### Mapping scalar values and enumerations
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

### Mapping optional scalar values and enumerations
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

### Mapping non scalar values

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

### Mapping non scalar `oneof` field to rust enumeration
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
### Custom mapping scalar values
See examples at tests [for prost](https://github.com/fpaschos/proto-mapper/blob/main/tests/prost/struct_scalar_custom_mappings_tests.rs) and [for rust-protobuf](https://github.com/fpaschos/proto-mapper/blob/main/tests/protobuf/struct_scalar_custom_mappings_tests.rs)

## Differences between `prost` and `rust-protobuf` usage
TODO

## How it works
Internally the macro discriminates between scalar and not scalar types.

Scalar types are shown below as well as the protobuf types that `rust-protobuf` and `prost` autogenerated code maps to.

| Protobuf Type | Rust Type |
| --- | --- |
| `double` | `f64` |
| `float` | `f32` |
| `int32` | `i32` |
| `int64` | `i64` |
| `uint32` | `u32` |
| `uint64` | `u64` |
| `sint32` | `i32` |
| `sint64` | `i64` |
| `fixed32` | `u32` |
| `fixed64` | `u64` |
| `sfixed32` | `i32` |
| `sfixed64` | `i64` |
| `bool` | `bool` |
| `string` | `String` |
| `bytes` | `Vec<u8>` |
_(table taken from prost project  README.md)_

All other rust types are considered as non scalar.
An exception to that rule is  protobuf `enum` types that need to be marked with meta attribute `#[proto_map(enumeration)]`,

The library implements automatically two traits according to the struct types.

For scalar types:
```rust
pub trait ProtoMapScalar<P: ProtoScalar>: Sized {
    /// Converts a reference of [`Self`] to a [`ProtoScalar`]
    fn to_scalar(&self) -> P;

    /// Consumes a [`ProtoScalar`] and returns a [`Self`] or error in the conversion failed
    fn from_scalar(proto: P) -> Result<Self, anyhow::Error>;
}
```

For non scalar types:
```rust
pub trait ProtoMap
    where
        Self: Sized,
{
    type ProtoStruct;
    /// Converts a reference of [`Self`] struct to proto [`Self::ProtoStruct`]
    fn to_proto(&self) -> Self::ProtoStruct;

    /// Consumes a proto [`Self::ProtoStruct`] and returns a [`Self`] struct or error in the conversion failed
    fn from_proto(proto: Self::ProtoStruct) -> Result<Self, anyhow::Error>;
}
```

Note that protobuf `enum` types are treated as non scalar with `rust-protobuf` but as scalar (i32 value) with `prost`.  

Also a third trait named `ProtoScalar` implementation is provided by the library for all proto scalar types.

To get a rough view of what the macro implement see the [prost manual tests](https://github.com/fpaschos/proto-mapper/blob/main/tests/prost/manual_implementation_tests.rs) or the [rust-protobuf manual tests](https://github.com/fpaschos/proto-mapper/blob/main/tests/protobuf/manual_implementation_tests.rs) used as guides for creating the implementations.


## Limitations
TODO

## Related Projects
- [Github: protobuf-convert](https://github.com/aleksuss/protobuf-convert/blob/master/README.md)
- [Github: rust-protobuf](https://github.com/stepancheg/rust-protobuf)
- [Github: prost](https://github.com/tokio-rs/prost)

### Resources
- [The little book of Rust Macros](https://veykril.github.io/tlborm/introduction.html)
- [The Rust reference](https://doc.rust-lang.org/reference/introduction.html)
- [How to write hygienic macros](https://gist.github.com/Kestrer/8c05ebd4e0e9347eb05f265dfb7252e1)
- [Medium: Nine rules for creating procedural macros in rust](https://towardsdatascience.com/nine-rules-for-creating-procedural-macros-in-rust-595aa476a7ff)