use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::go_lang::GoLang;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock<GoLang>) -> String {
    let mut fb = FileSpec::builder_with("test.go", GoLang::new());
    fb.add_code(block.clone());
    fb.build().unwrap().render(80).unwrap()
}

#[test]
fn test_indent() {
    let block = sigil_quote!(GoLang {
        namespace Foo {
        $>
        North = iota;
        East;
        South;
        West;
        $<
        }
    })
    .unwrap();
    golden::assert_golden("go/macro_indent.go", &render(&block));
}
