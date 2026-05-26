use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::ruby::Ruby;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

#[test]
fn test_hash_constructor() {
    let mut b = CodeBlock::builder();
    b.add("config = {", ());
    b.add_line();
    b.add("%>", ());
    b.add_statement("host: 'localhost'", ());
    b.add_statement("port: 8080", ());
    b.add_statement("debug: true", ());
    b.add("%<", ());
    b.add("}", ());
    b.add_line();

    let file = FileSpec::builder_with("test.rb", Ruby::new())
        .add_code(b.build().unwrap())
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ruby/builder_hash.rb", &output);
}

#[test]
fn test_array_literal() {
    let mut b = CodeBlock::builder();
    b.add_statement("items = [1, 2, 3]", ());
    b.add_statement("names = ['alice', 'bob', 'carol']", ());

    let file = FileSpec::builder_with("test.rb", Ruby::new())
        .add_code(b.build().unwrap())
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ruby/builder_array.rb", &output);
}

#[test]
fn test_block_with_do() {
    let mut b = CodeBlock::builder();
    b.add("items.each do |item|", ());
    b.add_line();
    b.add("%>", ());
    b.add_statement("puts item", ());
    b.add("%<", ());
    b.add("end", ());
    b.add_line();

    let file = FileSpec::builder_with("test.rb", Ruby::new())
        .add_code(b.build().unwrap())
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ruby/builder_block.rb", &output);
}

#[test]
fn test_case_when() {
    let mut b = CodeBlock::builder();
    b.add("case x", ());
    b.add_line();
    b.add("%>", ());
    b.add("when 0", ());
    b.add_line();
    b.add("%>", ());
    b.add_statement("'zero'", ());
    b.add("%<", ());
    b.add("when 1", ());
    b.add_line();
    b.add("%>", ());
    b.add_statement("'one'", ());
    b.add("%<", ());
    b.add("else", ());
    b.add_line();
    b.add("%>", ());
    b.add_statement("'many'", ());
    b.add("%<", ());
    b.add("end", ());
    b.add_line();
    b.add("%<", ());
    b.add("end", ());
    b.add_line();

    let file = FileSpec::builder_with("test.rb", Ruby::new())
        .add_code(b.build().unwrap())
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ruby/builder_case.rb", &output);
}
