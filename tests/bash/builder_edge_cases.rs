use sigil_stitch::code_block::{CodeBlock, StringLitArg};
use sigil_stitch::lang::bash::Bash;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

#[test]
fn test_variable_assignment() {
    let mut b = CodeBlock::<Bash>::builder();
    b.add("NAME=%S\n", (StringLitArg("world".into()),));
    b.add("COUNT=42\n", ());
    b.add("READONLY_VAR=%S\n", (StringLitArg("constant".into()),));
    let block = b.build().unwrap();

    let mut fb = FileSpec::<Bash>::builder("vars.bash");
    fb.add_code(block);
    let file = fb.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("bash/variable_assignment.bash", &output);
}

#[test]
fn test_shebang() {
    let mut header_b = CodeBlock::<Bash>::builder();
    header_b.add("#!/usr/bin/env bash\n", ());
    header_b.add("set -euo pipefail", ());
    let header = header_b.build().unwrap();

    let mut body = CodeBlock::<Bash>::builder();
    body.add_statement("echo \"hello\"", ());
    let block = body.build().unwrap();

    let mut fb = FileSpec::<Bash>::builder("script.bash");
    fb.header(header);
    fb.add_code(block);
    let file = fb.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("bash/shebang.bash", &output);
}

#[test]
fn test_string_escaping() {
    let mut b = CodeBlock::<Bash>::builder();
    b.add("MSG=%S\n", (StringLitArg("hello \"world\"".into()),));
    b.add("PATH_VAR=%S\n", (StringLitArg("$HOME/bin".into()),));
    b.add("CMD=%S\n", (StringLitArg("`whoami`".into()),));
    b.add("BANG=%S\n", (StringLitArg("wow!".into()),));
    let block = b.build().unwrap();

    let mut fb = FileSpec::<Bash>::builder("escape.bash");
    fb.add_code(block);
    let file = fb.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("bash/string_escaping.bash", &output);
}
