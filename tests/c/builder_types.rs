use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::c_lang::CLang;
use sigil_stitch::spec::enum_variant_spec::EnumVariantSpec;
use sigil_stitch::spec::field_spec::FieldSpec;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::modifiers::TypeKind;
use sigil_stitch::spec::type_spec::TypeSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_struct_with_fields() {
    let ts = TypeSpec::builder("Config", TypeKind::Struct)
        .doc("Application configuration.")
        .add_field(
            FieldSpec::builder("timeout", TypeName::primitive("int"))
                .build()
                .unwrap(),
        )
        .add_field(
            FieldSpec::builder("name", TypeName::primitive("char*"))
                .build()
                .unwrap(),
        )
        .add_field(
            FieldSpec::builder("verbose", TypeName::primitive("int"))
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let file = FileSpec::builder_with("config.h", CLang::header())
        .header(CodeBlock::of("#pragma once", ()).unwrap())
        .add_type(ts)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("c/struct_with_fields.c", &output);
}

#[test]
fn test_enum() {
    let ts = TypeSpec::builder("Direction", TypeKind::Enum)
        .doc("Cardinal directions.")
        .add_variant(EnumVariantSpec::new("UP").unwrap())
        .add_variant(EnumVariantSpec::new("DOWN").unwrap())
        .add_variant(EnumVariantSpec::new("LEFT").unwrap())
        .add_variant(EnumVariantSpec::new("RIGHT").unwrap())
        .build()
        .unwrap();

    let file = FileSpec::builder_with("direction.h", CLang::header())
        .add_type(ts)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("c/enum.c", &output);
}
