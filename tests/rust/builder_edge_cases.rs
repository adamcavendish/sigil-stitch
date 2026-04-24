use super::golden;

use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::rust_lang::RustLang;
use sigil_stitch::spec::field_spec::FieldSpec;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::modifiers::{TypeKind, Visibility};
use sigil_stitch::spec::type_spec::TypeSpec;
use sigil_stitch::type_name::TypeName;

#[test]
fn test_optional_field() {
    let output = FileSpec::builder_with("config.rs", RustLang::new())
        .add_type(
            TypeSpec::builder("Config", TypeKind::Struct)
                .visibility(Visibility::Public)
                .add_field(
                    FieldSpec::builder("name", TypeName::primitive("String"))
                        .build()
                        .unwrap(),
                )
                .add_field(
                    FieldSpec::builder("description", TypeName::primitive("String"))
                        .is_optional()
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .build()
        .unwrap()
        .render(80)
        .unwrap();

    golden::assert_golden("rust/optional_field.rs", &output);
}

#[test]
fn test_derive_annotation() {
    let derive = CodeBlock::of("#[derive(Debug, Clone, PartialEq)]", ()).unwrap();
    let tb = TypeSpec::builder("Point", TypeKind::Struct)
        .visibility(Visibility::Public)
        .annotation(derive)
        .add_field(
            FieldSpec::builder("x", TypeName::primitive("f64"))
                .build()
                .unwrap(),
        )
        .add_field(
            FieldSpec::builder("y", TypeName::primitive("f64"))
                .build()
                .unwrap(),
        );

    let output = FileSpec::builder_with("point.rs", RustLang::new())
        .add_type(tb.build().unwrap())
        .build()
        .unwrap()
        .render(80)
        .unwrap();

    golden::assert_golden("rust/derive_annotation.rs", &output);
}
