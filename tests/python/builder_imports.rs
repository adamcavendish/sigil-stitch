use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::python::Python;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_function_with_imports() {
    let json_dumps = TypeName::importable("json", "dumps");
    let http_server = TypeName::importable("http.server", "HTTPServer");

    let mut b = CodeBlock::builder();
    b.add("def start_server():", ());
    b.add_line();
    b.add("%>", ());
    b.add_statement("srv = %T()", (http_server,));
    b.add_statement("data = %T(srv)", (json_dumps,));
    b.add("%<", ());
    let block = b.build().unwrap();

    let file = FileSpec::builder_with("server.py", Python::new())
        .add_code(block)
        .build()
        .unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("python/function_with_imports.py", &output);
}

#[test]
fn test_import_grouping() {
    let json_dumps = TypeName::importable("json", "dumps");
    let json_loads = TypeName::importable("json", "loads");
    let flask = TypeName::importable("flask", "Flask");
    let os_path = TypeName::importable("os.path", "join");

    let mut b = CodeBlock::builder();
    b.add_statement("_ = %T()", (json_dumps,));
    b.add_statement("_ = %T()", (json_loads,));
    b.add_statement("_ = %T()", (flask,));
    b.add_statement("_ = %T()", (os_path,));
    let block = b.build().unwrap();

    let file = FileSpec::builder_with("app.py", Python::new())
        .add_code(block)
        .build()
        .unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("python/import_grouping.py", &output);
}
