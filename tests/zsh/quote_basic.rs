use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder("test.zsh")
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_basic() {
    let block = sigil_quote!(Zsh {
        NAME=$S("Alice");
        AGE=30;
        echo $L("$NAME") $L("$AGE");
    })
    .unwrap();
    golden::assert_golden("zsh/macro_basic.zsh", &render(&block));
}
