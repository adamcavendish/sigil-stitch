use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::java_lang::JavaLang;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder_with("Test.java", JavaLang::new())
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_ternary() {
    let block = sigil_quote!(JavaLang {
        String result = x != null ? x.toString() : "default";
        int value = flag ? 1 : 0;
    })
    .unwrap();
    golden::assert_golden("java/quote_ternary.java", &render(&block));
}
