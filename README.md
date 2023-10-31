## Proto Mapper
[![CI/main](https://github.com/fpaschos/proto-mapper/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/fpaschos/proto-mapper/actions/workflows/ci.yml)

Macro implementation library for mapping between custom models and protobuf generated code
### Usage

First, add the dependency in `Cargo.toml`:

```toml
proto-mapper = {version = "0.1.0", features = ["protobuf"] } 
```

or 

```toml
proto-mapper = {version = "0.1.0", features = ["prost"] } 
```

__NOTE__: Features `prost` or `protobuf` are __mutually exclusive and required__.
Use one of them according to targeted generated code proto framework that you use


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


### Related Projects
- [Github: protobuf-convert](https://github.com/aleksuss/protobuf-convert/blob/master/README.md)

#### Resources
- [The little book of Rust Macros](https://veykril.github.io/tlborm/introduction.html)
- [The Rust reference](https://doc.rust-lang.org/reference/introduction.html)
- [How to write hygienic macros](https://gist.github.com/Kestrer/8c05ebd4e0e9347eb05f265dfb7252e1)
- [Medium: Nine rules for creating procedural macros in rust](https://towardsdatascience.com/nine-rules-for-creating-procedural-macros-in-rust-595aa476a7ff)