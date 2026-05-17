use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::kotlin::Kotlin;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder_with("test.kt", Kotlin::new())
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_safe_call() {
    let block = sigil_quote!(Kotlin {
        val name = response.body?.string();
        val length = name?.length;
    })
    .unwrap();
    golden::assert_golden("kotlin/quote_safe_call.kt", &render(&block));
}

#[test]
fn test_elvis() {
    let block = sigil_quote!(Kotlin {
        val name: String = input ?: "default";
        val count = list?.size ?: 0;
    })
    .unwrap();
    golden::assert_golden("kotlin/quote_elvis.kt", &render(&block));
}
