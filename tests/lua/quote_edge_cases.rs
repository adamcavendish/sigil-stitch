use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::lua::Lua;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

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
fn test_string_concat() {
    let block = sigil_quote!(Lua {
        local greeting = "Hello, "..name.."!"
        local path = dir.."/"..file
    })
    .unwrap();
    golden::assert_golden("lua/quote_string_concat.lua", &render(&block));
}

#[test]
fn test_local_function() {
    let block = sigil_quote!(Lua {
        local function add(a, b) {
            return a + b
        }
        local result = add(1, 2)
    })
    .unwrap();
    golden::assert_golden("lua/quote_local_function.lua", &render(&block));
}
