use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::rust_lang::RustLang;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock<RustLang>) -> String {
    let mut fb = FileSpec::builder_with("test.rs", RustLang::new());
    fb.add_code(block.clone());
    fb.build().unwrap().render(80).unwrap()
}

#[test]
fn test_path_separator() {
    let block = sigil_quote!(RustLang {
        let size = std::mem::size_of::<u32>();
        let x = std::cmp::max(1, 2);
    })
    .unwrap();
    golden::assert_golden("rust/macro_path_separator.rs", &render(&block));
}
