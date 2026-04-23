use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::kotlin::Kotlin;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

fn render(block: &CodeBlock<Kotlin>) -> String {
    let mut fb = FileSpec::<Kotlin>::builder("test.kt");
    fb.add_code(block.clone());
    fb.build().unwrap().render(80).unwrap()
}

#[test]
fn test_imports() {
    let list_of = TypeName::<Kotlin>::importable("kotlin.collections", "listOf");
    let block = sigil_quote!(Kotlin {
        val items = $T(list_of)(1, 2, 3);
    })
    .unwrap();
    golden::assert_golden("kotlin/macro_imports.kt", &render(&block));
}
