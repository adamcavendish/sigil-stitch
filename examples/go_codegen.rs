//! Generate a Go file using structural specs.
//!
//! Run with: `cargo run --example go_codegen`

use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::go_lang::GoLang;
use sigil_stitch::spec::field_spec::FieldSpec;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::modifiers::TypeKind;
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::spec::type_spec::TypeSpec;
use sigil_stitch::type_name::TypeName;

fn main() {
    // Importable types.
    let json_marshal = TypeName::importable("encoding/json", "Marshal");
    let http_server = TypeName::importable("net/http", "Server");

    // Build a struct with struct tags.
    let tb = TypeSpec::builder("Config", TypeKind::Struct)
        .doc("Config holds application configuration.")
        .add_field(
            FieldSpec::builder("Host", TypeName::primitive("string"))
                .tag("json:\"host\"")
                .build()
                .unwrap(),
        )
        .add_field(
            FieldSpec::builder("Port", TypeName::primitive("int"))
                .tag("json:\"port\"")
                .build()
                .unwrap(),
        );

    // Build receiver methods as separate top-level functions.
    let start_fn = FunSpec::builder("Start")
        .doc("Start begins listening on the configured address.")
        .receiver(
            ParameterSpec::new("c", TypeName::pointer(TypeName::primitive("Config"))).unwrap(),
        )
        .returns(TypeName::primitive("error"))
        .body(CodeBlock::of("return %T(c.Host, nil)", (http_server,)).unwrap());

    let to_json_fn = FunSpec::builder("ToJSON")
        .receiver(
            ParameterSpec::new("c", TypeName::pointer(TypeName::primitive("Config"))).unwrap(),
        )
        .returns(TypeName::raw("([]byte, error)"))
        .body(CodeBlock::of("return %T(c)", (json_marshal,)).unwrap());

    // Assemble the file.
    let spec = FileSpec::builder_with("config.go", GoLang::new())
        .header(CodeBlock::of("package config", ()).unwrap())
        .add_type(tb.build().unwrap())
        .add_function(start_fn.build().unwrap())
        .add_function(to_json_fn.build().unwrap())
        .build()
        .unwrap();

    let output = spec.render(100).unwrap();
    println!("{output}");
}
