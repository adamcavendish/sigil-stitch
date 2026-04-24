use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::ocaml::OCaml;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_function_with_imports() {
    let list_mod = TypeName::importable("List", "t");

    let mut b = CodeBlock::builder();
    b.add_statement("let result = %T.map f xs", (list_mod,));
    let block = b.build().unwrap();

    let file = FileSpec::builder_with("app.ml", OCaml::new())
        .add_code(block)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ocaml/function_with_imports.ml", &output);
}
