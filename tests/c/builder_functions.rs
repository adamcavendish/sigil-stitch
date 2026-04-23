use sigil_stitch::code_block::{CodeBlock, StringLitArg};
use sigil_stitch::lang::c_lang::CLang;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::modifiers::Visibility;
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_function_with_params() {
    let body = CodeBlock::<CLang>::of("return a + b;", ()).unwrap();
    let mut fb = FunSpec::<CLang>::builder("add");
    fb.add_param(ParameterSpec::new("a", TypeName::primitive("int")).unwrap());
    fb.add_param(ParameterSpec::new("b", TypeName::primitive("int")).unwrap());
    fb.returns(TypeName::primitive("int"));
    fb.body(body);
    let fun = fb.build().unwrap();

    let mut file_b = FileSpec::builder_with("math.c", CLang::new());
    file_b.add_function(fun);
    let file = file_b.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("c/function_with_params.c", &output);
}

#[test]
fn test_void_function() {
    let printf_type = TypeName::<CLang>::importable("stdio.h", "printf");
    let body = CodeBlock::<CLang>::of(
        "%T(%S, name);",
        (printf_type, StringLitArg("Hello, %s!\\n".to_string())),
    )
    .unwrap();
    let mut fb = FunSpec::<CLang>::builder("greet");
    fb.add_param(ParameterSpec::new("name", TypeName::primitive("const char*")).unwrap());
    fb.returns(TypeName::primitive("void"));
    fb.body(body);
    let fun = fb.build().unwrap();

    let mut file_b = FileSpec::builder_with("greet.c", CLang::new());
    file_b.add_function(fun);
    let file = file_b.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("c/void_function.c", &output);
}

#[test]
fn test_static_function() {
    let body = CodeBlock::<CLang>::of("return x * x;", ()).unwrap();
    let mut fb = FunSpec::<CLang>::builder("square");
    fb.visibility(Visibility::Private);
    fb.is_static();
    fb.add_param(ParameterSpec::new("x", TypeName::primitive("int")).unwrap());
    fb.returns(TypeName::primitive("int"));
    fb.body(body);
    let fun = fb.build().unwrap();

    let mut file_b = FileSpec::builder_with("helpers.c", CLang::new());
    file_b.add_function(fun);
    let file = file_b.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("c/static_function.c", &output);
}

#[test]
fn test_function_declaration() {
    // Forward declaration — no body, should end with semicolon.
    let mut fb = FunSpec::<CLang>::builder("process");
    fb.add_param(ParameterSpec::new("data", TypeName::primitive("const char*")).unwrap());
    fb.add_param(ParameterSpec::new("len", TypeName::primitive("size_t")).unwrap());
    fb.returns(TypeName::primitive("int"));
    let fun = fb.build().unwrap();

    let mut file_b = FileSpec::builder_with("api.h", CLang::header());
    file_b.add_function(fun);
    let file = file_b.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("c/function_declaration.c", &output);
}

#[test]
fn test_function_with_doc() {
    let body = CodeBlock::<CLang>::of("return a + b;", ()).unwrap();
    let mut fb = FunSpec::<CLang>::builder("add");
    fb.doc("Add two integers.");
    fb.add_param(ParameterSpec::new("a", TypeName::primitive("int")).unwrap());
    fb.add_param(ParameterSpec::new("b", TypeName::primitive("int")).unwrap());
    fb.returns(TypeName::primitive("int"));
    fb.body(body);
    let fun = fb.build().unwrap();

    let mut file_b = FileSpec::builder_with("math_doc.c", CLang::new());
    file_b.add_function(fun);
    let file = file_b.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("c/function_with_doc.c", &output);
}
