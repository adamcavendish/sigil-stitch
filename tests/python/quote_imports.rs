use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder("test.py")
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_imports() {
    let json_dumps = TypeName::importable("json", "dumps");
    let path_join = TypeName::importable("os.path", "join");
    let block = sigil_quote!(Python {
        data = $T(json_dumps)(obj);
        full_path = $T(path_join)(base, name);
    })
    .unwrap();
    golden::assert_golden("python/quote_imports.py", &render(&block));
}
