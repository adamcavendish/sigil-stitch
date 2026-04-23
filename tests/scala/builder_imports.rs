use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::scala::Scala;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_function_with_imports() {
    let list = TypeName::<Scala>::importable("scala.collection.immutable", "List");
    let user = TypeName::<Scala>::importable("com.example.model", "User");

    let mut b = CodeBlock::<Scala>::builder();
    b.add_statement("val users: %T[%T] = getAll()", (list, user));
    let block = b.build().unwrap();

    let mut fb = FileSpec::builder_with("App.scala", Scala::new());
    fb.add_code(block);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("scala/function_with_imports.scala", &output);
}

#[test]
fn test_import_grouping() {
    let list = TypeName::<Scala>::importable("scala.collection.immutable", "List");
    let uuid = TypeName::<Scala>::importable("java.util", "UUID");
    let inject = TypeName::<Scala>::importable("javax.inject", "Inject");
    let user = TypeName::<Scala>::importable("com.example.model", "User");
    let spark = TypeName::<Scala>::importable("org.apache.spark", "SparkContext");

    let mut b = CodeBlock::<Scala>::builder();
    b.add("// %T %T %T %T %T", (list, uuid, inject, user, spark));
    b.add_line();
    let block = b.build().unwrap();

    let mut fb = FileSpec::builder_with("Imports.scala", Scala::new());
    fb.add_code(block);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("scala/import_grouping.scala", &output);
}
