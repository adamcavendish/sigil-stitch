use crate::code_block::CodeBlock;
use crate::code_renderer::CodeRenderer;
use crate::import::ImportGroup;
use crate::import_collector;
use crate::lang::CodeLang;
use crate::spec::fun_spec::FunSpec;
use crate::spec::modifiers::DeclarationContext;
use crate::spec::type_spec::TypeSpec;

/// A member of a file.
#[derive(Debug, Clone)]
pub enum FileMember<L: CodeLang> {
    /// A CodeBlock (e.g., module-level statements, class declarations).
    Code(CodeBlock<L>),
    /// Raw content string (escape hatch, no import tracking).
    RawContent(String),
    /// A type declaration (struct, class, interface, trait, enum).
    Type(TypeSpec<L>),
    /// A top-level function.
    Fun(FunSpec<L>),
}

/// A complete source file with automatic import management.
///
/// FileSpec orchestrates the three-pass rendering model:
/// 1. **Pass 1 (Collect)**: Walk all CodeBlocks, extract import references
/// 2. **Import Resolution**: Dedup, detect conflicts, assign aliases
/// 3. **Pass 2 (Render)**: Render with resolved names, column tracking, pretty printing
#[derive(Debug, Clone)]
pub struct FileSpec<L: CodeLang> {
    filename: String,
    header: Option<CodeBlock<L>>,
    members: Vec<FileMember<L>>,
    lang: L,
}

impl<L: CodeLang> FileSpec<L> {
    /// Create a new FileSpec builder.
    pub fn builder(filename: &str) -> FileSpecBuilder<L>
    where
        L: Default,
    {
        FileSpecBuilder {
            filename: filename.to_string(),
            header: None,
            members: Vec::new(),
            lang: L::default(),
        }
    }

    /// Create a builder with a specific language configuration.
    pub fn builder_with(filename: &str, lang: L) -> FileSpecBuilder<L> {
        FileSpecBuilder {
            filename: filename.to_string(),
            header: None,
            members: Vec::new(),
            lang,
        }
    }

    /// Get the filename.
    pub fn filename(&self) -> &str {
        &self.filename
    }

    /// Render the file to a string using the three-pass algorithm.
    ///
    /// `width` controls the target line width for pretty-printing.
    pub fn render(&self, width: usize) -> Result<String, String> {
        // Phase 0: Materialize specs into CodeBlocks.
        enum Materialized<L: CodeLang> {
            Blocks(Vec<CodeBlock<L>>),
            Raw(String),
        }

        let materialized: Vec<Materialized<L>> = self
            .members
            .iter()
            .map(|m| match m {
                FileMember::Code(b) => Materialized::Blocks(vec![b.clone()]),
                FileMember::RawContent(s) => Materialized::Raw(s.clone()),
                FileMember::Type(spec) => Materialized::Blocks(spec.emit(&self.lang)),
                FileMember::Fun(spec) => {
                    Materialized::Blocks(vec![spec.emit(&self.lang, DeclarationContext::TopLevel)])
                }
            })
            .collect();

        // Pass 1: Collect imports from all CodeBlock members.
        let mut import_refs = Vec::new();

        if let Some(header) = &self.header {
            import_refs.extend(import_collector::collect_imports(header));
        }

        for mat in &materialized {
            if let Materialized::Blocks(blocks) = mat {
                for block in blocks {
                    import_refs.extend(import_collector::collect_imports(block));
                }
            }
        }

        // Import Resolution: Dedup, conflict detection, alias assignment.
        let imports = ImportGroup::resolve(&import_refs);

        // Pass 2: Render with resolved names.
        let mut output = String::new();

        // Render header block if present (e.g., license comment, Go package declaration).
        if let Some(header) = &self.header {
            let mut renderer = CodeRenderer::new(&self.lang, &imports, width);
            let header_output = renderer.render(header);
            if !header_output.is_empty() {
                output.push_str(&header_output);
                if !header_output.ends_with('\n') {
                    output.push('\n');
                }
                output.push('\n');
            }
        }

        // Render import header.
        let import_header = self.lang.render_imports(&imports);
        if !import_header.is_empty() {
            output.push_str(&import_header);
            output.push_str("\n\n");
        }

        // Render materialized members.
        for (i, mat) in materialized.iter().enumerate() {
            if i > 0 {
                output.push('\n');
            }
            match mat {
                Materialized::Blocks(blocks) => {
                    for (j, block) in blocks.iter().enumerate() {
                        if j > 0 {
                            output.push('\n');
                        }
                        let mut renderer = CodeRenderer::new(&self.lang, &imports, width);
                        let member_output = renderer.render(block);
                        output.push_str(&member_output);
                        if !member_output.ends_with('\n') {
                            output.push('\n');
                        }
                    }
                }
                Materialized::Raw(content) => {
                    output.push_str(content);
                    if !content.ends_with('\n') {
                        output.push('\n');
                    }
                }
            }
        }

        Ok(output)
    }
}

/// Builder for FileSpec.
#[derive(Debug)]
pub struct FileSpecBuilder<L: CodeLang> {
    filename: String,
    header: Option<CodeBlock<L>>,
    members: Vec<FileMember<L>>,
    lang: L,
}

impl<L: CodeLang> FileSpecBuilder<L> {
    /// Set a file header (e.g., license comment, package declaration).
    pub fn header(&mut self, block: CodeBlock<L>) -> &mut Self {
        self.header = Some(block);
        self
    }

