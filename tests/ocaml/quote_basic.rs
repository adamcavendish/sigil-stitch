use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::ocaml::OCaml;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock<OCaml>) -> String {
    let mut fb = FileSpec::builder_with("test.ml", OCaml::new());
    fb.add_code(block.clone());
    fb.build().unwrap().render(80).unwrap()
}

#[test]
fn test_basic() {
    let block = sigil_quote!(OCaml {
        let x = 42;
        let name = $S("Alice");
    })
    .unwrap();
    golden::assert_golden("ocaml/macro_basic.ml", &render(&block));
}
