//! Example: Generate a C header file with sigil-stitch.
//!
//! Demonstrates:
//! - `#pragma once` file header
//! - `#include` directives (system and local)
//! - Struct definition with typed fields
//! - Enum via extra_member
//! - Function declarations (no body) and function definitions (with body)
//! - Type-before-name and return-type-as-prefix emission
//!
//! Run: `cargo run --example c_codegen`

use sigil_stitch::code_block::{CodeBlock, StringLitArg};
use sigil_stitch::lang::c_lang::CLang;
use sigil_stitch::spec::field_spec::FieldSpec;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::modifiers::TypeKind;
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::spec::type_spec::TypeSpec;
use sigil_stitch::type_name::TypeName;

fn main() {
    // --- Types that trigger #include ---
    let printf = TypeName::importable("stdio.h", "printf");
    let malloc = TypeName::importable("stdlib.h", "malloc");
    let free = TypeName::importable("stdlib.h", "free");

    // --- Enum: LogLevel ---
    let enum_b = TypeSpec::builder("LogLevel", TypeKind::Enum);
    let enum_b = enum_b.doc("Severity levels for the logging system.");
    let mut members = CodeBlock::builder();
    members.add("LOG_DEBUG,", ());
    members.add_line();
    members.add("LOG_INFO,", ());
    members.add_line();
    members.add("LOG_WARN,", ());
    members.add_line();
    members.add("LOG_ERROR", ());
    members.add_line();
    let enum_b = enum_b.extra_member(members.build().unwrap());
    let log_level = enum_b.build().unwrap();

    // --- Struct: Config ---
    let config = TypeSpec::builder("Config", TypeKind::Struct)
        .doc("Application configuration.")
        .add_field(
            FieldSpec::builder("host", TypeName::primitive("const char*"))
                .build()
                .unwrap(),
        )
        .add_field(
            FieldSpec::builder("port", TypeName::primitive("int"))
                .build()
                .unwrap(),
        )
        .add_field(
            FieldSpec::builder("log_level", TypeName::primitive("enum LogLevel"))
                .build()
                .unwrap(),
        )
        .add_field(
            FieldSpec::builder("max_connections", TypeName::primitive("int"))
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    // --- Function: config_create ---
    let mut create_body_b = CodeBlock::builder();
    create_body_b.add(
        "struct Config* cfg = (struct Config*)%T(sizeof(struct Config));",
        (malloc,),
    );
    create_body_b.add_line();
    create_body_b.add("cfg->host = host;", ());
    create_body_b.add_line();
    create_body_b.add("cfg->port = port;", ());
    create_body_b.add_line();
    create_body_b.add("cfg->log_level = LOG_INFO;", ());
    create_body_b.add_line();
    create_body_b.add("cfg->max_connections = 100;", ());
    create_body_b.add_line();
    create_body_b.add("return cfg;", ());
    let create_body = create_body_b.build().unwrap();

    let config_create = FunSpec::builder("config_create")
        .add_param(ParameterSpec::new("host", TypeName::primitive("const char*")).unwrap())
        .add_param(ParameterSpec::new("port", TypeName::primitive("int")).unwrap())
        .returns(TypeName::primitive("struct Config*"))
        .body(create_body)
        .build()
        .unwrap();

    // --- Function: config_destroy ---
    let destroy_body = CodeBlock::of("%T(cfg);", (free,)).unwrap();
    let config_destroy = FunSpec::builder("config_destroy")
        .add_param(ParameterSpec::new("cfg", TypeName::primitive("struct Config*")).unwrap())
        .returns(TypeName::primitive("void"))
        .body(destroy_body)
        .build()
        .unwrap();

    // --- Function: config_print ---
    let print_body = CodeBlock::of(
        "%T(%S, cfg->host, cfg->port, cfg->log_level);",
        (
            printf,
            StringLitArg("Config { host=%s, port=%d, level=%d }\\n".to_string()),
        ),
    )
    .unwrap();
    let print_fn = FunSpec::builder("config_print")
        .add_param(ParameterSpec::new("cfg", TypeName::primitive("const struct Config*")).unwrap())
        .returns(TypeName::primitive("void"))
        .body(print_body);
    let config_print = print_fn.build().unwrap();

    // --- Assemble file ---
    let file = FileSpec::builder_with("config.h", CLang::header())
        .header(CodeBlock::of("#pragma once", ()).unwrap())
        .add_type(log_level)
        .add_type(config)
        .add_function(config_create)
        .add_function(config_destroy)
        .add_function(config_print)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();
    print!("{output}");
}
