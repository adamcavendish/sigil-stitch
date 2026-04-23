use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::dart::DartLang;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

fn render(block: &CodeBlock<DartLang>) -> String {
    let mut fb = FileSpec::<DartLang>::builder("test.dart");
    fb.add_code(block.clone());
    fb.build().unwrap().render(80).unwrap()
}

#[test]
fn test_imports() {
    let http = TypeName::<DartLang>::importable("package:http/http.dart", "Client");
    let convert = TypeName::<DartLang>::importable("dart:convert", "jsonDecode");
    let block = sigil_quote!(DartLang {
        final client = $T(http)();
        final data = $T(convert)(response.body);
    })
    .unwrap();
    golden::assert_golden("dart/macro_imports.dart", &render(&block));
}
