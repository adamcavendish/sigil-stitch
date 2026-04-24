use crate::code_block::{Arg, CodeBlock};
use crate::import::ImportRef;

/// Pass 1 of the three-pass rendering model.
///
/// Walks a CodeBlock tree structurally, collecting all `TypeName::Importable`
/// references. No rendering, no column tracking, no pretty printing.
pub fn collect_imports(block: &CodeBlock) -> Vec<ImportRef> {
    let mut refs = Vec::new();
    walk_parts(block, &mut refs);
    refs
}

fn walk_parts(block: &CodeBlock, refs: &mut Vec<ImportRef>) {
    // Walk args (which contain the actual TypeName/CodeBlock data).
    for arg in &block.args {
        match arg {
            Arg::TypeName(tn) => {
                tn.collect_imports(refs);
            }
            Arg::Code(inner) => {
                walk_parts(inner, refs);
            }
            _ => {}
        }
    }
}

/// Collect imports from multiple CodeBlocks.
pub fn collect_imports_many(blocks: &[&CodeBlock]) -> Vec<ImportRef> {
    let mut refs = Vec::new();
    for block in blocks {
        walk_parts(block, &mut refs);
    }
    refs
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::type_name::TypeName;

    #[test]
    fn test_collect_from_single_type() {
        let user = TypeName::importable("./models", "User");
        let mut builder = CodeBlock::builder();
        builder.add_statement("const u: %T = getUser()", (user,));
        let block = builder.build().unwrap();

        let refs = collect_imports(&block);
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].module, "./models");
        assert_eq!(refs[0].name, "User");
    }

    #[test]
    fn test_collect_from_nested_codeblock() {
        let user = TypeName::importable("./models", "User");
        let mut inner_builder = CodeBlock::builder();
        inner_builder.add_statement("return new %T()", (user,));
        let inner = inner_builder.build().unwrap();

        let mut outer_builder = CodeBlock::builder();
        outer_builder.add_code(inner);
        let outer = outer_builder.build().unwrap();

        let refs = collect_imports(&outer);
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].name, "User");
    }

    #[test]
    fn test_collect_three_types() {
        let user = TypeName::importable("./models", "User");
        let tag = TypeName::importable("./models", "Tag");
        let base = TypeName::importable("./base", "BaseApi");

        let mut builder = CodeBlock::builder();
        builder.add("const u: %T = get%T(%T)", (user, tag, base));
        let block = builder.build().unwrap();

        let refs = collect_imports(&block);
        assert_eq!(refs.len(), 3);
    }

    #[test]
    fn test_collect_no_types() {
        let mut builder = CodeBlock::builder();
        builder.add_statement("const x = 42", ());
        let block = builder.build().unwrap();

        let refs = collect_imports(&block);
        assert!(refs.is_empty());
    }

    #[test]
    fn test_collect_from_raw_content() {
        // Raw types should not produce imports.
        let raw = TypeName::raw("any");
        let mut builder = CodeBlock::builder();
        builder.add_statement("const x: %T = null", (raw,));
        let block = builder.build().unwrap();

        let refs = collect_imports(&block);
        assert!(refs.is_empty());
    }
}
