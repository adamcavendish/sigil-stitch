use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::typescript::TypeScript;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock<TypeScript>) -> String {
    let mut fb = FileSpec::<TypeScript>::builder("test.ts");
    fb.add_code(block.clone());
    fb.build().unwrap().render(80).unwrap()
}

#[test]
fn test_object_literal() {
    let block = sigil_quote!(TypeScript {
        const config = { timeout: 5000, retries: 3 };
        const nested = { a: 1, b: { c: 2 } };
    })
    .unwrap();
    golden::assert_golden("typescript/macro_object_literal.ts", &render(&block));
}
