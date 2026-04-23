use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::typescript::TypeScript;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

fn render(block: &CodeBlock<TypeScript>) -> String {
    let mut fb = FileSpec::<TypeScript>::builder("test.ts");
    fb.add_code(block.clone());
    fb.build().unwrap().render(80).unwrap()
}

#[test]
fn test_imports() {
    let user_type = TypeName::<TypeScript>::importable_type("./models", "User");
    let repo_type = TypeName::<TypeScript>::importable_type("./repos", "UserRepository");
    let logger_type = TypeName::<TypeScript>::importable_type("./logging", "Logger");
    let block = sigil_quote!(TypeScript {
        const repo: $T(repo_type) = getRepo();
        const logger: $T(logger_type) = getLogger();
        const user: $T(user_type) = repo.findOne();
        logger.info($S("found user"));
    })
    .unwrap();
    golden::assert_golden("typescript/macro_imports.ts", &render(&block));
}
