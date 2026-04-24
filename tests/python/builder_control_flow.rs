use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::python::Python;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

#[test]
fn test_control_flow() {
    let mut b = CodeBlock::builder();
    b.add("def classify(x: int) -> str:", ());
    b.add_line();
    b.add("%>", ());
    b.begin_control_flow("if x > 0", ());
    b.add_statement("return 'positive'", ());
    b.next_control_flow("elif x < 0", ());
    b.add_statement("return 'negative'", ());
    b.next_control_flow("else", ());
    b.add_statement("return 'zero'", ());
    b.end_control_flow();
    b.add("%<", ());
    let block = b.build().unwrap();

    let file = FileSpec::builder_with("classify.py", Python::new())
        .add_code(block)
        .build()
        .unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("python/control_flow.py", &output);
}

#[test]
fn test_class_basic() {
    let mut b = CodeBlock::builder();
    b.add("class Config:", ());
    b.add_line();
    b.add("%>", ());
    b.add_statement("name: str", ());
    b.add_statement("port: int", ());
    b.add_statement("debug: bool = False", ());
    b.add("%<", ());
    let block = b.build().unwrap();

    let file = FileSpec::builder_with("config.py", Python::new())
        .add_code(block)
        .build()
        .unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("python/class_basic.py", &output);
}
