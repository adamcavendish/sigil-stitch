use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder("test.swift")
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_imports() {
    let foundation = TypeName::importable("Foundation", "URL");
    let uikit = TypeName::importable("UIKit", "UIView");
    let block = sigil_quote!(Swift {
        let url: $T(foundation.clone()) = $T(foundation)(string: $S("https://example.com"));
        let view: $T(uikit.clone()) = $T(uikit)();
    })
    .unwrap();
    golden::assert_golden("swift/macro_imports.swift", &render(&block));
}
