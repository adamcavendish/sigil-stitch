use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::CodeLang;
use sigil_stitch::lang::go_lang::GoLang;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

#[test]
fn test_enum() {
    // Go has no native enum syntax. The idiomatic pattern is:
    //   type Direction int
    //   const ( North Direction = iota; East; ... )
    //
    // This doesn't fit TypeSpec, so we build it as a raw CodeBlock.
    let go = GoLang::new();

    let mut cb = CodeBlock::builder();
    let doc = go.render_doc_comment(&["Direction represents a cardinal direction."]);
    cb.add("%L", doc);
    cb.add_line();
    cb.add("type Direction int", ());
    cb.add_line();
    cb.add_line();
    cb.add("const (", ());
    cb.add_line();
    cb.add("%>", ());
    cb.add("North Direction = iota", ());
    cb.add_line();
    cb.add("East", ());
    cb.add_line();
    cb.add("South", ());
    cb.add_line();
    cb.add("West", ());
    cb.add_line();
    cb.add("%<", ());
    cb.add(")", ());
    cb.add_line();
    let block = cb.build().unwrap();

    let file = FileSpec::builder_with("direction.go", GoLang::new())
        .header(CodeBlock::of("package direction", ()).unwrap())
        .add_code(block)
        .build()
        .unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("go/enum.go", &output);
}
