use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::cpp_lang::CppLang;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_const_method() {
    let mut fb = FunSpec::<CppLang>::builder("size");
    fb.returns(TypeName::primitive("int"));
    fb.suffix("const");
    fb.suffix("noexcept");
    let fun = fb.build().unwrap();

    let mut file_b = FileSpec::builder_with("api.hpp", CppLang::header());
    file_b.add_function(fun);
    let file = file_b.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("cpp/const_method.cpp", &output);
}

#[test]
fn test_template_function() {
    let mut fb = FunSpec::<CppLang>::builder("max_of");
    fb.annotation(CodeBlock::<CppLang>::of("template<typename T>", ()).unwrap());
    fb.add_param(ParameterSpec::new("a", TypeName::primitive("const T&")).unwrap());
    fb.add_param(ParameterSpec::new("b", TypeName::primitive("const T&")).unwrap());
    fb.returns(TypeName::primitive("T"));
    let body = CodeBlock::<CppLang>::of("return (a > b) ? a : b;", ()).unwrap();
    fb.body(body);
    let fun = fb.build().unwrap();

    let mut file_b = FileSpec::builder_with("algo.hpp", CppLang::header());
    file_b.add_function(fun);
    let file = file_b.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("cpp/template_function.cpp", &output);
}

#[test]
fn test_static_method() {
    let body = CodeBlock::<CppLang>::of("return instance_count_;", ()).unwrap();
    let mut fb = FunSpec::<CppLang>::builder("count");
    fb.is_static();
    fb.returns(TypeName::primitive("int"));
    fb.body(body);
    let fun = fb.build().unwrap();

    let mut file_b = FileSpec::builder_with("helpers.cpp", CppLang::new());
    file_b.add_function(fun);
    let file = file_b.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("cpp/static_method.cpp", &output);
}

#[test]
fn test_function_with_doc() {
    let body = CodeBlock::<CppLang>::of("return (a > b) ? a : b;", ()).unwrap();
    let mut fb = FunSpec::<CppLang>::builder("max_val");
    fb.doc("Return the larger of two values.");
    fb.add_param(ParameterSpec::new("a", TypeName::primitive("int")).unwrap());
    fb.add_param(ParameterSpec::new("b", TypeName::primitive("int")).unwrap());
    fb.returns(TypeName::primitive("int"));
    fb.body(body);
    let fun = fb.build().unwrap();

    let mut file_b = FileSpec::builder_with("math_doc.cpp", CppLang::new());
    file_b.add_function(fun);
    let file = file_b.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("cpp/function_with_doc.cpp", &output);
}
