use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::ruby::Ruby;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder_with("test.rb", Ruby::new())
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_imports() {
    let json = TypeName::importable("json", "JSON");
    let net = TypeName::importable("net/http", "Net");
    let block = sigil_quote!(Ruby {
        data = $T(json).parse(input)
        $T(net).start
    })
    .unwrap();
    golden::assert_golden("ruby/quote_imports.rb", &render(&block));
}
