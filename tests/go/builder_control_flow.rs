use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::go_lang::GoLang;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

#[test]
fn test_control_flow() {
    let mut b = CodeBlock::<GoLang>::builder();
    b.add("func classify(x int) string {", ());
    b.add_line();
    b.add("%>", ());
    b.begin_control_flow("if x > 0", ());
    b.add_statement("return \"positive\"", ());
    b.end_control_flow();
    b.begin_control_flow("if x < 0", ());
    b.add_statement("return \"negative\"", ());
    b.end_control_flow();
    b.add_statement("return \"zero\"", ());
    b.add("%<", ());
    b.add("}", ());
    b.add_line();
    let block = b.build().unwrap();

    let mut fb = FileSpec::builder_with("classify.go", GoLang::new());
    fb.header(CodeBlock::<GoLang>::of("package main", ()).unwrap());
    fb.add_code(block);
    let file = fb.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("go/control_flow.go", &output);
}
