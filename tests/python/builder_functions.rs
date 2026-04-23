use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::python::Python;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_top_level_function() {
    let json_dumps = TypeName::<Python>::importable("json", "dumps");

    let mut fb = FunSpec::<Python>::builder("serialize");
    fb.doc("Serialize an object to JSON.");
    fb.add_param(ParameterSpec::new("value", TypeName::primitive("object")).unwrap());
    fb.returns(TypeName::primitive("str"));
    fb.body(CodeBlock::<Python>::of("return %T(value)", (json_dumps,)).unwrap());

    let mut file_b = FileSpec::builder_with("utils.py", Python::new());
    file_b.add_function(fb.build().unwrap());
    let file = file_b.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("python/top_level_function.py", &output);
}

#[test]
fn test_function_with_doc() {
    let body = CodeBlock::<Python>::of("return f\"Hello, {name}!\"", ()).unwrap();
    let mut fb = FunSpec::<Python>::builder("greet");
    fb.doc("Greet the user by name.");
    fb.add_param(ParameterSpec::new("name", TypeName::primitive("str")).unwrap());
    fb.returns(TypeName::primitive("str"));
    fb.body(body);
    let fun = fb.build().unwrap();

    let mut file_b = FileSpec::<Python>::builder("greet.py");
    file_b.add_function(fun);
    let file = file_b.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("python/function_with_doc.py", &output);
}
