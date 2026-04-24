use sigil_stitch::code_block::{CodeBlock, StringLitArg};
use sigil_stitch::lang::javascript::JavaScript;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_function_with_imports() {
    let format_date = TypeName::importable("./utils", "formatDate");
    let logger = TypeName::importable("./logger", "Logger");

    let mut b = CodeBlock::builder();
    b.add("function greet(name) {", ());
    b.add_line();
    b.add("%>", ());
    b.add("const date = %T();", (format_date,));
    b.add_line();
    b.add(
        "%T.log(%S + name);",
        (logger, StringLitArg("Hello, ".to_string())),
    );
    b.add_line();
    b.add("%<", ());
    b.add("}", ());
    b.add_line();
    let block = b.build().unwrap();

    let file = FileSpec::builder_with("greet.js", JavaScript::new())
        .add_code(block)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("javascript/function_with_imports.js", &output);
}

#[test]
fn test_no_import_type() {
    // Even when is_type_only is true, JS should NOT emit `import type`.
    let user = TypeName::importable_type("./models", "User");
    let create = TypeName::importable("./models", "createUser");

    let mut b = CodeBlock::builder();
    b.add("const u = new %T();", (user,));
    b.add_line();
    b.add("%T();", (create,));
    b.add_line();
    let block = b.build().unwrap();

    let file = FileSpec::builder_with("test.js", JavaScript::new())
        .add_code(block)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("javascript/no_import_type.js", &output);
}
