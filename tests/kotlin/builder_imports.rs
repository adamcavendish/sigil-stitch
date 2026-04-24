use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::kotlin::Kotlin;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_function_with_imports() {
    let list = TypeName::importable("kotlin.collections", "List");
    let user = TypeName::importable("com.example.model", "User");

    let mut b = CodeBlock::builder();
    b.add_statement("val users: %T<%T> = getAll()", (list, user));
    let block = b.build().unwrap();

    let file = FileSpec::builder_with("App.kt", Kotlin::new())
        .add_code(block)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("kotlin/function_with_imports.kt", &output);
}

#[test]
fn test_import_grouping() {
    let list = TypeName::importable("kotlin.collections", "List");
    let scope = TypeName::importable("kotlinx.coroutines", "CoroutineScope");
    let uuid = TypeName::importable("java.util", "UUID");
    let inject = TypeName::importable("javax.inject", "Inject");
    let user = TypeName::importable("com.example.model", "User");
    let helper = TypeName::importable("io.ktor.server", "Application");

    let mut b = CodeBlock::builder();
    b.add(
        "// %T %T %T %T %T %T",
        (list, scope, uuid, inject, user, helper),
    );
    b.add_line();
    let block = b.build().unwrap();

    let file = FileSpec::builder_with("Imports.kt", Kotlin::new())
        .add_code(block)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("kotlin/import_grouping.kt", &output);
}
