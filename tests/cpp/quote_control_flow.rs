use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::cpp_lang::CppLang;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder_with("test.cpp", CppLang::new())
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_control_flow() {
    let block = sigil_quote!(CppLang {
        if(x > 0) {
            return true;
        } else {
            return false;
        }
    })
    .unwrap();
    golden::assert_golden("cpp/macro_control_flow.cpp", &render(&block));
}
