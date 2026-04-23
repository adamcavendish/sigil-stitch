use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::rust_lang::RustLang;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

fn render(block: &CodeBlock<RustLang>) -> String {
    let mut fb = FileSpec::builder_with("test.rs", RustLang::new());
    fb.add_code(block.clone());
    fb.build().unwrap().render(80).unwrap()
}

#[test]
fn test_imports() {
    let hashmap = TypeName::<RustLang>::importable("std::collections", "HashMap");
    let vec_deque = TypeName::<RustLang>::importable("std::collections", "VecDeque");
    let block = sigil_quote!(RustLang {
        let map: $T(hashmap.clone()) = $T(hashmap)::new();
        let deque: $T(vec_deque.clone()) = $T(vec_deque)::new();
    })
    .unwrap();
    golden::assert_golden("rust/macro_imports.rs", &render(&block));
}
