use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::python::Python;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_function_with_imports() {
    let json_dumps = TypeName::<Python>::importable("json", "dumps");
    let http_server = TypeName::<Python>::importable("http.server", "HTTPServer");

    let mut b = CodeBlock::<Python>::builder();
    b.add("def start_server():", ());
    b.add_line();
    b.add("%>", ());
    b.add_statement("srv = %T()", (http_server,));
    b.add_statement("data = %T(srv)", (json_dumps,));
    b.add("%<", ());
    let block = b.build().unwrap();

    let mut fb = FileSpec::builder_with("server.py", Python::new());
    fb.add_code(block);
    let file = fb.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("python/function_with_imports.py", &output);
}

#[test]
fn test_import_grouping() {
    let json_dumps = TypeName::<Python>::importable("json", "dumps");
    let json_loads = TypeName::<Python>::importable("json", "loads");
    let flask = TypeName::<Python>::importable("flask", "Flask");
    let os_path = TypeName::<Python>::importable("os.path", "join");

    let mut b = CodeBlock::<Python>::builder();
    b.add_statement("_ = %T()", (json_dumps,));
    b.add_statement("_ = %T()", (json_loads,));
    b.add_statement("_ = %T()", (flask,));
    b.add_statement("_ = %T()", (os_path,));
    let block = b.build().unwrap();

    let mut fb = FileSpec::builder_with("app.py", Python::new());
    fb.add_code(block);
    let file = fb.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("python/import_grouping.py", &output);
}
