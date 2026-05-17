use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::lua::Lua;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

#[test]
fn test_string_concat() {
    let mut b = CodeBlock::builder();
    b.add_statement("local greeting = \"Hello, \"..name..\"!\"", ());
    b.add_statement("local path = dir..\"/\"..file", ());
    let block = b.build().unwrap();

    let file = FileSpec::builder_with("test.lua", Lua::new())
        .add_code(block)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("lua/builder_string_concat.lua", &output);
}

#[test]
fn test_table_constructor() {
    let mut b = CodeBlock::builder();
    b.add("local config = {", ());
    b.add_line();
    b.add("%>", ());
    b.add_statement("host = \"localhost\"", ());
    b.add_statement("port = 8080", ());
    b.add_statement("[\"special-key\"] = true", ());
    b.add("%<", ());
    b.add("}", ());
    b.add_line();
    let block = b.build().unwrap();

    let file = FileSpec::builder_with("test.lua", Lua::new())
        .add_code(block)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("lua/builder_table_constructor.lua", &output);
}
