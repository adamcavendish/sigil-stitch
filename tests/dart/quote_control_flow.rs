use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::dart::DartLang;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock<DartLang>) -> String {
    let mut fb = FileSpec::<DartLang>::builder("test.dart");
    fb.add_code(block.clone());
    fb.build().unwrap().render(80).unwrap()
}

#[test]
fn test_control_flow() {
    let block = sigil_quote!(DartLang {
        if(x > 0) {
            return true;
        } else {
            return false;
        }
    })
    .unwrap();
    golden::assert_golden("dart/macro_control_flow.dart", &render(&block));
}
