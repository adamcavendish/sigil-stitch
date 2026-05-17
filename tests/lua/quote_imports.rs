use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::lua::Lua;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder_with("test.lua", Lua::new())
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_imports() {
    let json = TypeName::importable("dkjson", "json");
    let inspect = TypeName::importable("inspect", "inspect");
    let block = sigil_quote!(Lua {
        local data = $T(json).decode(input)
        $T(inspect)(data)
    })
    .unwrap();
    golden::assert_golden("lua/quote_imports.lua", &render(&block));
}
