use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::haskell::Haskell;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock<Haskell>) -> String {
    let mut fb = FileSpec::builder_with("test.hs", Haskell::new());
    fb.add_code(block.clone());
    fb.build().unwrap().render(80).unwrap()
}

#[test]
fn test_basic() {
    let block = sigil_quote!(Haskell {
        let x = 42;
        putStrLn $S("hello");
    })
    .unwrap();
    golden::assert_golden("haskell/macro_basic.hs", &render(&block));
}
