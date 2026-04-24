//! Haskell language implementation.

use crate::import::{ImportEntry, ImportGroup};
use crate::lang::CodeLang;
use crate::spec::modifiers::{DeclarationContext, TypeKind, Visibility};

/// Haskell language implementation.
///
/// Haskell-specific behaviors:
/// - Prefix generic application (juxtaposition): `Maybe Int`, `Either String Int`
/// - No function keyword (type signatures use `::`, definitions have no keyword)
/// - `import Module (Name1, Name2)` for imports
/// - No semicolons (indentation-based)
/// - `data` for structs/classes/enums, `class` for type classes, `type` for aliases,
///   `newtype` for newtypes
/// - Record fields terminated with `,`
/// - Haddock doc comments: `-- | line1` / `--   line2`
/// - Line comments with `--`
/// - Curried function types: `Int -> String -> Bool`
/// - List type: `[Int]`
/// - Visibility controlled via module exports, not keywords
///
/// # Import conventions
///
/// Use [`crate::type_name::TypeName::importable`] with the module and name:
/// ```text
/// TypeName::importable("Data.Map", "Map")
/// TypeName::importable("Data.Text", "Text")
/// TypeName::importable("Control.Monad", "when")
/// ```
///
/// # Prefix generics
///
/// Haskell uses prefix generic application (juxtaposition):
/// - `Maybe Int`, `Either String Int`
/// - `Map String (Maybe Int)`
///
/// This is handled automatically via `generic_application_style() -> PrefixJuxtaposition`.
///
/// # Known limitations
///
/// - `block_open` returns `" ="` which works for function definitions and type
///   aliases, but type class declarations need `" where"`. Use
///   `begin_control_flow_with_open("class Functor f", (), " where")` for type classes.
/// - Complex multi-param type class constraints (e.g., `MonadReader Env m`) are not
///   directly modeled. Use `TypeName::primitive("(MonadIO m, MonadReader Env m) => m String")`
///   for complex constrained return types.
#[derive(Debug, Clone)]
pub struct Haskell {
    /// Indent with this string (default: "  " — 2 spaces).
    pub indent: String,
    /// File extension (default: "hs").
    pub extension: String,
}

impl Default for Haskell {
    fn default() -> Self {
        Self {
            indent: "  ".to_string(),
            extension: "hs".to_string(),
        }
    }
}

impl Haskell {
    /// Create a new Haskell language instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the indent string (e.g., `"  "` for 2-space default, `"\t"` for tabs).
    pub fn with_indent(mut self, s: &str) -> Self {
        self.indent = s.to_string();
        self
    }

    /// Set the file extension (default: `"hs"`). Set to `"lhs"` for literate Haskell.
    pub fn with_extension(mut self, s: &str) -> Self {
        self.extension = s.to_string();
        self
    }
}

#[rustfmt::skip]
const HASKELL_RESERVED: &[&str] = &[
    "as", "case", "class", "data", "default", "deriving", "do", "else",
    "forall", "foreign", "hiding", "if", "import", "in", "infix",
    "infixl", "infixr", "instance", "let", "module", "newtype", "of",
    "qualified", "then", "type", "where",
];

/// Classify an import module for ordering.
/// 0 = base/Prelude, 1 = standard libs (Data.*, Control.*, System.*), 2 = everything else.
fn import_group_order(module: &str) -> u8 {
    if module == "Prelude"
        || module.starts_with("Prelude.")
        || module == "GHC.Base"
        || module.starts_with("GHC.")
    {
        0
    } else if module.starts_with("Data.")
        || module.starts_with("Control.")
        || module.starts_with("System.")
    {
        1
    } else {
        2
    }
}

impl CodeLang for Haskell {
    fn file_extension(&self) -> &str {
        &self.extension
    }

    fn reserved_words(&self) -> &[&str] {
        HASKELL_RESERVED
    }

