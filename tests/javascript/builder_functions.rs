use sigil_stitch::code_block::{CodeBlock, StringLitArg};
use sigil_stitch::lang::javascript::JavaScript;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::modifiers::Visibility;
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

/// Shorthand for a JS parameter (no type annotation).
fn param(name: &str) -> ParameterSpec {
    ParameterSpec::new(name, TypeName::primitive("")).unwrap()
}

#[test]
fn test_export_function() {
    let body = CodeBlock::of(
        "console.log(%S + name);",
        (StringLitArg("Hello, ".to_string()),),
    )
    .unwrap();
    let fun = FunSpec::builder("greet")
        .visibility(Visibility::Public)
        .add_param(param("name"))
        .body(body)
        .build()
        .unwrap();

    let file = FileSpec::builder_with("greet.js", JavaScript::new())
        .add_function(fun)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("javascript/export_function.js", &output);
}

#[test]
fn test_async_function() {
    let fetch_type = TypeName::importable("node-fetch", "fetch");

    let body = CodeBlock::of(
        "const response = await %T(url);\nreturn response.json();",
        (fetch_type,),
    )
    .unwrap();
    let fun = FunSpec::builder("fetchData")
        .visibility(Visibility::Public)
        .is_async()
        .add_param(param("url"))
        .body(body)
        .build()
        .unwrap();

    let file = FileSpec::builder_with("api.js", JavaScript::new())
        .add_function(fun)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("javascript/async_function.js", &output);
}

#[test]
fn test_function_with_doc() {
    let body = CodeBlock::of("console.log('Hello, ' + name);", ()).unwrap();
    let fun = FunSpec::builder("greet")
        .visibility(Visibility::Public)
        .doc("Greet the user by name.")
        .add_param(param("name"))
        .body(body)
        .build()
        .unwrap();

    let file = FileSpec::builder_with("greet_doc.js", JavaScript::new())
        .add_function(fun)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("javascript/function_with_doc.js", &output);
}
