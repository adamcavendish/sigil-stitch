use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::swift::Swift;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock<Swift>) -> String {
    let mut fb = FileSpec::<Swift>::builder("test.swift");
    fb.add_code(block.clone());
    fb.build().unwrap().render(80).unwrap()
}

#[test]
fn test_basic() {
    let block = sigil_quote!(Swift {
        let name: String = $S("Alice");
        let age: Int = 30;
        print(name, age);
    })
    .unwrap();
    golden::assert_golden("swift/macro_basic.swift", &render(&block));
}
