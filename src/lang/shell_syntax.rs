//! Shared control-flow syntax for POSIX-like shells (Bash, Zsh).
//!
//! Both shells use identical block delimiters: `if/then/fi`, `for/do/done`,
//! `while/do/done`, `until/do/done`, `case/in/esac`.

/// Map a control-flow condition to its shell block-opening delimiter.
pub(crate) fn shell_block_open_for(condition: &str) -> Option<&'static str> {
    let raw = condition.trim();
    let t = raw.trim_end_matches(';').trim();
    if t.ends_with("; then")
        || t.ends_with("; do")
        || t.ends_with(" in")
        || t == "else"
        || t == "elif"
    {
        Some("")
    } else if t.starts_with("if ") || t.starts_with("elif ") {
        if raw.ends_with(';') {
            Some(" then")
        } else {
            Some("; then")
        }
    } else if t.starts_with("for ") || t.starts_with("while ") || t.starts_with("until ") {
        if raw.ends_with(';') {
            Some(" do")
        } else {
            Some("; do")
        }
    } else if t.starts_with("case ") {
        Some(" in")
    } else {
        None
    }
}

/// Map a control-flow condition to its shell block-closing delimiter.
pub(crate) fn shell_block_close_for(condition: &str) -> Option<&'static str> {
    let t = condition.trim().trim_end_matches(';').trim();
    if t.starts_with("if ") || t.starts_with("elif ") || t == "else" {
        Some("fi")
    } else if t.starts_with("for ") || t.starts_with("while ") || t.starts_with("until ") {
        Some("done")
    } else if t.starts_with("case ") {
        Some("esac")
    } else {
        None
    }
}
