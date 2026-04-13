//! Import resolution data structures.
//!
//! These types represent the result of import collection and resolution:
//! deduplicated, conflict-resolved, ready for language-specific rendering.

/// A single resolved import entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportEntry {
    /// Module path (e.g., "./models", "std::collections", "net/http").
    pub module: String,
    /// Simple name being imported (e.g., "User", "HashMap").
    pub name: String,
    /// Alias if there was a naming conflict (e.g., "OtherUser").
    pub alias: Option<String>,
    /// Whether this is a type-only import (TypeScript `import type`).
    pub is_type_only: bool,
}

impl ImportEntry {
    /// The name to use when referencing this import in code.
    pub fn resolved_name(&self) -> &str {
        self.alias.as_deref().unwrap_or(&self.name)
    }
}

/// A collection of resolved import entries, ready for rendering.
#[derive(Debug, Clone, Default)]
pub struct ImportGroup {
    pub entries: Vec<ImportEntry>,
}

/// Raw import reference collected from a CodeBlock tree walk (Pass 1).
/// Not yet resolved (no alias, no dedup).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImportRef {
    pub module: String,
    pub name: String,
    pub is_type_only: bool,
}

impl ImportGroup {
    pub fn new() -> Self {
        Self::default()
    }

    /// Resolve a list of raw import references into a deduplicated ImportGroup.
    /// First-encountered wins the simple name; later duplicates get aliases.
    pub fn resolve(refs: &[ImportRef]) -> Self {
        let mut entries = Vec::new();
        // Track which simple names are claimed and by which module.
        let mut claimed: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        // Track (module, name) pairs we've already added.
        let mut seen: std::collections::HashSet<(String, String)> =
            std::collections::HashSet::new();

        for import_ref in refs {
            let key = (import_ref.module.clone(), import_ref.name.clone());
            if seen.contains(&key) {
                continue;
            }
            seen.insert(key);

            let alias = if let Some(existing_module) = claimed.get(&import_ref.name) {
                if *existing_module == import_ref.module {
                    // Same module, same name, already claimed. No alias needed.
                    None
                } else {
                    // Conflict: another module already claimed this simple name.
                    // Generate alias from module path + name.
                    let module_prefix = module_to_prefix(&import_ref.module);
                    Some(format!("{}{}", module_prefix, import_ref.name))
                }
            } else {
                // First to claim this simple name.
                claimed.insert(import_ref.name.clone(), import_ref.module.clone());
                None
            };

            entries.push(ImportEntry {
                module: import_ref.module.clone(),
                name: import_ref.name.clone(),
                alias,
                is_type_only: import_ref.is_type_only,
            });
        }

        Self { entries }
    }

    /// Look up the resolved name for a given (module, name) pair.
    pub fn resolved_name(&self, module: &str, name: &str) -> Option<&str> {
        self.entries
            .iter()
            .find(|e| e.module == module && e.name == name)
            .map(|e| e.resolved_name())
    }
}

/// Convert a module path to a CamelCase prefix for aliasing.
/// "./models" -> "Models", "std::collections" -> "Collections",
/// "github.com/foo/bar" -> "Bar"
fn module_to_prefix(module: &str) -> String {
    let last_segment = module
        .rsplit(['/', ':', '.'])
        .find(|s| !s.is_empty())
        .unwrap_or(module);

    let mut chars = last_segment.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            let upper: String = first.to_uppercase().collect();
            format!("{upper}{}", chars.as_str())
        }
    }
}

/// Validate that a module path doesn't contain injection-prone characters.
pub fn validate_module_path(path: &str) -> Result<(), String> {
    if path.is_empty() {
        return Err("Module path cannot be empty".to_string());
    }
    // Reject characters that could break import syntax.
    for ch in path.chars() {
        match ch {
            '\n' | '\r' | '\'' | '"' | '`' | ';' | '{' | '}' | '(' | ')' => {
                return Err(format!(
                    "Module path contains invalid character: {:?}",
                    ch
                ));
            }
            _ => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedup_same_module() {
        let refs = vec![
            ImportRef {
                module: "./models".into(),
                name: "User".into(),
                is_type_only: true,
            },
            ImportRef {
                module: "./models".into(),
                name: "User".into(),
                is_type_only: true,
            },
        ];
        let group = ImportGroup::resolve(&refs);
        assert_eq!(group.entries.len(), 1);
        assert_eq!(group.entries[0].name, "User");
        assert!(group.entries[0].alias.is_none());
    }

    #[test]
    fn test_conflict_different_modules() {
        let refs = vec![
            ImportRef {
                module: "./models".into(),
                name: "User".into(),
                is_type_only: true,
            },
            ImportRef {
                module: "./other".into(),
                name: "User".into(),
                is_type_only: true,
            },
        ];
        let group = ImportGroup::resolve(&refs);
        assert_eq!(group.entries.len(), 2);
        // First wins simple name.
        assert!(group.entries[0].alias.is_none());
        assert_eq!(group.entries[0].name, "User");
        // Second gets alias.
        assert_eq!(group.entries[1].alias.as_deref(), Some("OtherUser"));
    }

    #[test]
    fn test_resolved_name_lookup() {
        let refs = vec![
            ImportRef {
                module: "./models".into(),
                name: "User".into(),
                is_type_only: true,
            },
            ImportRef {
                module: "./other".into(),
                name: "User".into(),
                is_type_only: true,
            },
        ];
        let group = ImportGroup::resolve(&refs);
        assert_eq!(group.resolved_name("./models", "User"), Some("User"));
        assert_eq!(group.resolved_name("./other", "User"), Some("OtherUser"));
    }

    #[test]
    fn test_module_to_prefix() {
        assert_eq!(module_to_prefix("./models"), "Models");
        assert_eq!(module_to_prefix("std::collections"), "Collections");
        assert_eq!(module_to_prefix("github.com/foo/bar"), "Bar");
        assert_eq!(module_to_prefix("net/http"), "Http");
    }

    #[test]
    fn test_validate_module_path() {
        assert!(validate_module_path("./models").is_ok());
        assert!(validate_module_path("std::collections").is_ok());
        assert!(validate_module_path("").is_err());
        assert!(validate_module_path("foo\nbar").is_err());
        assert!(validate_module_path("foo'bar").is_err());
    }
}
