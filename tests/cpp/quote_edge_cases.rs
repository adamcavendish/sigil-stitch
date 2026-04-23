use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::cpp_lang::CppLang;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

fn render(block: &CodeBlock<CppLang>) -> String {
    let mut fb = FileSpec::builder_with("test.cpp", CppLang::new());
    fb.add_code(block.clone());
    fb.build().unwrap().render(80).unwrap()
}

#[test]
fn test_includes() {
    let iostream = TypeName::<CppLang>::importable("iostream", "cout");
    let memory = TypeName::<CppLang>::importable("memory", "unique_ptr");
    let block = sigil_quote!(CppLang {
        auto ptr = std::make_unique<int>(42);
        $T(iostream) << $T(memory)(ptr.get()) << std::endl;
    })
    .unwrap();
    golden::assert_golden("cpp/macro_includes.cpp", &render(&block));
}
