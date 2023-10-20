use anyhow::Error;

pub trait ProtoScalar: Sized + private::Sealed {
    fn has_value(&self) -> bool;
}

mod private {
    // see https://rust-lang.github.io/api-guidelines/future-proofing.html#c-sealed for sealed trait
    pub trait Sealed {}
    impl Sealed for u32 {}
    impl Sealed for i32 {}
    impl Sealed for u64 {}
    impl Sealed for i64 {}
    impl Sealed for f64 {}
    impl Sealed for f32 {}
    impl Sealed for bool {}
    impl Sealed for String {}
    impl Sealed for Vec<u8> {}
}

pub trait ProtoMapScalar<P: ProtoScalar>: Sized {
    /// Converts a reference of [`Self`] to a [`ProtoScalar`]
    fn to_scalar(&self) -> P;

    /// Consumes a [`ProtoScalar`] and returns a [`Self`] or error in the conversion failed
    fn from_scalar(proto: P) -> Result<Self, anyhow::Error>;
}

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

macro_rules! impl_proto_scalar {
    ( $( $name:tt ),* )=> {
        $(
            impl ProtoScalar for $name {
                fn has_value(&self) -> bool {
                    *self != 0 as $name
                }
            }
        )*
    };
}

impl_proto_scalar! { u32, u64, i32, i64, f32, f64}

impl ProtoScalar for bool {
    fn has_value(&self) -> bool {
        *self
    }
}

impl ProtoScalar for String {
    fn has_value(&self) -> bool {
        !self.is_empty()
    }
}

impl ProtoScalar for Vec<u8> {
    fn has_value(&self) -> bool {
        !self.is_empty()
    }
}

macro_rules! impl_proto_convert_scalar {
    ( $( $name:tt ),* )=> {
        $(
            impl ProtoMapScalar<$name> for $name {
                fn to_scalar(&self) -> $name {
                    *self
                }
                fn from_scalar(proto: $name) -> Result<Self, Error> {
                    Ok(proto)
                }
            }
        )*
    };
}

impl_proto_convert_scalar! { bool, u32, u64, i32, i64, f32, f64 }

impl ProtoMapScalar<String> for String {
    fn to_scalar(&self) -> String {
        self.clone()
    }

    fn from_scalar(proto: String) -> Result<Self, Error> {
        Ok(proto)
    }
}

impl ProtoMapScalar<Vec<u8>> for Vec<u8> {
    fn to_scalar(&self) -> Vec<u8> {
        self.clone()
    }

    fn from_scalar(proto: Vec<u8>) -> Result<Self, Error> {
        Ok(proto)
    }
}
