use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::c_lang::CLang;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

fn render(block: &CodeBlock<CLang>) -> String {
    let mut fb = FileSpec::builder_with("test.c", CLang::new());
    fb.add_code(block.clone());
    fb.build().unwrap().render(80).unwrap()
}

#[test]
fn test_imports() {
    let stdio = TypeName::<CLang>::importable("stdio.h", "printf");
    let stdlib = TypeName::<CLang>::importable("stdlib.h", "malloc");
    let block = sigil_quote!(CLang {
        $T(stdio)($S("hello"));
        void* p = $T(stdlib)(sizeof(int));
    })
    .unwrap();
    golden::assert_golden("c/macro_imports.c", &render(&block));
}
