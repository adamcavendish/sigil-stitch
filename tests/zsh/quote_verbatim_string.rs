//! Tests for `$V` verbatim string literal in Zsh.

use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder("test.zsh")
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn verbatim_preserves_zsh_parameter_flags() {
    let block = sigil_quote!(Zsh {
        local lower = $V("${(L)USERNAME}")
    })
    .unwrap();
    let output = render(&block);
    assert!(
        output.contains("\"${(L)USERNAME}\""),
        "Expected zsh parameter flag expansion, got:\n{output}"
    );
}

#[test]
fn verbatim_preserves_zsh_glob_qualifier() {
    let block = sigil_quote!(Zsh {
        local files = $V("${~pattern}")
    })
    .unwrap();
    let output = render(&block);
    assert!(
        output.contains("\"${~pattern}\""),
        "Expected zsh glob expansion, got:\n{output}"
    );
}

#[test]
fn verbatim_preserves_zsh_array_slicing() {
    let block = sigil_quote!(Zsh {
        echo $V("${array[2,-1]}")
    })
    .unwrap();
    let output = render(&block);
    assert!(
        output.contains("\"${array[2,-1]}\""),
        "Expected zsh array slice, got:\n{output}"
    );
}

#[test]
fn verbatim_preserves_zsh_substitution_flags() {
    let block = sigil_quote!(Zsh {
        local replaced = $V("${input//pattern/replacement}")
    })
    .unwrap();
    let output = render(&block);
    assert!(
        output.contains("\"${input//pattern/replacement}\""),
        "Expected zsh pattern substitution, got:\n{output}"
    );
}

#[test]
fn verbatim_preserves_process_substitution() {
    let block = sigil_quote!(Zsh {
        diff $V("$(cat file1.txt)") $V("$(cat file2.txt)")
    })
    .unwrap();
    let output = render(&block);
    assert!(
        output.contains("\"$(cat file1.txt)\""),
        "Expected process sub 1, got:\n{output}"
    );
    assert!(
        output.contains("\"$(cat file2.txt)\""),
        "Expected process sub 2, got:\n{output}"
    );
}

#[test]
fn verbatim_complex_zsh_script() {
    let block = sigil_quote!(Zsh {
        echo $V("Building ${(U)PROJECT_NAME} from ${GIT_BRANCH:-main}")
        echo $V("Artifacts: ${build_dir}/${(j:/:)path_components}")
        echo $V("Timestamp: $(date -u +%Y-%m-%dT%H:%M:%SZ)")
    })
    .unwrap();
    let output = render(&block);
    assert!(
        output.contains("\"Building ${(U)PROJECT_NAME} from ${GIT_BRANCH:-main}\""),
        "Expected uppercase flag + default, got:\n{output}"
    );
    assert!(
        output.contains("\"Artifacts: ${build_dir}/${(j:/:)path_components}\""),
        "Expected join flag, got:\n{output}"
    );
    assert!(
        output.contains("\"Timestamp: $(date -u +%Y-%m-%dT%H:%M:%SZ)\""),
        "Expected command sub, got:\n{output}"
    );
}
