use heck::{ToShoutySnakeCase, ToSnakeCase};
use syn::{Attribute, Meta};

pub mod proto_map;
mod proto_enum;
mod proto_struct;

#[cfg(test)]
mod tests;
mod types;

const PROTO_MAP_ATTRIBUTE: &str = "proto_map";
const SNAKE_CASE_ATTRIBUTE_VALUE: &str = "snake_case";

const SCREAMING_SNAKE_CASE_ATTRIBUTE_VALUE: &str = "STREAMING_SNAKE_CASE";

pub(crate) fn find_proto_map_meta(attrs: &[Attribute]) -> Option<&Meta> {
    attrs
        .iter()
        .find(|a| a.path().is_ident(PROTO_MAP_ATTRIBUTE))
        .map(|a| &a.meta)
}

pub(crate) fn rename_item(item: &str, to_case: &str) -> darling::Result<String> {
    match to_case {
        SNAKE_CASE_ATTRIBUTE_VALUE => Ok(item.to_string().to_snake_case()),
        SCREAMING_SNAKE_CASE_ATTRIBUTE_VALUE => Ok(item.to_string().to_shouty_snake_case()),

        _ => Err(darling::Error::unknown_value(&format!(
            "Unknown rename case attribute = `{}` ",
            to_case
        ))),
    }
}

/// Returns a struct field name given an identifier and a rename field attribute.
/// remove_last_char_if is used in cases that we want to remove special characters such as '_'
pub(crate) fn get_proto_field_name(name: &str, remove_last_char_if: Option<char>) -> String {
    if let Some(c) = remove_last_char_if {
        let mut rename_rev = name.chars().rev().peekable();
        if rename_rev.peek().copied() == Some(c) {
            return name[..name.len() - 1].to_string();
        }
    }
    name.to_string()
}
