use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder("test.kt")
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_imports() {
    let list_of = TypeName::importable("kotlin.collections", "listOf");
    let block = sigil_quote!(Kotlin {
        val items = $T(list_of)(1, 2, 3);
    })
    .unwrap();
    golden::assert_golden("kotlin/macro_imports.kt", &render(&block));
}
