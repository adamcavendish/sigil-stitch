use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::rust_lang::RustLang;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder_with("test.rs", RustLang::new())
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_control_flow() {
    let block = sigil_quote!(RustLang {
        if x > 0 {
            return Ok(x);
        } else {
            return Err($S("negative"));
        }
    })
    .unwrap();
    golden::assert_golden("rust/macro_control_flow.rs", &render(&block));
}
