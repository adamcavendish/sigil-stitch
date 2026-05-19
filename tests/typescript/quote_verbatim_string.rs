//! Tests for `$V` verbatim string literal in TypeScript.

use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder("test.ts")
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn verbatim_uses_template_literal() {
    let block = sigil_quote!(TypeScript {
        const msg = $V("Hello, ${name}!")
    })
    .unwrap();
    let output = render(&block);
    assert!(
        output.contains("`Hello, ${name}!`"),
        "Expected template literal with interpolation, got:\n{output}"
    );
}

#[test]
fn verbatim_escapes_backtick() {
    let block = sigil_quote!(TypeScript {
        const msg = $V("use \\` for templates")
    })
    .unwrap();
    let output = render(&block);
    // Input: "use \` for templates" → escapes \ to \\ and ` to \`
    assert!(
        output.contains("`use \\\\\\` for templates`"),
        "Expected escaped backtick in template literal, got:\n{output}"
    );
}
