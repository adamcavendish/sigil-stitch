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
    let mut tb = TypeSpec::<CLang>::builder("Config", TypeKind::Struct);
    tb.doc("Application configuration.");
    tb.add_field(
        FieldSpec::builder("timeout", TypeName::primitive("int"))
            .build()
            .unwrap(),
    );
    tb.add_field(
        FieldSpec::builder("name", TypeName::primitive("char*"))
            .build()
            .unwrap(),
    );
    tb.add_field(
        FieldSpec::builder("verbose", TypeName::primitive("int"))
            .build()
            .unwrap(),
    );
    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("config.h", CLang::header());
    fb.header(CodeBlock::<CLang>::of("#pragma once", ()).unwrap());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("c/struct_with_fields.c", &output);
}

#[test]
fn test_enum() {
    let mut tb = TypeSpec::<CLang>::builder("Direction", TypeKind::Enum);
    tb.doc("Cardinal directions.");
    tb.add_variant(EnumVariantSpec::new("UP").unwrap());
    tb.add_variant(EnumVariantSpec::new("DOWN").unwrap());
    tb.add_variant(EnumVariantSpec::new("LEFT").unwrap());
    tb.add_variant(EnumVariantSpec::new("RIGHT").unwrap());
    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("direction.h", CLang::header());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("c/enum.c", &output);
}
