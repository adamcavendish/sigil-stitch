use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::dart::DartLang;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder_with("test.dart", DartLang::new())
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_cascade() {
    let block = sigil_quote!(DartLang {
        final builder = StringBuffer()
            ..write("hello")
            ..write(" world");
    })
    .unwrap();
    golden::assert_golden("dart/quote_cascade.dart", &render(&block));
}
