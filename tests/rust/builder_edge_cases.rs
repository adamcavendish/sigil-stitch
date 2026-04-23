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
    let mut tb = TypeSpec::<RustLang>::builder("Config", TypeKind::Struct);
    tb.visibility(Visibility::Public);

    tb.add_field(
        FieldSpec::builder("name", TypeName::<RustLang>::primitive("String"))
            .build()
            .unwrap(),
    );

    let mut opt = FieldSpec::builder("description", TypeName::<RustLang>::primitive("String"));
    opt.is_optional();
    tb.add_field(opt.build().unwrap());

    let mut file = FileSpec::builder_with("config.rs", RustLang::new());
    file.add_type(tb.build().unwrap());
    let output = file.build().unwrap().render(80).unwrap();

    golden::assert_golden("rust/optional_field.rs", &output);
}

#[test]
fn test_derive_annotation() {
    let mut tb = TypeSpec::<RustLang>::builder("Point", TypeKind::Struct);
    tb.visibility(Visibility::Public);

    let derive = CodeBlock::<RustLang>::of("#[derive(Debug, Clone, PartialEq)]", ()).unwrap();
    tb.annotation(derive);

    tb.add_field(
        FieldSpec::builder("x", TypeName::<RustLang>::primitive("f64"))
            .build()
            .unwrap(),
    );
    tb.add_field(
        FieldSpec::builder("y", TypeName::<RustLang>::primitive("f64"))
            .build()
            .unwrap(),
    );

    let mut file = FileSpec::builder_with("point.rs", RustLang::new());
    file.add_type(tb.build().unwrap());
    let output = file.build().unwrap().render(80).unwrap();

    golden::assert_golden("rust/derive_annotation.rs", &output);
}
