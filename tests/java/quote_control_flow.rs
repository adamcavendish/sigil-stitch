use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::java_lang::JavaLang;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock<JavaLang>) -> String {
    let mut fb = FileSpec::builder_with("Test.java", JavaLang::new());
    fb.add_code(block.clone());
    fb.build().unwrap().render(80).unwrap()
}

#[test]
fn test_control_flow() {
    let block = sigil_quote!(JavaLang {
        if(x > 0) {
            return $S("positive");
        } else {
            return $S("negative");
        }
    })
    .unwrap();
    golden::assert_golden("java/macro_control_flow.java", &render(&block));
}
