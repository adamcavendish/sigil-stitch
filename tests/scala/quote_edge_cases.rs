use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::scala::Scala;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder_with("test.scala", Scala::new())
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_pattern_match() {
    let block = sigil_quote!(Scala {
        val result = x match {
            case(1) => "one"
            case(_) => "other"
        }
    })
    .unwrap();
    golden::assert_golden("scala/quote_pattern_match.scala", &render(&block));
}
