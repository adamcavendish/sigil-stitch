use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::ocaml::OCaml;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder_with("test.ml", OCaml::new())
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_imports() {
    let list_mod = TypeName::importable("List", "t");
    let block = sigil_quote!(OCaml {
        let result = $T(list_mod).map f xs
    })
    .unwrap();
    golden::assert_golden("ocaml/quote_imports.ml", &render(&block));
}