    /// Add a CodeBlock member.
    pub fn add_code(&mut self, block: CodeBlock<L>) -> &mut Self {
        self.members.push(FileMember::Code(block));
        self
    }

    /// Add raw content (no import tracking).
    pub fn add_raw(&mut self, content: &str) -> &mut Self {
        self.members
            .push(FileMember::RawContent(content.to_string()));
        self
    }

    /// Add a generic member.
    pub fn add_member(&mut self, member: FileMember<L>) -> &mut Self {
        self.members.push(member);
        self
    }

    /// Add a type declaration (struct, class, interface, trait, enum).
    pub fn add_type(&mut self, spec: TypeSpec<L>) -> &mut Self {
        self.members.push(FileMember::Type(spec));
        self
    }

    /// Add a top-level function.
    pub fn add_function(&mut self, spec: FunSpec<L>) -> &mut Self {
        self.members.push(FileMember::Fun(spec));
        self
    }

    /// Set the language configuration.
    pub fn lang(&mut self, lang: L) -> &mut Self {
        self.lang = lang;
        self
    }

    /// Build the FileSpec.
    pub fn build(self) -> FileSpec<L> {
        FileSpec {
            filename: self.filename,
            header: self.header,
            members: self.members,
            lang: self.lang,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lang::typescript::TypeScript;
    use crate::type_name::TypeName;

    #[test]
    fn test_empty_file() {
        let file = FileSpec::<TypeScript>::builder("empty.ts").build();
        let output = file.render(80).unwrap();
        assert!(output.is_empty() || output.trim().is_empty());
    }

    #[test]
    fn test_simple_file_with_import() {
        let user = TypeName::<TypeScript>::importable_type("./models", "User");

        let mut b = CodeBlock::<TypeScript>::builder();
        b.add_statement("const u: %T = getUser()", (user,));
        let block = b.build().unwrap();

        let mut fb = FileSpec::<TypeScript>::builder("user.ts");
        fb.add_code(block);
        let file = fb.build();

        let output = file.render(80).unwrap();
        assert!(output.contains("import type { User } from './models'"));
        assert!(output.contains("const u: User = getUser();"));
    }

    #[test]
    fn test_conflicting_imports() {
        let user1 = TypeName::<TypeScript>::importable_type("./models", "User");
        let user2 = TypeName::<TypeScript>::importable_type("./other", "User");

        let mut b = CodeBlock::<TypeScript>::builder();
        b.add_statement("const u1: %T = get1()", (user1,));
        b.add_statement("const u2: %T = get2()", (user2,));
        let block = b.build().unwrap();

        let mut fb = FileSpec::<TypeScript>::builder("user.ts");
        fb.add_code(block);
        let file = fb.build();

        let output = file.render(80).unwrap();
        // First wins simple name.
        assert!(output.contains("const u1: User = get1();"));
        // Second gets alias.
        assert!(output.contains("const u2: OtherUser = get2();"));
        assert!(output.contains("User as OtherUser"));
    }

    #[test]
    fn test_raw_content_no_import_tracking() {
        let mut fb = FileSpec::<TypeScript>::builder("raw.ts");
        fb.add_raw("// This is raw content\nexport const VERSION = '1.0.0';\n");
        let file = fb.build();

        let output = file.render(80).unwrap();
        assert!(output.contains("// This is raw content"));
        assert!(output.contains("export const VERSION = '1.0.0';"));
        // No import header.
        assert!(!output.contains("import"));
    }

    #[test]
    fn test_mixed_code_and_raw() {
        let user = TypeName::<TypeScript>::importable_type("./models", "User");

        let mut b = CodeBlock::<TypeScript>::builder();
        b.add_statement("const u: %T = getUser()", (user,));
        let block = b.build().unwrap();

        let mut fb = FileSpec::<TypeScript>::builder("mixed.ts");
        fb.add_raw("// Generated file, do not edit.\n");
        fb.add_code(block);
        let file = fb.build();

        let output = file.render(80).unwrap();
        assert!(output.contains("import type { User }"));
        assert!(output.contains("// Generated file"));
        assert!(output.contains("const u: User = getUser();"));
    }

    #[test]
    fn test_file_with_header() {
        let mut header_builder = CodeBlock::<TypeScript>::builder();
        header_builder.add("// License: MIT", ());
        let header = header_builder.build().unwrap();

        let mut b = CodeBlock::<TypeScript>::builder();
        b.add_statement("const x = 1", ());
        let block = b.build().unwrap();

        let mut fb = FileSpec::<TypeScript>::builder("test.ts");
        fb.header(header);
        fb.add_code(block);
        let file = fb.build();

        let output = file.render(80).unwrap();
        assert!(output.starts_with("// License: MIT"));
        assert!(output.contains("const x = 1;"));
    }

    #[test]
    fn test_dedup_same_import() {
        let user1 = TypeName::<TypeScript>::importable_type("./models", "User");
        let user2 = TypeName::<TypeScript>::importable_type("./models", "User");

        let mut b = CodeBlock::<TypeScript>::builder();
        b.add_statement("const u1: %T = get1()", (user1,));
        b.add_statement("const u2: %T = get2()", (user2,));
        let block = b.build().unwrap();

        let mut fb = FileSpec::<TypeScript>::builder("user.ts");
        fb.add_code(block);
        let file = fb.build();

        let output = file.render(80).unwrap();
        // Should appear only once.
        let import_count = output.matches("import type { User }").count();
        assert_eq!(import_count, 1);
    }
}
