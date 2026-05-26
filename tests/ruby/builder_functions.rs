use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::CodeLang;
use sigil_stitch::lang::ruby::Ruby;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

#[test]
fn test_method() {
    let ruby = Ruby::new();
    let doc = ruby.render_doc_comment(&["Add returns the sum of two integers."]);

    let mut b = CodeBlock::builder();
    b.add("%L", doc);
    b.add_line();
    b.add("def add(a, b)", ());
    b.add_line();
    b.add("%>", ());
    b.add_statement("a + b", ());
    b.add("%<", ());
    b.add("end", ());
    b.add_line();

    let file = FileSpec::builder_with("math.rb", Ruby::new())
        .add_code(b.build().unwrap())
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ruby/method.rb", &output);
}

#[test]
fn test_class_method() {
    let ruby = Ruby::new();
    let doc = ruby.render_doc_comment(&["Create returns a new instance."]);

    let mut b = CodeBlock::builder();
    b.add("%L", doc);
    b.add_line();
    b.add("def self.create(name)", ());
    b.add_line();
    b.add("%>", ());
    b.add_statement("new(name: name)", ());
    b.add("%<", ());
    b.add("end", ());
    b.add_line();

    let file = FileSpec::builder_with("factory.rb", Ruby::new())
        .add_code(b.build().unwrap())
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ruby/class_method.rb", &output);
}

#[test]
fn test_control_flow_method() {
    let mut b = CodeBlock::builder();
    b.add("def describe(x)", ());
    b.add_line();
    b.add("%>", ());
    b.add("if x > 0", ());
    b.add_line();
    b.add("%>", ());
    b.add_statement("\"positive\"", ());
    b.add("%<", ());
    b.add("else", ());
    b.add_line();
    b.add("%>", ());
    b.add_statement("\"not positive\"", ());
    b.add("%<", ());
    b.add("end", ());
    b.add_line();
    b.add("%<", ());
    b.add("end", ());
    b.add_line();

    let file = FileSpec::builder_with("describe.rb", Ruby::new())
        .add_code(b.build().unwrap())
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ruby/builder_control_flow.rb", &output);
}
