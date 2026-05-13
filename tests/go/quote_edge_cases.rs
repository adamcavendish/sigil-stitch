use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::go_lang::GoLang;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder_with("test.go", GoLang::new())
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_indent() {
    let block = sigil_quote!(GoLang {
        namespace Foo {
        $>
        North = iota;
        East;
        South;
        West;
        $<
        }
    })
    .unwrap();
    golden::assert_golden("go/macro_indent.go", &render(&block));
}

#[test]
fn test_name_escape_in_macro() {
    let name = "type";
    let block = sigil_quote!(GoLang {
        var $N(name) string
    })
    .unwrap();

    let output = render(&block);
    assert!(
        output.contains("var type_ string"),
        "Expected 'var type_ string', got: {output}"
    );
}

#[test]
fn test_name_escape_multiple_keywords_in_macro() {
    let pkg = "package";
    let ret = "return";
    let block = sigil_quote!(GoLang {
        $N(pkg) = $N(ret)
    })
    .unwrap();

    let output = render(&block);
    assert!(output.contains("package_"), "Expected 'package_': {output}");
    assert!(output.contains("return_"), "Expected 'return_': {output}");
}

#[test]
fn test_name_no_escape_in_macro() {
    let name = "myHandler";
    let block = sigil_quote!(GoLang {
        func $N(name)()
    })
    .unwrap();

    let output = render(&block);
    assert!(
        output.contains("func myHandler()"),
        "Expected 'func myHandler()', got: {output}"
    );
}
