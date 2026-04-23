use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::typescript::TypeScript;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

fn render(block: &CodeBlock<TypeScript>) -> String {
    let mut fb = FileSpec::<TypeScript>::builder("test.ts");
    fb.add_code(block.clone());
    fb.build().unwrap().render(80).unwrap()
}

#[test]
fn test_control_flow() {
    let error_type = TypeName::<TypeScript>::importable_type("./errors", "NotFoundError");
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
