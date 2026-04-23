use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::javascript::JavaScript;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock<JavaScript>) -> String {
    let mut fb = FileSpec::builder_with("test.js", JavaScript::new());
    fb.add_code(block.clone());
    fb.build().unwrap().render(80).unwrap()
}

#[test]
fn test_basic() {
    let block = sigil_quote!(JavaScript {
        const name = $S("Alice");
        const age = $L("30");
        console.log(name, age);
    })
    .unwrap();
    golden::assert_golden("javascript/macro_basic.js", &render(&block));
}
