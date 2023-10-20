use crate::proto_enum::Enum;
use crate::proto_struct::Struct;
use proc_macro2::TokenStream;
use syn::{Data, DeriveInput};

mod implement_enum_tests;
mod implement_struct_tests;
mod parse_enum_tests;
mod parse_struct_tests;
mod type_scanner_tests;

#[cfg(test)]
pub(crate) fn from_derive_input_struct(input: &DeriveInput) -> darling::Result<Struct> {
    if let Data::Struct(data) = &input.data {
        let s = Struct::try_from_data(&input.ident, data, &input.attrs)?;
        Ok(s)
    } else {
        Err(darling::Error::unsupported_shape("Expected `struct` item"))
    }
}

#[cfg(test)]
pub(crate) fn from_derive_input_enum(input: &DeriveInput) -> darling::Result<Enum> {
    if let Data::Enum(data) = &input.data {
        let s = Enum::try_from_data(&input.ident, data, &input.attrs)?;
        Ok(s)
    } else {
        Err(darling::Error::unsupported_shape("Expected `struct` item"))
    }
}

#[cfg(test)]
pub fn assert_tokens_eq(expected: &TokenStream, actual: &TokenStream) {
    let expected = expected.to_string();
    let actual = actual.to_string();

    if expected != actual {
        println!(
            "{}",
            colored_diff::PrettyDifference {
                expected: &expected,
                actual: &actual,
            }
        );
        println!("expected: {}", &expected);
        println!("actual  : {}", &actual);
        panic!("expected != actual");
    }
}
