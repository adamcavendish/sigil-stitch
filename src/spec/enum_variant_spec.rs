//! Enum variant specification for type-safe enum generation.

use crate::code_block::CodeBlock;
use crate::lang::CodeLang;
use crate::spec::annotation_spec::AnnotationSpec;

/// A single enum variant (e.g., `Red`, `Up = 'UP'`, `case red`).
///
/// Used with [`crate::spec::type_spec::TypeSpec`] via `add_variant()`.
/// The language's [`CodeLang::enum_variant_prefix`], [`CodeLang::enum_variant_separator`],
/// and [`CodeLang::enum_variant_trailing_separator`] control rendering.
///
/// For advanced forms (Rust tuple/struct variants, Java constructor args),
/// use `extra_member()` on TypeSpec as an escape hatch.
#[derive(Debug, Clone)]
pub struct EnumVariantSpec<L: CodeLang> {
    pub(crate) name: String,
    pub(crate) doc: Vec<String>,
    pub(crate) value: Option<CodeBlock<L>>,
    pub(crate) annotations: Vec<CodeBlock<L>>,
    pub(crate) annotation_specs: Vec<AnnotationSpec<L>>,
}

impl<L: CodeLang> EnumVariantSpec<L> {
    /// Create a simple variant with just a name.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            doc: Vec::new(),
            value: None,
            annotations: Vec::new(),
            annotation_specs: Vec::new(),
        }
    }

    /// Create a variant builder for more complex variants.
    pub fn builder(name: &str) -> EnumVariantSpecBuilder<L> {
        EnumVariantSpecBuilder {
            name: name.to_string(),
            doc: Vec::new(),
            value: None,
            annotations: Vec::new(),
            annotation_specs: Vec::new(),
        }
    }
}

/// Builder for [`EnumVariantSpec`].
#[derive(Debug)]
pub struct EnumVariantSpecBuilder<L: CodeLang> {
    name: String,
    doc: Vec<String>,
    value: Option<CodeBlock<L>>,
    annotations: Vec<CodeBlock<L>>,
    annotation_specs: Vec<AnnotationSpec<L>>,
}

impl<L: CodeLang> EnumVariantSpecBuilder<L> {
    /// Add a doc comment line.
    pub fn doc(&mut self, line: &str) -> &mut Self {
        self.doc.push(line.to_string());
        self
    }

    /// Set the variant's value (e.g., `= 0`, `= 'UP'`, `= auto()`).
    pub fn value(&mut self, val: CodeBlock<L>) -> &mut Self {
        self.value = Some(val);
        self
    }

    /// Add an annotation (e.g., `#[default]`, `@JsonValue`).
    pub fn annotation(&mut self, ann: CodeBlock<L>) -> &mut Self {
        self.annotations.push(ann);
        self
    }

    /// Add a structured annotation.
    pub fn annotate(&mut self, spec: AnnotationSpec<L>) -> &mut Self {
        self.annotation_specs.push(spec);
        self
    }

    /// Build the variant spec.
    pub fn build(self) -> EnumVariantSpec<L> {
        EnumVariantSpec {
            name: self.name,
            doc: self.doc,
            value: self.value,
            annotations: self.annotations,
            annotation_specs: self.annotation_specs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lang::rust_lang::RustLang;
    use crate::lang::swift::Swift;
    use crate::lang::typescript::TypeScript;
    use crate::spec::modifiers::TypeKind;
    use crate::spec::type_spec::TypeSpec;

    fn render_enum<L: CodeLang>(ts: &TypeSpec<L>, lang: &L) -> String {
        let blocks = ts.emit(lang);
        let imports = crate::import::ImportGroup::new();
        let mut output = String::new();
        for (i, block) in blocks.iter().enumerate() {
            if i > 0 {
                output.push('\n');
            }
            let mut renderer = crate::code_renderer::CodeRenderer::new(lang, &imports, 80);
            output.push_str(&renderer.render(block));
        }
        output
    }

    #[test]
    fn test_simple_variants() {
        let mut tb = TypeSpec::<RustLang>::builder("Color", TypeKind::Enum);
        tb.add_variant(EnumVariantSpec::new("Red"));
        tb.add_variant(EnumVariantSpec::new("Green"));
        tb.add_variant(EnumVariantSpec::new("Blue"));
        let ts = tb.build();
        let output = render_enum(&ts, &RustLang::new());
        assert!(output.contains("Red,"));
        assert!(output.contains("Green,"));
        assert!(output.contains("Blue,"));
    }

    #[test]
    fn test_variant_with_value() {
        let mut tb = TypeSpec::<TypeScript>::builder("Direction", TypeKind::Enum);
        let mut v = EnumVariantSpec::builder("Up");
        v.value(CodeBlock::<TypeScript>::of("'UP'", ()).unwrap());
        tb.add_variant(v.build());
        let ts = tb.build();
        let output = render_enum(&ts, &TypeScript::new());
        assert!(output.contains("Up = 'UP',"));
    }

    #[test]
    fn test_swift_variant_prefix() {
        let mut tb = TypeSpec::<Swift>::builder("Color", TypeKind::Enum);
        tb.add_variant(EnumVariantSpec::new("red"));
        tb.add_variant(EnumVariantSpec::new("green"));
        let ts = tb.build();
        let output = render_enum(&ts, &Swift::new());
        assert!(output.contains("case red"));
        assert!(output.contains("case green"));
        // Swift has no separator.
        assert!(!output.contains("case red,"));
    }

    #[test]
    fn test_trailing_separator() {
        let mut tb = TypeSpec::<RustLang>::builder("Color", TypeKind::Enum);
        tb.add_variant(EnumVariantSpec::new("Red"));
        let ts = tb.build();
        let output = render_enum(&ts, &RustLang::new());
        // Rust has trailing comma.
        assert!(output.contains("Red,"));
    }

    #[test]
    fn test_no_trailing_separator() {
        let mut tb = TypeSpec::<crate::lang::c_lang::CLang>::builder("Color", TypeKind::Enum);
        tb.add_variant(EnumVariantSpec::new("RED"));
        tb.add_variant(EnumVariantSpec::new("GREEN"));
        let ts = tb.build();
        let output = render_enum(&ts, &crate::lang::c_lang::CLang::new());
        assert!(output.contains("RED,"));
        // Last variant has no trailing comma in C.
        assert!(output.contains("GREEN\n"));
        assert!(!output.contains("GREEN,"));
    }
}
