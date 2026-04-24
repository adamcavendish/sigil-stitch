//! Generate a Rust file using structural specs.
//!
//! Run with: `cargo run --example rust_codegen`

use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::rust_lang::RustLang;
use sigil_stitch::spec::field_spec::FieldSpec;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::modifiers::{TypeKind, Visibility};
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::spec::type_spec::TypeSpec;
use sigil_stitch::type_name::TypeName;

fn main() {
    // Define types from different crate groups.
    let hashmap = TypeName::importable("std::collections", "HashMap");
    let serialize = TypeName::importable("serde", "Serialize");
    let deserialize = TypeName::importable("serde", "Deserialize");

    // Build a struct using TypeSpec.
    let tb = TypeSpec::builder("Config", TypeKind::Struct).visibility(Visibility::Public);

    // Derive annotation.
    let derive = CodeBlock::of("#[derive(%T, %T)]", (serialize, deserialize)).unwrap();
    let tb = tb.annotation(derive);

    // Fields.
    let tb = tb
        .add_field(
            FieldSpec::builder("name", TypeName::primitive("String"))
                .visibility(Visibility::Public)
                .build()
                .unwrap(),
        )
        .add_field(
            FieldSpec::builder(
                "values",
                TypeName::generic(
                    hashmap,
                    vec![TypeName::primitive("String"), TypeName::primitive("i64")],
                ),
            )
            .visibility(Visibility::Public)
            .build()
            .unwrap(),
        );

    // Constructor method.
    let body = CodeBlock::of(
        "Self { name: name.to_string(), values: HashMap::new() }",
        (),
    )
    .unwrap();
    let tb = tb.add_method(
        FunSpec::builder("new")
            .visibility(Visibility::Public)
            .add_param(ParameterSpec::new("name", TypeName::primitive("&str")).unwrap())
            .returns(TypeName::primitive("Self"))
            .body(body)
            .build()
            .unwrap(),
    );

    // Build and render.
    let spec = FileSpec::builder_with("config.rs", RustLang::new())
        .add_type(tb.build().unwrap())
        .build()
        .unwrap();

    let output = spec.render(100).unwrap();
    println!("{output}");
}
