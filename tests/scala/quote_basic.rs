use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::scala::Scala;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock<Scala>) -> String {
    let mut fb = FileSpec::<Scala>::builder("test.scala");
    fb.add_code(block.clone());
    fb.build().unwrap().render(80).unwrap()
}

#[test]
fn test_basic() {
    let block = sigil_quote!(Scala {
        val name = $S("Alice");
        val age = 30;
        println(name);
    })
    .unwrap();
    golden::assert_golden("scala/macro_basic.scala", &render(&block));
}
