use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::kotlin::Kotlin;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock<Kotlin>) -> String {
    let mut fb = FileSpec::<Kotlin>::builder("test.kt");
    fb.add_code(block.clone());
    fb.build().unwrap().render(80).unwrap()
}

#[test]
fn test_control_flow() {
    let block = sigil_quote!(Kotlin {
        if(x > 0) {
            return $S("positive");
        } else {
            return $S("negative");
        }
    })
    .unwrap();
    golden::assert_golden("kotlin/macro_control_flow.kt", &render(&block));
}
