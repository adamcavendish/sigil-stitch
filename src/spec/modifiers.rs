//! Modifier types for structural specs: visibility, declaration context, type kind.

/// Where a declaration appears: top-level file scope vs. inside a type body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclarationContext {
    /// Top-level (file scope): e.g., `export class` (TS), `pub struct` (Rust).
    TopLevel,
    /// Inside a type body: e.g., `private name: string` (TS), `pub name: String` (Rust).
    Member,
}

/// Visibility level for declarations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Visibility {
    /// No explicit visibility keyword.
    #[default]
    Inherited,
    /// `pub` (Rust), `export` (TS top-level), `public` (TS member).
    Public,
    /// `private` (TS member). Rust default is private, no keyword needed.
    Private,
    /// `protected` (TS member). No Rust equivalent.
    Protected,
    /// `pub(crate)` (Rust). No TS equivalent.
    PublicCrate,
    /// `pub(super)` (Rust). No TS equivalent.
    PublicSuper,
}

/// The kind of type declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeKind {
    /// TS: `class`, Rust: `struct` (struct-with-methods pattern).
    Class,
    /// Rust: `struct` (data-only). TS: maps to `interface`.
    Struct,
    /// TS: `interface`, Rust: `trait`.
    Interface,
    /// Rust: `trait`, TS: `interface`.
    Trait,
    /// Both languages: `enum`.
    Enum,
}

/// Modifier flags for a declaration.
#[derive(Debug, Clone, Default)]
pub struct Modifiers {
    pub visibility: Visibility,
    pub is_static: bool,
    pub is_abstract: bool,
    pub is_readonly: bool,
    pub is_async: bool,
    pub is_override: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visibility_default() {
        let vis = Visibility::default();
        assert_eq!(vis, Visibility::Inherited);
    }

    #[test]
    fn test_modifiers_default() {
        let m = Modifiers::default();
        assert_eq!(m.visibility, Visibility::Inherited);
        assert!(!m.is_static);
        assert!(!m.is_abstract);
        assert!(!m.is_readonly);
        assert!(!m.is_async);
        assert!(!m.is_override);
    }
}
