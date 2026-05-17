use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::swift::Swift;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder_with("test.swift", Swift::new())
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_optional_chaining() {
    let block = sigil_quote!(Swift {
        let name = user?.profile?.displayName;
        let count = items?.count ?? 0;
    })
    .unwrap();
    golden::assert_golden("swift/quote_optional_chain.swift", &render(&block));
}
