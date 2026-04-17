# Changelog

## 0.1.0 (unreleased)

Initial release.

### Code generation

- Type-safe, import-aware, width-aware code generation across 13 languages:
  TypeScript, JavaScript, Rust, Go, Python, Java, Kotlin, Swift, Dart, C, C++,
  Bash, Zsh.
- `CodeBlock` with format specifiers: `%T` (type, tracks imports), `%N` (name),
  `%S` (string literal), `%L` (literal or nested block), `%W` (soft line break),
  `%>`/`%<` (indent/dedent), `%[`/`%]` (statement boundaries), `%%` (escape).
- Three-pass rendering pipeline: materialize specs, collect and resolve imports,
  render with Wadler-Lindig pretty printing via the `pretty` crate.
- Automatic import conflict resolution with first-wins simple names and
  module-derived aliases.

### Spec layer

- `TypeSpec` (struct / class / interface / trait / enum), `FunSpec`,
  `FieldSpec`, `ParameterSpec`, `PropertySpec`, `AnnotationSpec`,
  `EnumVariantSpec`, `ImportSpec`.
- `FileSpec` for per-file orchestration; `ProjectSpec` for multi-file output
  with filesystem writing.
- `CodeTemplate` for reusable named-parameter templates (`#{name:K}` syntax).
- `sigil_quote!` proc macro for inline target-language code with `$T` / `$S` /
  `$N` / `$L` / `$C` / `$W` interpolation markers.

### Type-safe language parameterization

- Every type carries an `L: CodeLang` phantom parameter. Cross-language mixing
  is rejected at compile time.
- `TypeName` variants: `Importable`, `Primitive`, `Generic`, `Array`, `Union`,
  `Optional`, `Function`, `Map`, `Pointer`, `Slice`, `Raw`. All recursively
  collect imports.

### Errors and diagnostics

- `SigilStitchError` via `snafu`, no panics from builder paths.
- `FormatArgCount` includes the format string, expected specifiers, and actual
  argument kinds so mismatches point at the exact slot.
- `InvalidFormatSpecifier` surfaces unrecognised `%` sequences at build time.
- `TypeSpec::build()` rejects duplicate field names within a single type.

### Serialization

- All spec types derive `serde::Serialize` and `serde::Deserialize`. Specs
  round-trip through JSON, YAML, or any serde format. No feature flag required.

### Threading

- Rendering uses `pretty::BoxDoc` internally, so rendered documents are
  `Send + Sync` and can cross thread boundaries.

### Documentation

- mdbook at `doc/` covering introduction, getting started, architecture, format
  specifiers, spec layer, code templates, `sigil_quote!`, and adding a language.
- Published via GitHub Pages.
