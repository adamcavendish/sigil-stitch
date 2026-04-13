use crate::import::{ImportEntry, ImportGroup};
use crate::lang::CodeLang;

/// Rust language implementation.
#[derive(Debug, Clone)]
pub struct RustLang {
    /// Indent with this string (default: "    ").
    pub indent: String,
}

impl Default for RustLang {
    fn default() -> Self {
        Self {
            indent: "    ".to_string(),
        }
    }
}

impl RustLang {
    pub fn new() -> Self {
        Self::default()
    }
}

const RUST_RESERVED: &[&str] = &[
    // Strict keywords (2024 edition)
    "as", "async", "await", "break", "const", "continue", "crate", "dyn",
    "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in",
    "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while",
    // Reserved keywords (cannot be used as identifiers)
    "abstract", "become", "box", "do", "final", "gen", "macro", "override",
    "priv", "try", "typeof", "unsized", "virtual", "yield",
];

impl CodeLang for RustLang {
    fn file_extension(&self) -> &str {
        "rs"
    }

    fn reserved_words(&self) -> &[&str] {
        RUST_RESERVED
    }

    fn render_imports(&self, imports: &ImportGroup) -> String {
        if imports.entries.is_empty() {
            return String::new();
        }

        // Group by crate origin: std/core first, then external, then crate::.
        let mut std_imports: Vec<&ImportEntry> = Vec::new();
        let mut external_imports: Vec<&ImportEntry> = Vec::new();
        let mut crate_imports: Vec<&ImportEntry> = Vec::new();

        for entry in &imports.entries {
            if entry.module.starts_with("std::") || entry.module.starts_with("core::") || entry.module == "std" || entry.module == "core" {
                std_imports.push(entry);
            } else if entry.module.starts_with("crate::") || entry.module.starts_with("super::") || entry.module.starts_with("self::") {
                crate_imports.push(entry);
            } else {
                external_imports.push(entry);
            }
        }

        let mut lines = Vec::new();

        // Group imports from the same module into `use mod::{A, B}` form.
        fn render_group(entries: &[&ImportEntry], lines: &mut Vec<String>) {
            let mut by_module: std::collections::BTreeMap<&str, Vec<&ImportEntry>> =
                std::collections::BTreeMap::new();
            for entry in entries {
                by_module.entry(&entry.module).or_default().push(entry);
            }
            for (module, items) in &by_module {
                if items.len() == 1 {
                    let entry = items[0];
                    if let Some(alias) = &entry.alias {
                        lines.push(format!("use {module}::{} as {alias};", entry.name));
                    } else {
                        lines.push(format!("use {module}::{};", entry.name));
                    }
                } else {
                    let mut specs: Vec<String> = items
                        .iter()
                        .map(|e| {
                            if let Some(alias) = &e.alias {
                                format!("{} as {alias}", e.name)
                            } else {
                                e.name.clone()
                            }
                        })
                        .collect();
                    specs.sort();
                    lines.push(format!("use {module}::{{{}}};", specs.join(", ")));
                }
            }
        }

        if !std_imports.is_empty() {
            render_group(&std_imports, &mut lines);
        }
        if !external_imports.is_empty() {
            if !lines.is_empty() {
                lines.push(String::new());
            }
            render_group(&external_imports, &mut lines);
        }
        if !crate_imports.is_empty() {
            if !lines.is_empty() {
                lines.push(String::new());
            }
            render_group(&crate_imports, &mut lines);
        }

        lines.join("\n")
    }

    fn render_string_literal(&self, s: &str) -> String {
        format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
    }

    fn render_doc_comment(&self, lines: &[&str]) -> String {
        lines
            .iter()
            .map(|line| {
                if line.is_empty() {
                    "///".to_string()
                } else {
                    format!("/// {line}")
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn line_comment_prefix(&self) -> &str {
        "//"
    }

    fn indent_unit(&self) -> &str {
        &self.indent
    }

    fn uses_semicolons(&self) -> bool {
        // Rust uses semicolons for statements but not for the last expression.
        // For code generation purposes, we default to true.
        true
    }

    fn escape_reserved(&self, name: &str) -> String {
        if self.reserved_words().contains(&name) {
            format!("r#{name}")
        } else {
            name.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_extension() {
        let rs = RustLang::new();
        assert_eq!(rs.file_extension(), "rs");
    }

    #[test]
    fn test_escape_reserved() {
        let rs = RustLang::new();
        assert_eq!(rs.escape_reserved("type"), "r#type");
        assert_eq!(rs.escape_reserved("my_var"), "my_var");
        // 2024 edition: gen is reserved
        assert_eq!(rs.escape_reserved("gen"), "r#gen");
    }

    #[test]
    fn test_render_imports_grouped() {
        let rs = RustLang::new();
        let imports = ImportGroup {
            entries: vec![
                ImportEntry {
                    module: "std::collections".into(),
                    name: "HashMap".into(),
                    alias: None,
                    is_type_only: false,
                },
                ImportEntry {
                    module: "std::collections".into(),
                    name: "BTreeMap".into(),
                    alias: None,
                    is_type_only: false,
                },
                ImportEntry {
                    module: "serde".into(),
                    name: "Serialize".into(),
                    alias: None,
                    is_type_only: false,
                },
                ImportEntry {
                    module: "crate::models".into(),
                    name: "User".into(),
                    alias: None,
                    is_type_only: false,
                },
            ],
        };
        let output = rs.render_imports(&imports);
        let lines: Vec<&str> = output.lines().collect();
        // std group first
        assert_eq!(lines[0], "use std::collections::{BTreeMap, HashMap};");
        // blank line
        assert_eq!(lines[1], "");
        // external
        assert_eq!(lines[2], "use serde::Serialize;");
        // blank line
        assert_eq!(lines[3], "");
        // crate
        assert_eq!(lines[4], "use crate::models::User;");
    }

    #[test]
    fn test_render_imports_with_alias() {
        let rs = RustLang::new();
        let imports = ImportGroup {
            entries: vec![ImportEntry {
                module: "models".into(),
                name: "User".into(),
                alias: Some("ModelsUser".into()),
                is_type_only: false,
            }],
        };
        let output = rs.render_imports(&imports);
        assert_eq!(output, "use models::User as ModelsUser;");
    }

    #[test]
    fn test_doc_comment() {
        let rs = RustLang::new();
        let doc = rs.render_doc_comment(&["Get the user.", "", "Returns None if not found."]);
        assert!(doc.contains("/// Get the user."));
        assert!(doc.contains("///\n"));
        assert!(doc.contains("/// Returns None if not found."));
    }

    #[test]
    fn test_string_literal() {
        let rs = RustLang::new();
        assert_eq!(rs.render_string_literal("hello"), "\"hello\"");
        assert_eq!(rs.render_string_literal("it\"s"), "\"it\\\"s\"");
    }
}
