use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::java_lang::JavaLang;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

fn render(block: &CodeBlock<JavaLang>) -> String {
    let mut fb = FileSpec::builder_with("Test.java", JavaLang::new());
    fb.add_code(block.clone());
    fb.build().unwrap().render(80).unwrap()
}

#[test]
fn test_imports() {
    let list_type = TypeName::<JavaLang>::importable("java.util", "List");
    let map_type = TypeName::<JavaLang>::importable("java.util", "Map");
    let block = sigil_quote!(JavaLang {
        $T(list_type) items = new ArrayList<>();
        $T(map_type) lookup = new HashMap<>();
    })
    .unwrap();
    golden::assert_golden("java/macro_imports.java", &render(&block));
}
