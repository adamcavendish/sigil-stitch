use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder("test.scala")
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_control_flow() {
    let block = sigil_quote!(Scala {
        if(x > 0) {
            return $S("positive");
        } else {
            return $S("negative");
        }
    })
    .unwrap();
    golden::assert_golden("scala/macro_control_flow.scala", &render(&block));
}
