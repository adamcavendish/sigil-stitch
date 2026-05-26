use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::CodeLang;
use sigil_stitch::lang::ruby::Ruby;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

#[test]
fn test_class() {
    let ruby = Ruby::new();
    let doc = ruby.render_doc_comment(&["Greeter says hello."]);

    let mut b = CodeBlock::builder();
    b.add("%L", doc);
    b.add_line();
    b.add("class Greeter", ());
    b.add_line();
    b.add("%>", ());
    b.add("def initialize(name)", ());
    b.add_line();
    b.add("%>", ());
    b.add_statement("@name = name", ());
    b.add("%<", ());
    b.add("end", ());
    b.add_line();
    b.add_line();
    b.add("def greet", ());
    b.add_line();
    b.add("%>", ());
    b.add_statement(r##""Hello, #{@name}""##, ());
    b.add("%<", ());
    b.add("end", ());
    b.add_line();
    b.add("%<", ());
    b.add("end", ());
    b.add_line();

    let file = FileSpec::builder_with("greeter.rb", Ruby::new())
        .add_code(b.build().unwrap())
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ruby/class.rb", &output);
}

#[test]
fn test_module() {
    let mut b = CodeBlock::builder();
    b.add("module StringHelpers", ());
    b.add_line();
    b.add("%>", ());
    b.add("def self.pluralize(word)", ());
    b.add_line();
    b.add("%>", ());
    b.add_statement(r##""#{word}s""##, ());
    b.add("%<", ());
    b.add("end", ());
    b.add_line();
    b.add("%<", ());
    b.add("end", ());
    b.add_line();

    let file = FileSpec::builder_with("helpers.rb", Ruby::new())
        .add_code(b.build().unwrap())
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ruby/module.rb", &output);
}
