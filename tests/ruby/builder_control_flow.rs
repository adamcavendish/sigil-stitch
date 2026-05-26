use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::ruby::Ruby;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

#[test]
fn test_if_else() {
    let mut cb = CodeBlock::builder();
    cb.begin_control_flow("if x > 0", ());
    cb.add_statement("'positive'", ());
    cb.next_control_flow("elsif x < 0", ());
    cb.add_statement("'negative'", ());
    cb.next_control_flow("else", ());
    cb.add_statement("'zero'", ());
    cb.end_control_flow();

    let file = FileSpec::builder_with("test.rb", Ruby::new())
        .add_code(cb.build().unwrap())
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ruby/builder_if_else.rb", &output);
}

#[test]
fn test_unless() {
    let mut cb = CodeBlock::builder();
    cb.begin_control_flow("unless x > 0", ());
    cb.add_statement("'not positive'", ());
    cb.next_control_flow("else", ());
    cb.add_statement("'positive'", ());
    cb.end_control_flow();

    let file = FileSpec::builder_with("test.rb", Ruby::new())
        .add_code(cb.build().unwrap())
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ruby/builder_unless.rb", &output);
}

#[test]
fn test_while_loop() {
    let mut cb = CodeBlock::builder();
    cb.begin_control_flow("while x > 0", ());
    cb.add_statement("x -= 1", ());
    cb.end_control_flow();

    let file = FileSpec::builder_with("test.rb", Ruby::new())
        .add_code(cb.build().unwrap())
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ruby/builder_while.rb", &output);
}

#[test]
fn test_begin_rescue() {
    let mut cb = CodeBlock::builder();
    cb.begin_control_flow("begin", ());
    cb.add_statement("do_something", ());
    cb.next_control_flow("rescue => e", ());
    cb.add_statement("handle_error(e)", ());
    cb.next_control_flow("ensure", ());
    cb.add_statement("cleanup", ());
    cb.end_control_flow();

    let file = FileSpec::builder_with("test.rb", Ruby::new())
        .add_code(cb.build().unwrap())
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ruby/builder_begin_rescue.rb", &output);
}
