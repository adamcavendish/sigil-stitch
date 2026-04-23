use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::zsh::Zsh;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock<Zsh>) -> String {
    let mut fb = FileSpec::<Zsh>::builder("test.zsh");
    fb.add_code(block.clone());
    fb.build().unwrap().render(80).unwrap()
}

#[test]
fn test_control_flow() {
    let block = sigil_quote!(Zsh {
        if [ $L("$x") -gt 0 ] {
            echo $S("positive");
        } else {
            echo $S("negative");
        }
    })
    .unwrap();
    golden::assert_golden("zsh/macro_control_flow.zsh", &render(&block));
}
