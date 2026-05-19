//! Tests for `$V` verbatim string literal in shell.

use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder("test.bash")
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn verbatim_preserves_dollar() {
    let block = sigil_quote!(Bash {
        echo $V("$HOME/.config")
    })
    .unwrap();
    let output = render(&block);
    assert!(
        output.contains("\"$HOME/.config\""),
        "Expected verbatim string with preserved $, got:\n{output}"
    );
}

#[test]
fn verbatim_preserves_command_substitution() {
    let block = sigil_quote!(Bash {
        local date = $V("$(date +%Y-%m-%d)")
    })
    .unwrap();
    let output = render(&block);
    assert!(
        output.contains("\"$(date +%Y-%m-%d)\""),
        "Expected verbatim string with command sub, got:\n{output}"
    );
}

#[test]
fn verbatim_escapes_backslash_and_quote() {
    let block = sigil_quote!(Bash {
        echo $V("path\\to\"file")
    })
    .unwrap();
    let output = render(&block);
    assert!(
        output.contains("\"path\\\\to\\\"file\""),
        "Expected escaped backslash and quote, got:\n{output}"
    );
}

#[test]
fn verbatim_vs_string_lit_comparison() {
    let block = sigil_quote!(Bash {
        echo $S("$HOME")
        echo $V("$HOME")
    })
    .unwrap();
    let output = render(&block);
    assert!(
        output.contains("\"\\$HOME\""),
        "Expected $S to escape dollar, got:\n{output}"
    );
    assert!(
        output.contains("\"$HOME\""),
        "Expected $V to preserve dollar, got:\n{output}"
    );
}
