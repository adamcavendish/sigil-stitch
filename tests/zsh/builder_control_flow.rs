use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

#[test]
fn test_if_then_fi() {
    let mut b = CodeBlock::builder();
    b.add("if [[ -f \"$1\" ]]; then\n", ());
    b.add("%>", ());
    b.add_statement("echo \"file exists\"", ());
    b.add("%<", ());
    b.add("else\n", ());
    b.add("%>", ());
    b.add_statement("echo \"file not found\"", ());
    b.add("%<", ());
    b.add("fi\n", ());
    let block = b.build().unwrap();

    let file = FileSpec::builder("check.zsh")
        .add_code(block)
        .build()
        .unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("zsh/if_then_fi.zsh", &output);
}
