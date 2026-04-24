use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder("test.ts")
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_basic() {
    let block = sigil_quote!(TypeScript {
        const name = $S("Alice");
        const age = $L("30");
        console.log(name, age);
    })
    .unwrap();
    golden::assert_golden("typescript/macro_basic.ts", &render(&block));
}
