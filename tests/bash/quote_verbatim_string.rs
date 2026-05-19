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
fn verbatim_preserves_braced_expansion() {
    let block = sigil_quote!(Bash {
        local fallback = $V("${CONFIG_DIR:-$HOME/.config}")
    })
    .unwrap();
    let output = render(&block);
    assert!(
        output.contains("\"${CONFIG_DIR:-$HOME/.config}\""),
        "Expected braced default expansion, got:\n{output}"
    );
}

#[test]
fn verbatim_preserves_arithmetic() {
    let block = sigil_quote!(Bash {
        echo $V("Result: $((count + 1)) items processed")
    })
    .unwrap();
    let output = render(&block);
    assert!(
        output.contains("\"Result: $((count + 1)) items processed\""),
        "Expected arithmetic expansion, got:\n{output}"
    );
}

#[test]
fn verbatim_preserves_special_vars() {
    let block = sigil_quote!(Bash {
        echo $V("PID=$$ args=$# status=$? all=$@")
    })
    .unwrap();
    let output = render(&block);
    assert!(
        output.contains("\"PID=$$ args=$# status=$? all=$@\""),
        "Expected special variables, got:\n{output}"
    );
}

#[test]
fn verbatim_preserves_array_expansion() {
    let block = sigil_quote!(Bash {
        echo $V("${files[@]}")
    })
    .unwrap();
    let output = render(&block);
    assert!(
        output.contains("\"${files[@]}\""),
        "Expected array expansion, got:\n{output}"
    );
}

#[test]
fn verbatim_preserves_nested_substitution() {
    let block = sigil_quote!(Bash {
        local version = $V("$(cat ${PROJECT_ROOT}/VERSION | tr -d '\\n')")
    })
    .unwrap();
    let output = render(&block);
    assert!(
        output.contains("\"$(cat ${PROJECT_ROOT}/VERSION | tr -d '\\\\n')\""),
        "Expected nested command + braced sub, got:\n{output}"
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

#[test]
fn verbatim_complex_script_snippet() {
    let block = sigil_quote!(Bash {
        echo $V("Deploying ${APP_NAME} v${VERSION} to ${ENVIRONMENT}")
        echo $V("Commit: $(git rev-parse --short HEAD)")
        echo $V("Build time: $(date -u +%Y-%m-%dT%H:%M:%SZ)")
    })
    .unwrap();
    let output = render(&block);
    assert!(
        output.contains("\"Deploying ${APP_NAME} v${VERSION} to ${ENVIRONMENT}\""),
        "Expected complex braced vars, got:\n{output}"
    );
    assert!(
        output.contains("\"Commit: $(git rev-parse --short HEAD)\""),
        "Expected git command sub, got:\n{output}"
    );
    assert!(
        output.contains("\"Build time: $(date -u +%Y-%m-%dT%H:%M:%SZ)\""),
        "Expected date command sub, got:\n{output}"
    );
}
