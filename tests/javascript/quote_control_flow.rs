use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::javascript::JavaScript;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder_with("test.js", JavaScript::new())
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_control_flow() {
    let block = sigil_quote!(JavaScript {
        if(x > 0) {
            return $S("positive");
        } else if(x < 0) {
            return $S("negative");
        } else {
            return $S("zero");
        }
    })
    .unwrap();
    golden::assert_golden("javascript/macro_control_flow.js", &render(&block));
}
