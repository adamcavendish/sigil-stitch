use sigil_stitch::code_block::{CodeBlock, StringLitArg};
use sigil_stitch::lang::javascript::JavaScript;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::modifiers::Visibility;
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

/// Shorthand for a JS parameter (no type annotation).
fn param(name: &str) -> ParameterSpec<JavaScript> {
    ParameterSpec::new(name, TypeName::primitive("")).unwrap()
}

#[test]
fn test_export_function() {
    let body = CodeBlock::<JavaScript>::of(
        "console.log(%S + name);",
        (StringLitArg("Hello, ".to_string()),),
    )
    .unwrap();
    let mut fb = FunSpec::<JavaScript>::builder("greet");
    fb.visibility(Visibility::Public);
    fb.add_param(param("name"));
    fb.body(body);
    let fun = fb.build().unwrap();

    let mut file_b = FileSpec::builder_with("greet.js", JavaScript::new());
    file_b.add_function(fun);
    let file = file_b.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("javascript/export_function.js", &output);
}

#[test]
fn test_async_function() {
    let fetch_type = TypeName::<JavaScript>::importable("node-fetch", "fetch");

    let body = CodeBlock::<JavaScript>::of(
        "const response = await %T(url);\nreturn response.json();",
        (fetch_type,),
    )
    .unwrap();
    let mut fb = FunSpec::<JavaScript>::builder("fetchData");
    fb.visibility(Visibility::Public);
    fb.is_async();
    fb.add_param(param("url"));
    fb.body(body);
    let fun = fb.build().unwrap();

    let mut file_b = FileSpec::builder_with("api.js", JavaScript::new());
    file_b.add_function(fun);
    let file = file_b.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("javascript/async_function.js", &output);
}

#[test]
fn test_function_with_doc() {
    let body = CodeBlock::<JavaScript>::of("console.log('Hello, ' + name);", ()).unwrap();
    let mut fb = FunSpec::<JavaScript>::builder("greet");
    fb.visibility(Visibility::Public);
    fb.doc("Greet the user by name.");
    fb.add_param(param("name"));
    fb.body(body);
    let fun = fb.build().unwrap();

    let mut file_b = FileSpec::builder_with("greet_doc.js", JavaScript::new());
    file_b.add_function(fun);
    let file = file_b.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("javascript/function_with_doc.js", &output);
}
