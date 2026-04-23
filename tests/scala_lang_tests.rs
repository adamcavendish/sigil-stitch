mod golden;

use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::scala::Scala;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

#[test]
fn test_scala_function_with_imports() {
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
fn test_scala_import_grouping() {
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

#[test]
fn test_scala_control_flow() {
    let mut b = CodeBlock::<Scala>::builder();
    b.begin_control_flow("if (x > 0)", ());
    b.add_statement("return 1", ());
    b.next_control_flow("else if (x < 0)", ());
    b.add_statement("return -1", ());
    b.next_control_flow("else", ());
    b.add_statement("return 0", ());
    b.end_control_flow();
    let block = b.build().unwrap();

    let mut fb = FileSpec::builder_with("Flow.scala", Scala::new());
    fb.add_code(block);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("scala/control_flow.scala", &output);
}
