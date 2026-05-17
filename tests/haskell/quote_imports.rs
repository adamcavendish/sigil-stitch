use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::haskell::Haskell;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder_with("test.hs", Haskell::new())
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_imports() {
    let map_type = TypeName::importable("Data.Map", "Map");
    let text_type = TypeName::importable("Data.Text", "Text");
    let block = sigil_quote!(Haskell {
        let users = $T(map_type).fromList [($T(text_type).pack "alice", 1)]
    })
    .unwrap();
    golden::assert_golden("haskell/quote_imports.hs", &render(&block));
}
