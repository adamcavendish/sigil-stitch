use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::bash::Bash;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

#[test]
fn test_if_then_fi() {
    let mut b = CodeBlock::<Bash>::builder();
    b.add("if [ -f \"$1\" ]; then\n", ());
    b.add("%>", ());
    b.add_statement("echo \"file exists\"", ());
    b.add("%<", ());
    b.add("else\n", ());
    b.add("%>", ());
    b.add_statement("echo \"file not found\"", ());
    b.add("%<", ());
    b.add("fi\n", ());
    let block = b.build().unwrap();

    let mut fb = FileSpec::<Bash>::builder("check.bash");
    fb.add_code(block);
    let file = fb.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("bash/if_then_fi.bash", &output);
}

#[test]
fn test_for_loop() {
    let mut b = CodeBlock::<Bash>::builder();
    b.add("for f in *.txt; do\n", ());
    b.add("%>", ());
    b.add_statement("echo \"Processing $f\"", ());
    b.add("%<", ());
    b.add("done\n", ());
    let block = b.build().unwrap();

    let mut fb = FileSpec::<Bash>::builder("loop.bash");
    fb.add_code(block);
    let file = fb.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("bash/for_loop.bash", &output);
}

#[test]
fn test_while_loop() {
    let mut b = CodeBlock::<Bash>::builder();
    b.add("while read -r line; do\n", ());
    b.add("%>", ());
    b.add_statement("echo \"$line\"", ());
    b.add("%<", ());
    b.add("done\n", ());
    let block = b.build().unwrap();

    let mut fb = FileSpec::<Bash>::builder("reader.bash");
    fb.add_code(block);
    let file = fb.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("bash/while_loop.bash", &output);
}

#[test]
fn test_case_esac() {
    let mut b = CodeBlock::<Bash>::builder();
    b.add("case \"$1\" in\n", ());
    b.add("%>", ());
    b.add("start)\n", ());
    b.add("%>", ());
    b.add_statement("start_service", ());
    b.add("%<", ());
    b.add(";;\n", ());
    b.add("stop)\n", ());
    b.add("%>", ());
    b.add_statement("stop_service", ());
    b.add("%<", ());
    b.add(";;\n", ());
    b.add("*)\n", ());
    b.add("%>", ());
    b.add_statement("echo \"Usage: $0 {start|stop}\"", ());
    b.add("%<", ());
    b.add(";;\n", ());
    b.add("%<", ());
    b.add("esac\n", ());
    let block = b.build().unwrap();

    let mut fb = FileSpec::<Bash>::builder("service.bash");
    fb.add_code(block);
    let file = fb.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("bash/case_esac.bash", &output);
}

#[test]
fn test_nested_control_flow() {
    let mut b = CodeBlock::<Bash>::builder();
    b.add("if [ -d \"$1\" ]; then\n", ());
    b.add("%>", ());
    b.add("for f in \"$1\"/*; do\n", ());
    b.add("%>", ());
    b.add("if [ -f \"$f\" ]; then\n", ());
    b.add("%>", ());
    b.add_statement("process \"$f\"", ());
    b.add("%<", ());
    b.add("fi\n", ());
    b.add("%<", ());
    b.add("done\n", ());
    b.add("%<", ());
    b.add("fi\n", ());
    let block = b.build().unwrap();

    let mut fb = FileSpec::<Bash>::builder("nested.bash");
    fb.add_code(block);
    let file = fb.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("bash/nested_control_flow.bash", &output);
}
