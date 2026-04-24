use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::go_lang::GoLang;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder_with("test.go", GoLang::new())
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_basic() {
    let block = sigil_quote!(GoLang {
        x := 42;
        name := $S("Alice");
        fmt.Println(name, x);
    })
    .unwrap();
    golden::assert_golden("go/macro_basic.go", &render(&block));
}
