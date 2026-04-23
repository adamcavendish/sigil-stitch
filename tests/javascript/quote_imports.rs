use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::javascript::JavaScript;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

fn render(block: &CodeBlock<JavaScript>) -> String {
    let mut fb = FileSpec::builder_with("test.js", JavaScript::new());
    fb.add_code(block.clone());
    fb.build().unwrap().render(80).unwrap()
}

#[test]
fn test_imports() {
    let fs_type = TypeName::<JavaScript>::importable("fs", "readFileSync");
    let path_type = TypeName::<JavaScript>::importable("path", "join");
    let block = sigil_quote!(JavaScript {
        const data = $T(fs_type)($S("input.txt"));
        const full = $T(path_type)($S("dir"), $S("file"));
    })
    .unwrap();
    golden::assert_golden("javascript/macro_imports.js", &render(&block));
}
