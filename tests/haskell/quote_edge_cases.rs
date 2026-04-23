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
fn test_open_where() {
    let block = sigil_quote!(Haskell {
        class Functor f $open(" where") {
            fmap :: (a -> b) -> f a -> f b;
        }
    })
    .unwrap();
    golden::assert_golden("haskell/macro_open_where.hs", &render(&block));
}
