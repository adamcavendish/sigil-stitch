use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::CodeLang;
use sigil_stitch::lang::ocaml::OCaml;
use sigil_stitch::spec::field_spec::FieldSpec;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::modifiers::TypeKind;
use sigil_stitch::spec::type_spec::TypeSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_record_type() {
    let mut tb = TypeSpec::<OCaml>::builder("person", TypeKind::Struct);
    tb.doc("A person record.");

    tb.add_field(
        FieldSpec::builder("name", TypeName::primitive("string"))
            .build()
            .unwrap(),
    );
    tb.add_field(
        FieldSpec::builder("age", TypeName::primitive("int"))
            .build()
            .unwrap(),
    );
    tb.add_field(
        FieldSpec::builder("email", TypeName::primitive("string"))
            .build()
            .unwrap(),
    );

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("person.ml", OCaml::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ocaml/record_type.ml", &output);
}

#[test]
fn test_type_alias() {
    let mut tb = TypeSpec::<OCaml>::builder("string_list", TypeKind::TypeAlias);
    tb.extends(TypeName::primitive("string list"));

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("aliases.ml", OCaml::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ocaml/type_alias.ml", &output);
}

#[test]
fn test_module_type() {
    let ml = OCaml::new();

    let mut outer = CodeBlock::<OCaml>::builder();
    let doc = ml.render_doc_comment(&["Comparable interface."]);
    outer.add("%L", doc);
    outer.add_line();

    let mut inner = CodeBlock::<OCaml>::builder();
    inner.add_statement("val compare : t -> t -> int", ());
    let body = inner.build().unwrap();

    let module = OCaml::module_sig_block("COMPARABLE", body).unwrap();
    outer.add_code(module);
    let block = outer.build().unwrap();

    let mut fb = FileSpec::builder_with("comparable.ml", OCaml::new());
    fb.add_code(block);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ocaml/module_type.ml", &output);
}

#[test]
fn test_module_block() {
    let mut inner = CodeBlock::<OCaml>::builder();
    inner.add_statement("let greeting = \"hello\"", ());
    inner.add_statement("let farewell = \"goodbye\"", ());
    let body = inner.build().unwrap();

    let module = OCaml::module_block("MyModule", body).unwrap();

    let mut fb = FileSpec::builder_with("mymodule.ml", OCaml::new());
    fb.add_code(module);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ocaml/module_block.ml", &output);
}

#[test]
fn test_module_sig_block() {
    let mut inner = CodeBlock::<OCaml>::builder();
    inner.add_statement("val greeting : string", ());
    inner.add_statement("val farewell : string", ());
    let body = inner.build().unwrap();

    let module = OCaml::module_sig_block("MY_SIG", body).unwrap();

    let mut fb = FileSpec::builder_with("my_sig.ml", OCaml::new());
    fb.add_code(module);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ocaml/module_sig.ml", &output);
}
