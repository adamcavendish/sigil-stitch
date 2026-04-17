//! Shared configuration types used across language implementations.
//!
//! These types live here (rather than inside individual `src/lang/*.rs` files)
//! because they represent cross-cutting concepts — quote style, optional-field
//! semantics — that multiple languages express in similar ways.

/// Quote style for rendering string literals.
///
/// Used by `TypeScript`, `JavaScript`, and `Python` where either single or
/// double quotes are valid and the choice is a project style decision.
/// Languages with a fixed quote style (Rust, Java, Go, etc.) don't consult
/// this enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum QuoteStyle {
    /// Single quotes (`'hello'`).
    #[default]
    Single,
    /// Double quotes (`"hello"`).
    Double,
}

impl QuoteStyle {
    /// The quote character for this style.
    pub fn char(self) -> char {
        match self {
            QuoteStyle::Single => '\'',
            QuoteStyle::Double => '"',
        }
    }
}

/// How a language expresses that a field is optional (key may be absent).
///
/// This is distinct from nullability (value may be `null`), which is handled
/// by `TypeName::Optional`. A `FieldSpec` marked `is_optional` is rendered
/// using this style.
///
/// Examples:
///
/// | Language | Style | Rendered output |
/// |----------|-------|-----------------|
/// | TypeScript | `NameSuffix("?")` | `name?: T` |
/// | Rust | `TypeWrap { open: "Option<", close: ">" }` | `name: Option<T>` |
/// | Go | `TypePrefix("*")` | `name *T` |
/// | Python | `UnionWithNone(" \| ")` | `name: T \| None` |
/// | Kotlin, Swift, Dart | `TypeSuffix("?")` | `name: T?` or `T? name` |
/// | Java | `TypeWrap { open: "Optional<", close: ">" }` | `Optional<T> name` |
/// | C++ | `TypeWrap { open: "std::optional<", close: ">" }` | `std::optional<T> name` |
/// | C | `TypePrefix("*")` | `T *name` |
/// | JavaScript, Bash, Zsh | `Ignored` | field rendered without any optionality marker |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionalFieldStyle {
    /// Append a suffix to the field name. TypeScript: `name?: T`.
    NameSuffix(&'static str),
    /// Append a suffix to the type. Kotlin/Swift/Dart: `T?`.
    TypeSuffix(&'static str),
    /// Wrap the type in `open...close`. Rust `Option<T>`, Java `Optional<T>`,
    /// C++ `std::optional<T>`.
    TypeWrap {
        /// Opening wrapper, e.g. `"Option<"`.
        open: &'static str,
        /// Closing wrapper, e.g. `">"`.
        close: &'static str,
    },
    /// Prepend a prefix to the type. Go: `name *T`, C: `T *name`.
    TypePrefix(&'static str),
    /// Render as a union with `None`. Python: `T | None` (separator is
    /// language-configurable for future flexibility).
    UnionWithNone(&'static str),
    /// Optional fields are not expressible in this language's type system.
    /// The field is rendered without any marker.
    Ignored,
}
