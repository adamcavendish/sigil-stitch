use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::dart::DartLang;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_function_with_imports() {
    let future = TypeName::<DartLang>::importable("dart:async", "Future");
    let user = TypeName::<DartLang>::importable("package:myapp/models/user.dart", "User");

    let mut b = CodeBlock::<DartLang>::builder();
    b.add_statement("%T<%T> users = fetchAll()", (future, user));
    let block = b.build().unwrap();

    let mut fb = FileSpec::builder_with("app.dart", DartLang::new());
    fb.add_code(block);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("dart/function_with_imports.dart", &output);
}

#[test]
fn test_import_grouping() {
    let future = TypeName::<DartLang>::importable("dart:async", "Future");
    let convert = TypeName::<DartLang>::importable("dart:convert", "json");
    let http = TypeName::<DartLang>::importable("package:http/http.dart", "Client");
    let provider = TypeName::<DartLang>::importable("package:provider/provider.dart", "Provider");
    let user = TypeName::<DartLang>::importable("../models/user.dart", "User");
    let config = TypeName::<DartLang>::importable("./config.dart", "Config");

    let mut b = CodeBlock::<DartLang>::builder();
    b.add(
        "// %T %T %T %T %T %T",
        (future, convert, http, provider, user, config),
    );
    b.add_line();
    let block = b.build().unwrap();

    let mut fb = FileSpec::builder_with("imports.dart", DartLang::new());
    fb.add_code(block);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("dart/import_grouping.dart", &output);
}
