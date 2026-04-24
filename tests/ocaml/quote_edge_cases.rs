use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::ocaml::OCaml;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

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
fn test_open_struct() {
    let block = sigil_quote!(OCaml {
        module Foo $open(" = struct") {
            let x = 42;
            let name = $S("Alice");
        }
    })
    .unwrap();
    golden::assert_golden("ocaml/macro_open_struct.ml", &render(&block));
}