    fn escape_reserved(&self, name: &str) -> String {
        if self.reserved_words().contains(&name) {
            format!("{name}'")
        } else {
            name.to_string()
        }
    }

    fn render_imports(&self, imports: &ImportGroup) -> String {
        if imports.entries.is_empty() {
            return String::new();
        }

        // Group names by module.
        let mut by_module: std::collections::BTreeMap<&str, Vec<&ImportEntry>> =
            std::collections::BTreeMap::new();
        for entry in &imports.entries {
            if entry.is_side_effect {
                continue;
            }
            by_module.entry(&entry.module).or_default().push(entry);
        }

        let mut base_imports: Vec<String> = Vec::new();
        let mut std_imports: Vec<String> = Vec::new();
        let mut other_imports: Vec<String> = Vec::new();

        for (module, entries) in &by_module {
            let has_wildcard = entries.iter().any(|e| e.is_wildcard);
            let line = if has_wildcard {
                format!("import {module}")
            } else {
                let mut names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
                names.sort();
                names.dedup();
                format!("import {module} ({})", names.join(", "))
            };

            match import_group_order(module) {
                0 => base_imports.push(line),
                1 => std_imports.push(line),
                _ => other_imports.push(line),
            }
        }

        let groups: Vec<&Vec<String>> = [&base_imports, &std_imports, &other_imports]
            .into_iter()
            .filter(|g| !g.is_empty())
            .collect();

        let mut lines = Vec::new();
        for (i, group) in groups.iter().enumerate() {
            if i > 0 {
                lines.push(String::new());
            }
            lines.extend(group.iter().cloned());
        }

        lines.join("\n")
    }

    fn render_string_literal(&self, s: &str) -> String {
        format!(
            "\"{}\"",
            s.replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n")
                .replace('\t', "\\t")
        )
    }

