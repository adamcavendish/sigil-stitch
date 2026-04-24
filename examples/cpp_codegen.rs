//! Example: Generate a C++ header file with sigil-stitch.
//!
//! Demonstrates:
//! - `#pragma once` file header
//! - `#include` directives (system and local)
//! - Namespace wrapping via raw content
//! - Abstract base class with pure virtual methods
//! - Derived class with `override` suffix
//! - Template function via annotation
//! - `enum class` (scoped enum)
//! - Method suffixes (`const`, `override`, `= 0`)
//! - Access specifiers (`public:`, `private:`) via extra_member
//!
//! Run: `cargo run --example cpp_codegen`

use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::cpp_lang::CppLang;
use sigil_stitch::spec::field_spec::FieldSpec;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::modifiers::{DeclarationContext, TypeKind};
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::spec::type_spec::TypeSpec;
use sigil_stitch::type_name::TypeName;

/// Helper: emit a FunSpec as a CodeBlock for embedding in extra_member.
fn emit_fun(fun: &FunSpec) -> CodeBlock {
    let lang = CppLang::new();
    fun.emit(&lang, DeclarationContext::Member).unwrap()
}

/// Helper: emit a FieldSpec as a CodeBlock for embedding in extra_member.
fn emit_field(field: &FieldSpec) -> CodeBlock {
    let lang = CppLang::new();
    field.emit(&lang, DeclarationContext::Member).unwrap()
}

fn main() {
    // --- Imports ---
    let iostream = TypeName::importable("iostream", "std::cout");
    let string_h = TypeName::importable("string", "std::string");
    let vector_h = TypeName::importable("vector", "std::vector");

    // --- Enum class: LogLevel ---
    let enum_b = TypeSpec::builder("LogLevel", TypeKind::Enum);
    let enum_b = enum_b.doc("Severity levels for the logging system.");
    let mut members = CodeBlock::builder();
    members.add("Debug,", ());
    members.add_line();
    members.add("Info,", ());
    members.add_line();
    members.add("Warning,", ());
    members.add_line();
    members.add("Error", ());
    members.add_line();
    let enum_b = enum_b.extra_member(members.build().unwrap());
    let log_level = enum_b.build().unwrap();

    // --- Abstract base class: Logger ---
    let logger_b = TypeSpec::builder("Logger", TypeKind::Class);
    let logger_b = logger_b.doc("Abstract base class for loggers.");

    let mut pub_section = CodeBlock::builder();
    pub_section.add("%<", ());
    pub_section.add("public:", ());
    pub_section.add_line();
    pub_section.add("%>", ());

    // Pure virtual: virtual void log(const char* msg) = 0;
    pub_section.add_code(emit_fun(
        &FunSpec::builder("log")
            .is_abstract()
            .add_param(ParameterSpec::new("msg", TypeName::primitive("const char*")).unwrap())
            .returns(TypeName::primitive("void"))
            .suffix("= 0")
            .build()
            .unwrap(),
    ));

    // Pure virtual: virtual LogLevel level() const = 0;
    pub_section.add_line();
    pub_section.add_code(emit_fun(
        &FunSpec::builder("level")
            .is_abstract()
            .returns(TypeName::primitive("LogLevel"))
            .suffix("const")
            .suffix("= 0")
            .build()
            .unwrap(),
    ));

    // Virtual destructor
    pub_section.add_line();
    pub_section.add_code(emit_fun(
        &FunSpec::builder("~Logger")
            .is_abstract()
            .suffix("= default")
            .build()
            .unwrap(),
    ));

    let logger_b = logger_b.extra_member(pub_section.build().unwrap());
    let logger = logger_b.build().unwrap();

    // --- Derived class: ConsoleLogger ---
    let console_b = TypeSpec::builder("ConsoleLogger", TypeKind::Class);
    let console_b = console_b.extends(TypeName::primitive("Logger"));
    let console_b = console_b.doc("Logger that writes to stdout.");

    // private: section
    let mut priv_section = CodeBlock::builder();
    priv_section.add("%<", ());
    priv_section.add("private:", ());
    priv_section.add_line();
    priv_section.add("%>", ());
    let name_field = FieldSpec::builder("name_", TypeName::primitive("std::string"))
        .build()
        .unwrap();
    priv_section.add_code(emit_field(&name_field));
    let level_field = FieldSpec::builder("level_", TypeName::primitive("LogLevel"))
        .build()
        .unwrap();
    priv_section.add_code(emit_field(&level_field));
    let console_b = console_b.extra_member(priv_section.build().unwrap());

    // public: section
    let mut pub_section2 = CodeBlock::builder();
    pub_section2.add_line();
    pub_section2.add("%<", ());
    pub_section2.add("public:", ());
    pub_section2.add_line();
    pub_section2.add("%>", ());

    // Constructor
    let ctor_body = CodeBlock::of("name_ = name;\nlevel_ = LogLevel::Info;", ()).unwrap();
    pub_section2.add_code(emit_fun(
        &FunSpec::builder("ConsoleLogger")
            .add_param(
                ParameterSpec::new("name", TypeName::primitive("const std::string&")).unwrap(),
            )
            .body(ctor_body)
            .build()
            .unwrap(),
    ));

    // log override
    pub_section2.add_line();
    let log_body = CodeBlock::of(
        "%T << \"[\" << name_ << \"] \" << %T(msg) << std::endl;",
        (iostream, string_h),
    )
    .unwrap();
    pub_section2.add_code(emit_fun(
        &FunSpec::builder("log")
            .add_param(ParameterSpec::new("msg", TypeName::primitive("const char*")).unwrap())
            .returns(TypeName::primitive("void"))
            .suffix("override")
            .body(log_body)
            .build()
            .unwrap(),
    ));

    // level override
    pub_section2.add_line();
    let level_body = CodeBlock::of("return level_;", ()).unwrap();
    pub_section2.add_code(emit_fun(
        &FunSpec::builder("level")
            .returns(TypeName::primitive("LogLevel"))
            .suffix("const")
            .suffix("override")
            .body(level_body)
            .build()
            .unwrap(),
    ));

    let console_b = console_b.extra_member(pub_section2.build().unwrap());
    let console_logger = console_b.build().unwrap();

    // --- Template function: make_vector ---
    let make_vec_fn = FunSpec::builder("make_vector");
    let make_vec_fn = make_vec_fn.annotation(CodeBlock::of("template<typename T>", ()).unwrap());
    let make_vec_fn =
        make_vec_fn.add_param(ParameterSpec::new("first", TypeName::primitive("T")).unwrap());
    let make_vec_fn =
        make_vec_fn.add_param(ParameterSpec::new("second", TypeName::primitive("T")).unwrap());
    let make_vec_fn = make_vec_fn.returns(TypeName::primitive("std::vector<T>"));
    let vec_body = CodeBlock::of(
        "%T<T> result;\nresult.push_back(first);\nresult.push_back(second);\nreturn result;",
        (vector_h,),
    )
    .unwrap();
    let make_vec_fn = make_vec_fn.body(vec_body);
    let make_vector = make_vec_fn.build().unwrap();

    // --- Assemble file ---
    let file = FileSpec::builder_with("logging.hpp", CppLang::header())
        .header(CodeBlock::of("#pragma once", ()).unwrap())
        .add_raw("\nnamespace app {\n\n")
        .add_type(log_level)
        .add_type(logger)
        .add_type(console_logger)
        .add_function(make_vector)
        .add_raw("\n} // namespace app\n")
        .build()
        .unwrap();
    let output = file.render(80).unwrap();
    print!("{output}");
}
