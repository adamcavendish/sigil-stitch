pub mod typescript;
pub mod rust_lang;

use crate::import::ImportGroup;

/// Trait defining language-specific behavior for code generation.
///
/// All types in sigil-stitch are parameterized by `L: CodeLang`, allowing
/// the same CodeBlock and TypeName structures to generate output for any
/// supported language.
///
/// Methods that may vary per configuration (indent_unit, uses_semicolons,
/// render_string_literal) take `&self`. Methods that are truly invariant
/// per language (file_extension, reserved_words) also take `&self` for
/// consistency and future flexibility.
pub trait CodeLang: Sized + 'static {
    /// File extension for this language (e.g., "ts", "go", "rs").
    fn file_extension(&self) -> &str;

    /// Reserved words that need escaping.
    fn reserved_words(&self) -> &[&str];

    /// Render an import group to a string.
    /// `imports` is deduplicated and grouped by module.
    fn render_imports(&self, imports: &ImportGroup) -> String;

    /// Render a string literal with language-appropriate quoting and escaping.
    fn render_string_literal(&self, s: &str) -> String;

    /// Render a doc comment block.
    fn render_doc_comment(&self, lines: &[&str]) -> String;

    /// Single-line comment prefix (e.g., "//", "#").
    fn line_comment_prefix(&self) -> &str;

    /// Indentation unit (e.g., "  " for 2-space, "\t" for tabs).
    fn indent_unit(&self) -> &str;

    /// Whether this language uses semicolons to terminate statements.
    fn uses_semicolons(&self) -> bool;

    /// Escape a name if it collides with a reserved word.
    /// Default: append underscore.
    fn escape_reserved(&self, name: &str) -> String {
        if self.reserved_words().contains(&name) {
            format!("{name}_")
        } else {
            name.to_string()
        }
    }
}