    fn render_doc_comment(&self, lines: &[&str]) -> String {
        let mut result = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            if i == 0 {
                if line.is_empty() {
                    result.push("-- |".to_string());
                } else {
                    result.push(format!("-- | {line}"));
                }
            } else if line.is_empty() {
                result.push("--".to_string());
            } else {
                result.push(format!("--   {line}"));
            }
        }
        result.join("\n")
    }

    fn line_comment_prefix(&self) -> &str {
        "--"
    }

    fn indent_unit(&self) -> &str {
        &self.indent
    }

    fn uses_semicolons(&self) -> bool {
        false
    }

    fn render_visibility(&self, _vis: Visibility, _ctx: DeclarationContext) -> &str {
        ""
    }

    fn function_keyword(&self, _ctx: DeclarationContext) -> &str {
        ""
    }

    fn return_type_separator(&self) -> &str {
        " -> "
    }

    fn type_keyword(&self, kind: TypeKind) -> &str {
        match kind {
            TypeKind::Struct | TypeKind::Class => "data",
            TypeKind::Trait | TypeKind::Interface => "class",
            TypeKind::Enum => "data",
            TypeKind::TypeAlias => "type",
            TypeKind::Newtype => "newtype",
        }
    }

    fn field_terminator(&self) -> &str {
        ","
    }

    fn methods_inside_type_body(&self, kind: TypeKind) -> bool {
        matches!(kind, TypeKind::Trait | TypeKind::Interface)
    }

    fn generic_constraint_keyword(&self) -> &str {
        ""
    }

    fn generic_constraint_separator(&self) -> &str {
        ""
    }

    fn super_type_keyword(&self) -> &str {
        ""
    }

    fn implements_keyword(&self) -> &str {
        ""
    }

    fn type_annotation_separator(&self) -> &str {
        " :: "
    }

    fn generic_application_style(&self) -> crate::type_name::GenericApplicationStyle {
        crate::type_name::GenericApplicationStyle::PrefixJuxtaposition
    }

    fn generic_open(&self) -> &str {
        ""
    }

    fn generic_close(&self) -> &str {
        ""
    }

    fn enum_variant_separator(&self) -> &str {
        ""
    }

    fn enum_variant_prefix(&self) -> &str {
        "| "
    }

    fn enum_variant_prefix_first(&self) -> &str {
        ""
    }

    fn block_open(&self) -> &str {
        " ="
    }

    fn type_header_block_open(&self, kind: crate::spec::modifiers::TypeKind) -> &str {
        match kind {
            TypeKind::Trait | TypeKind::Interface => " where",
            _ => " =",
        }
    }

    fn block_close(&self) -> &str {
        ""
    }

    fn present_array(&self) -> crate::type_name::TypePresentation<'_> {
        crate::type_name::TypePresentation::Delimited {
            open: "[",
            sep: "",
            close: "]",
        }
    }

    fn present_readonly_array(&self) -> Option<crate::type_name::TypePresentation<'_>> {
        Some(crate::type_name::TypePresentation::Delimited {
            open: "[",
            sep: "",
            close: "]",
        })
    }

    fn present_optional(&self) -> crate::type_name::TypePresentation<'_> {
        crate::type_name::TypePresentation::GenericWrap { name: "Maybe" }
    }

    fn present_map(&self) -> crate::type_name::TypePresentation<'_> {
        crate::type_name::TypePresentation::GenericWrap { name: "Map" }
    }

    fn present_tuple(&self) -> crate::type_name::TypePresentation<'_> {
        crate::type_name::TypePresentation::Delimited {
            open: "(",
            sep: ", ",
            close: ")",
        }
    }

    fn present_function(&self) -> crate::type_name::FunctionPresentation<'_> {
        crate::type_name::FunctionPresentation {
            keyword: "",
            params_open: "",
            params_sep: " -> ",
            params_close: "",
            arrow: " -> ",
            return_first: false,
            curried: true,
            wrapper_open: "",
            wrapper_close: "",
        }
    }

    fn present_union(&self) -> crate::type_name::TypePresentation<'_> {
        crate::type_name::TypePresentation::Infix { sep: " | " }
    }

    fn render_newtype_line(&self, _vis: &str, name: &str, inner: &str) -> String {
        format!("newtype {name} = {name} {inner}")
    }

    fn function_signature_style(&self) -> crate::spec::fun_spec::FunctionSignatureStyle {
        crate::spec::fun_spec::FunctionSignatureStyle::Split
    }

    fn render_type_context(&self, type_params: &[crate::spec::fun_spec::TypeParamSpec]) -> String {
        let resolve = |_module: &str, name: &str| name.to_string();
        let mut constraints: Vec<String> = Vec::new();
        for tp in type_params {
            for bound in &tp.bounds {
                let bound_str = bound.render(80, &resolve).unwrap_or_default();
                constraints.push(format!("{bound_str} {}", tp.name));
            }
        }
        if constraints.is_empty() {
            return String::new();
        }
        if constraints.len() == 1 {
            format!("{} => ", constraints[0])
        } else {
            format!("({}) => ", constraints.join(", "))
        }
    }

    fn type_body_prefix(&self, name: &str, kind: crate::spec::modifiers::TypeKind) -> String {
        match kind {
            TypeKind::Struct | TypeKind::Class => format!("{name} {{"),
            _ => String::new(),
        }
    }

    fn type_body_suffix(&self, _name: &str, kind: crate::spec::modifiers::TypeKind) -> String {
        match kind {
            TypeKind::Struct | TypeKind::Class => "}".to_string(),
            _ => String::new(),
        }
    }

    fn render_type_close_suffix(
        &self,
        _kind: crate::spec::modifiers::TypeKind,
        impl_types: &[String],
    ) -> String {
        if impl_types.is_empty() {
            return String::new();
        }
        format!("  deriving ({})", impl_types.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_extension() {
        let hs = Haskell::new();
        assert_eq!(hs.file_extension(), "hs");
    }

    #[test]
    fn test_escape_reserved() {
        let hs = Haskell::new();
        assert_eq!(hs.escape_reserved("type"), "type'");
        assert_eq!(hs.escape_reserved("data"), "data'");
        assert_eq!(hs.escape_reserved("name"), "name");
    }

    #[test]
    fn test_render_imports_single() {
        let hs = Haskell::new();
        let imports = ImportGroup {
            entries: vec![ImportEntry {
                module: "Data.Map".into(),
                name: "Map".into(),
                alias: None,
                is_type_only: false,
                is_side_effect: false,
                is_wildcard: false,
            }],
        };
        assert_eq!(hs.render_imports(&imports), "import Data.Map (Map)");
    }

    #[test]
    fn test_render_imports_grouped() {
        let hs = Haskell::new();
        let imports = ImportGroup {
            entries: vec![
                ImportEntry {
                    module: "Data.Map".into(),
                    name: "Map".into(),
                    alias: None,
                    is_type_only: false,
                    is_side_effect: false,
                    is_wildcard: false,
                },
                ImportEntry {
                    module: "Data.Map".into(),
                    name: "fromList".into(),
                    alias: None,
                    is_type_only: false,
                    is_side_effect: false,
                    is_wildcard: false,
                },
                ImportEntry {
                    module: "MyApp.Types".into(),
                    name: "User".into(),
                    alias: None,
                    is_type_only: false,
                    is_side_effect: false,
                    is_wildcard: false,
                },
            ],
        };
        let output = hs.render_imports(&imports);
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines[0], "import Data.Map (Map, fromList)");
        assert_eq!(lines[1], "");
        assert_eq!(lines[2], "import MyApp.Types (User)");
    }

    #[test]
    fn test_render_imports_wildcard() {
        let hs = Haskell::new();
        let imports = ImportGroup {
            entries: vec![ImportEntry {
                module: "Data.List".into(),
                name: "".into(),
                alias: None,
                is_type_only: false,
                is_side_effect: false,
                is_wildcard: true,
            }],
        };
        assert_eq!(hs.render_imports(&imports), "import Data.List");
    }

    #[test]
    fn test_doc_comment_single() {
        let hs = Haskell::new();
        assert_eq!(
            hs.render_doc_comment(&["A brief description."]),
            "-- | A brief description."
        );
    }

    #[test]
    fn test_doc_comment_multi() {
        let hs = Haskell::new();
        let doc = hs.render_doc_comment(&["Get the user.", "", "Returns Nothing if not found."]);
        assert_eq!(
            doc,
            "-- | Get the user.\n--\n--   Returns Nothing if not found."
        );
    }

    #[test]
    fn test_string_literal() {
        let hs = Haskell::new();
        assert_eq!(hs.render_string_literal("hello"), "\"hello\"");
        assert_eq!(hs.render_string_literal("it\"s"), "\"it\\\"s\"");
        assert_eq!(hs.render_string_literal("new\nline"), "\"new\\nline\"");
    }

    #[test]
    fn test_type_keyword() {
        let hs = Haskell::new();
        assert_eq!(hs.type_keyword(TypeKind::Struct), "data");
        assert_eq!(hs.type_keyword(TypeKind::Class), "data");
        assert_eq!(hs.type_keyword(TypeKind::Trait), "class");
        assert_eq!(hs.type_keyword(TypeKind::Enum), "data");
        assert_eq!(hs.type_keyword(TypeKind::TypeAlias), "type");
        assert_eq!(hs.type_keyword(TypeKind::Newtype), "newtype");
    }

    #[test]
    fn test_visibility_always_empty() {
        let hs = Haskell::new();
        assert_eq!(
            hs.render_visibility(Visibility::Public, DeclarationContext::TopLevel),
            ""
        );
        assert_eq!(
            hs.render_visibility(Visibility::Private, DeclarationContext::TopLevel),
            ""
        );
    }

    #[test]
    fn test_no_semicolons() {
        let hs = Haskell::new();
        assert!(!hs.uses_semicolons());
    }

    #[test]
    fn test_generic_application_style() {
        let hs = Haskell::new();
        assert!(matches!(
            hs.generic_application_style(),
            crate::type_name::GenericApplicationStyle::PrefixJuxtaposition
        ));
    }

    #[test]
    fn test_type_annotation_separator() {
        let hs = Haskell::new();
        assert_eq!(hs.type_annotation_separator(), " :: ");
    }

    #[test]
    fn test_haskell_builder_fluent() {
        let hs = Haskell::new().with_indent("    ").with_extension("lhs");
        assert_eq!(hs.file_extension(), "lhs");
        assert_eq!(hs.indent_unit(), "    ");
    }

    #[test]
    fn test_import_group_order() {
        assert_eq!(import_group_order("Prelude"), 0);
        assert_eq!(import_group_order("GHC.Base"), 0);
        assert_eq!(import_group_order("Data.Map"), 1);
        assert_eq!(import_group_order("Control.Monad"), 1);
        assert_eq!(import_group_order("System.IO"), 1);
        assert_eq!(import_group_order("MyApp.Types"), 2);
    }

    #[test]
    fn test_render_type_context_empty() {
        let hs = Haskell::new();
        let params: Vec<crate::spec::fun_spec::TypeParamSpec> = vec![];
        assert_eq!(hs.render_type_context(&params), "");
    }

    #[test]
    fn test_render_type_context_single() {
        let hs = Haskell::new();
        let params = vec![
            crate::spec::fun_spec::TypeParamSpec::new("a")
                .with_bound(crate::type_name::TypeName::primitive("Show")),
        ];
        assert_eq!(hs.render_type_context(&params), "Show a => ");
    }

    #[test]
    fn test_render_type_context_multiple() {
        let hs = Haskell::new();
        let params = vec![
            crate::spec::fun_spec::TypeParamSpec::new("a")
                .with_bound(crate::type_name::TypeName::primitive("Show"))
                .with_bound(crate::type_name::TypeName::primitive("Eq")),
        ];
        assert_eq!(hs.render_type_context(&params), "(Show a, Eq a) => ");
    }

    #[test]
    fn test_type_body_prefix_struct() {
        let hs = Haskell::new();
        assert_eq!(hs.type_body_prefix("Person", TypeKind::Struct), "Person {");
    }

    #[test]
    fn test_type_body_prefix_trait() {
        let hs = Haskell::new();
        assert_eq!(hs.type_body_prefix("Functor", TypeKind::Trait), "");
    }

    #[test]
    fn test_type_body_suffix_struct() {
        let hs = Haskell::new();
        assert_eq!(hs.type_body_suffix("Person", TypeKind::Struct), "}");
    }

    #[test]
    fn test_render_type_close_suffix_empty() {
        let hs = Haskell::new();
        let empty: Vec<String> = vec![];
        assert_eq!(hs.render_type_close_suffix(TypeKind::Enum, &empty), "");
    }

    #[test]
    fn test_render_type_close_suffix_deriving() {
        let hs = Haskell::new();
        let types = vec!["Show".to_string(), "Eq".to_string()];
        assert_eq!(
            hs.render_type_close_suffix(TypeKind::Enum, &types),
            "  deriving (Show, Eq)"
        );
    }

    #[test]
    fn test_render_newtype_line() {
        let hs = Haskell::new();
        assert_eq!(
            hs.render_newtype_line("", "Meters", "f64"),
            "newtype Meters = Meters f64"
        );
    }

    #[test]
    fn test_function_signature_style() {
        let hs = Haskell::new();
        assert_eq!(
            hs.function_signature_style(),
            crate::spec::fun_spec::FunctionSignatureStyle::Split
        );
    }
}
