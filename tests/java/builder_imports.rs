use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::java_lang::JavaLang;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_function_with_imports() {
    let list = TypeName::<JavaLang>::importable("java.util", "List");
    let user = TypeName::<JavaLang>::importable("com.example.model", "User");

    let mut b = CodeBlock::<JavaLang>::builder();
    b.add_statement("%T<%T> users = getAll()", (list, user));
    let block = b.build().unwrap();

    let mut fb = FileSpec::builder_with("App.java", JavaLang::new());
    fb.add_code(block);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("java/function_with_imports.java", &output);
}

#[test]
fn test_import_grouping() {
    let list = TypeName::<JavaLang>::importable("java.util", "List");
    let file_type = TypeName::<JavaLang>::importable("java.io", "File");
    let entity = TypeName::<JavaLang>::importable("javax.persistence", "Entity");
    let user = TypeName::<JavaLang>::importable("com.example.model", "User");
    let helper = TypeName::<JavaLang>::importable("com.example.util", "Helper");

    let mut b = CodeBlock::<JavaLang>::builder();
    b.add("// %T %T %T %T %T", (list, file_type, entity, user, helper));
    b.add_line();
    let block = b.build().unwrap();

    let mut fb = FileSpec::builder_with("Imports.java", JavaLang::new());
    fb.add_code(block);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("java/import_grouping.java", &output);
}
