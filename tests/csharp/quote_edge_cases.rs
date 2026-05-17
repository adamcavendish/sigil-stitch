use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::csharp::CSharp;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder_with("Test.cs", CSharp::new())
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_null_conditional() {
    let block = sigil_quote!(CSharp {
        var name = user?.Profile?.DisplayName;
        var count = items?.Count ?? 0;
    })
    .unwrap();
    golden::assert_golden("csharp/quote_null_conditional.cs", &render(&block));
}
