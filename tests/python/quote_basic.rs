use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::python::Python;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock<Python>) -> String {
    let mut fb = FileSpec::<Python>::builder("test.py");
    fb.add_code(block.clone());
    fb.build().unwrap().render(80).unwrap()
}

#[test]
fn test_basic() {
    let block = sigil_quote!(Python {
        name = $S("Alice");
        age = 30;
        print(name, age);
    })
    .unwrap();
    golden::assert_golden("python/macro_basic.py", &render(&block));
}
