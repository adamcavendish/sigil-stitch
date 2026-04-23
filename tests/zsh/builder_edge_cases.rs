use sigil_stitch::code_block::{CodeBlock, StringLitArg};
use sigil_stitch::lang::zsh::Zsh;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

#[test]
fn test_variable_assignment() {
    let mut b = CodeBlock::<Zsh>::builder();
    b.add("NAME=%S\n", (StringLitArg("world".into()),));
    b.add("COUNT=42\n", ());
    let block = b.build().unwrap();

    let mut fb = FileSpec::<Zsh>::builder("vars.zsh");
    fb.add_code(block);
    let file = fb.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("zsh/variable_assignment.zsh", &output);
}

#[test]
fn test_percent_escaping() {
    let mut b = CodeBlock::<Zsh>::builder();
    b.add("MSG=%S\n", (StringLitArg("100% done".into()),));
    b.add("PROMPT=%S\n", (StringLitArg("%F{red}error%f".into()),));
    let block = b.build().unwrap();

    let mut fb = FileSpec::<Zsh>::builder("prompt.zsh");
    fb.add_code(block);
    let file = fb.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("zsh/percent_escaping.zsh", &output);
}
