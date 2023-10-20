use proc_macro::TokenStream;
use proto_mapper_core::proto_map::implement_proto_map;

#[proc_macro_derive(ProtoMap, attributes(proto_map))]
pub fn generate_proto_map(input: TokenStream) -> TokenStream {
    implement_proto_map(input.into()).into()
}
