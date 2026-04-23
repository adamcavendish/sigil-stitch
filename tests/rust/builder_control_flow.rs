use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::rust_lang::RustLang;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

#[test]
fn test_control_flow() {
    let mut b = CodeBlock::<RustLang>::builder();
    b.add("pub fn classify(x: i32) -> &'static str {", ());
    b.add_line();
    b.add("%>", ());
    b.begin_control_flow("if x > 0", ());
    b.add_statement("\"positive\"", ());
    b.next_control_flow("else if x < 0", ());
    b.add_statement("\"negative\"", ());
    b.next_control_flow("else", ());
    b.add_statement("\"zero\"", ());
    b.end_control_flow();
    b.add("%<", ());
    b.add("}", ());
    b.add_line();
    let block = b.build().unwrap();

    let mut fb = FileSpec::builder_with("classify.rs", RustLang::new());
    fb.add_code(block);
    let file = fb.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("rust/control_flow.rs", &output);
}
