//! Tests for `$V` verbatim string literal in shell.
//!
//! $V for Bash/Zsh is pure passthrough — no quoting, no escaping.
//! Users include their own quotes in the $V content when shell quoting is needed.

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
    // Passthrough: no wrapping quotes — output is `echo $HOME/.config`
    assert!(
        output.contains("echo $HOME/.config"),
        "Expected passthrough (no quotes), got:\n{output}"
    );
    assert!(
        !output.contains("echo \"$HOME/.config\""),
        "Should NOT wrap in quotes, got:\n{output}"
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
        output.contains("= $(date +%Y-%m-%d)"),
        "Expected passthrough command sub, got:\n{output}"
    );
    assert!(
        !output.contains("\"$(date +%Y-%m-%d)\""),
        "Should NOT wrap in quotes, got:\n{output}"
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
        output.contains("= ${CONFIG_DIR:-$HOME/.config}"),
        "Expected passthrough braced expansion, got:\n{output}"
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
        output.contains("echo Result: $((count + 1)) items processed"),
        "Expected passthrough arithmetic, got:\n{output}"
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
        output.contains("echo PID=$$ args=$# status=$? all=$@"),
        "Expected passthrough special variables, got:\n{output}"
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
        output.contains("echo ${files[@]}"),
        "Expected passthrough array expansion, got:\n{output}"
    );
}

#[test]
fn verbatim_preserves_nested_substitution() {
    let block = sigil_quote!(Bash {
        local version = $V("$(cat ${PROJECT_ROOT}/VERSION | tr -d '\\n')")
    })
    .unwrap();
    let output = render(&block);
    // Passthrough: the Rust string literal `'\\n'` becomes `'\n'` which is passed through as-is
    assert!(
        output.contains("$(cat ${PROJECT_ROOT}/VERSION | tr -d"),
        "Expected passthrough nested sub, got:\n{output}"
    );
}

#[test]
fn verbatim_passthrough_no_escaping() {
    // Backslashes and quotes pass through unchanged
    let block = sigil_quote!(Bash {
        echo $V("path\\to\"file")
    })
    .unwrap();
    let output = render(&block);
    assert!(
        output.contains("echo path\\to\"file"),
        "Expected raw passthrough (no escaping), got:\n{output}"
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
    // $S escapes dollar: echo "\$HOME"
    assert!(
        output.contains("echo \"\\$HOME\""),
        "Expected $S to escape dollar, got:\n{output}"
    );
    // $V passthrough: echo $HOME (no quotes)
    assert!(
        output.contains("echo $HOME"),
        "Expected $V passthrough (no quotes), got:\n{output}"
    );
    // Verify the $V line doesn't have wrapping quotes
    for line in output.lines() {
        if line.contains("$HOME") && !line.contains("\\$HOME") {
            assert!(
                !line.contains("\"$HOME\""),
                "$V should not wrap in quotes, got line: {line}"
            );
        }
    }
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
        output.contains("echo Deploying ${APP_NAME} v${VERSION} to ${ENVIRONMENT}"),
        "Expected passthrough complex vars, got:\n{output}"
    );
    assert!(
        output.contains("echo Commit: $(git rev-parse --short HEAD)"),
        "Expected passthrough git sub, got:\n{output}"
    );
    assert!(
        output.contains("echo Build time: $(date -u +%Y-%m-%dT%H:%M:%SZ)"),
        "Expected passthrough date sub, got:\n{output}"
    );
}

#[test]
fn verbatim_with_explicit_quotes() {
    // When the user wants shell quoting, they include the quotes in the $V content
    let block = sigil_quote!(Bash {
        echo $V("\"Hello, ${USER}!\"")
        local config=$V("\"${XDG_CONFIG_HOME:-$HOME/.config}\"")
    })
    .unwrap();
    let output = render(&block);
    assert!(
        output.contains("echo \"Hello, ${USER}!\""),
        "Expected user-provided quotes passed through, got:\n{output}"
    );
    assert!(
        output.contains("config=\"${XDG_CONFIG_HOME:-$HOME/.config}\""),
        "Expected user-provided quotes for assignment, got:\n{output}"
    );
}
