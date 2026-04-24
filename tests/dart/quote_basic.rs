use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder("test.dart")
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_basic() {
    let block = sigil_quote!(DartLang {
        final name = $S("Alice");
        final age = 30;
        print(name);
    })
    .unwrap();
    golden::assert_golden("dart/macro_basic.dart", &render(&block));
}
