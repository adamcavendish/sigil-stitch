//! Generate a Python file using structural specs.
//!
//! Run with: `cargo run --example python_codegen`

use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::python::Python;
use sigil_stitch::spec::field_spec::FieldSpec;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::modifiers::TypeKind;
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::spec::type_spec::TypeSpec;
use sigil_stitch::type_name::TypeName;

fn main() {
    // Importable types.
    let json_dumps = TypeName::importable("json", "dumps");
    let dataclass_import = TypeName::importable("dataclasses", "dataclass");

    // Build a dataclass.
    let tb = TypeSpec::builder("Config", TypeKind::Class)
        .doc("Application configuration.")
        .annotation(CodeBlock::of("@%T", (dataclass_import,)).unwrap())
        .add_field(
            FieldSpec::builder("host", TypeName::primitive("str"))
                .build()
                .unwrap(),
        )
        .add_field(
            FieldSpec::builder("port", TypeName::primitive("int"))
                .build()
                .unwrap(),
        )
        .add_field(
            FieldSpec::builder("debug", TypeName::primitive("bool"))
                .initializer(CodeBlock::of("False", ()).unwrap())
                .build()
                .unwrap(),
        )
        .add_method(
            FunSpec::builder("to_json")
                .doc("Serialize to JSON string.")
                .add_param(ParameterSpec::new("self", TypeName::primitive("")).unwrap())
                .returns(TypeName::primitive("str"))
                .body(
                    CodeBlock::of(
                        "return %T({'host': self.host, 'port': self.port})",
                        (json_dumps,),
                    )
                    .unwrap(),
                )
                .build()
                .unwrap(),
        );

    // Build a standalone function.
    let greet = FunSpec::builder("greet")
        .add_param(ParameterSpec::new("name", TypeName::primitive("str")).unwrap())
        .returns(TypeName::primitive("str"))
        .body(CodeBlock::of("return f'Hello, {name}!'", ()).unwrap());

    // Assemble the file.
    let spec = FileSpec::builder_with("config.py", Python::new())
        .add_type(tb.build().unwrap())
        .add_function(greet.build().unwrap())
        .build()
        .unwrap();

    let output = spec.render(80).unwrap();
    println!("{output}");
}
