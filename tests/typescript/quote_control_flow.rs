use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder("test.ts")
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_control_flow() {
    let error_type = TypeName::importable_type("./errors", "NotFoundError");
    let block = sigil_quote!(TypeScript {
        if(!user) {
            throw new $T(error_type)($S("not found"));
        } else {
            return user;
        }
    })
    .unwrap();
    golden::assert_golden("typescript/macro_control_flow.ts", &render(&block));
}
