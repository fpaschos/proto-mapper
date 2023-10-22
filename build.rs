use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    // Generate protobuf crate proto rust code
    let out_dir = std::env::var("OUT_DIR").expect("Unable to get OUT_DIR");

    protobuf_codegen::Codegen::new()
        .pure()
        .out_dir(&out_dir)
        .input("tests/proto/entities.proto")
        .include("tests/proto")
        .customize(
            protobuf_codegen::Customize::default().generate_accessors(true), // .gen_mod_rs(true),
        )
        .run_from_script();

    // Override the mod file to include all the generated protos
    let mod_file_content = r#"//@generated
    pub mod entities;
    "#;
    let mod_file_path = Path::new(&out_dir).join("mod.rs");

    let mut file = fs::File::create(mod_file_path).expect("Unable to create mod.rs file");
    file.write_all(mod_file_content.to_string().as_ref())
        .expect("Unable to write mod.rs file");

    // Generate prost crate proto rust code
    prost_build::compile_protos(&["tests/proto/entities.proto"], &["/tests"]).unwrap();
}
