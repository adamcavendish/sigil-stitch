//! Generate a Rust file with grouped imports and control flow.
//!
//! Run with: `cargo run --example rust_codegen`

use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::rust_lang::RustLang;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

fn main() {
    // Define types from different crate groups.
    let hashmap = TypeName::<RustLang>::importable("std::collections", "HashMap");
    let serialize = TypeName::<RustLang>::importable("serde", "Serialize");
    let deserialize = TypeName::<RustLang>::importable("serde", "Deserialize");

    // Build a struct with derive.
    let mut s = CodeBlock::<RustLang>::builder();
    s.add("#[derive(%T, %T)]", (serialize, deserialize));
    s.add_line();
    s.add("pub struct Config {", ());
    s.add_line();
    s.add("%>", ());
    s.add_statement("pub name: String", ());
    s.add_statement("pub values: %T<String, i64>", (hashmap,));
    s.add("%<", ());
    s.add("}", ());
    s.add_line();
    let struct_block = s.build().unwrap();

    // Build a function.
    let mut f = CodeBlock::<RustLang>::builder();
    f.add("impl Config {", ());
    f.add_line();
    f.add("%>", ());
    f.add("pub fn new(name: &str) -> Self {", ());
    f.add_line();
    f.add("%>", ());
    f.add_statement("Self { name: name.to_string(), values: HashMap::new() }", ());
    f.add("%<", ());
    f.add("}", ());
    f.add_line();
    f.add("%<", ());
    f.add("}", ());
    f.add_line();
    let impl_block = f.build().unwrap();

    // Build the file.
    let mut file = FileSpec::builder_with("config.rs", RustLang::new());
    file.add_code(struct_block);
    file.add_code(impl_block);
    let spec = file.build();

    // Render at 100 columns.
    let output = spec.render(100).unwrap();
    println!("{output}");
}
