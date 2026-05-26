use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::ruby::Ruby;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_import() {
    let json = TypeName::importable("json", "JSON");
    let net = TypeName::importable("net/http", "Net");

    let mut b = CodeBlock::builder();
    b.add_statement("# %T %T", (json, net));
    let block = b.build().unwrap();

    let file = FileSpec::builder_with("test.rb", Ruby::new())
        .add_code(block)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ruby/builder_import.rb", &output);
}

#[test]
fn test_require_gem() {
    let mut b = CodeBlock::builder();
    b.add_statement("require 'json'", ());
    b.add_statement("require 'net/http'", ());
    b.add_statement("require_relative 'lib/helpers'", ());

    let file = FileSpec::builder_with("test.rb", Ruby::new())
        .add_code(b.build().unwrap())
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ruby/builder_require.rb", &output);
}
