use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::python::Python;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::modifiers::TypeKind;
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::spec::type_spec::TypeSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_abstract_method() {
    let abc = TypeName::<Python>::importable("abc", "ABC");
    let abstractmethod = TypeName::<Python>::importable("abc", "abstractmethod");

    let mut tb = TypeSpec::<Python>::builder("BaseController", TypeKind::Class);
    tb.extends(abc);

    let mut handle = FunSpec::<Python>::builder("handle_request");
    handle.annotation(CodeBlock::<Python>::of("@%T", (abstractmethod,)).unwrap());
    handle.add_param(ParameterSpec::new("self", TypeName::primitive("")).unwrap());
    handle.add_param(ParameterSpec::new("req", TypeName::primitive("Request")).unwrap());
    handle.returns(TypeName::primitive("Response"));
    // No body — should emit `...`
    tb.add_method(handle.build().unwrap());

    let mut log_fn = FunSpec::<Python>::builder("log");
    log_fn.add_param(ParameterSpec::new("self", TypeName::primitive("")).unwrap());
    log_fn.returns(TypeName::primitive("None"));
    log_fn.body(CodeBlock::<Python>::of("print('handled')", ()).unwrap());
    tb.add_method(log_fn.build().unwrap());

    let mut fb = FileSpec::builder_with("controller.py", Python::new());
    fb.add_type(tb.build().unwrap());
    let file = fb.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("python/abstract_class.py", &output);
}

#[test]
fn test_decorated_function() {
    let mut fb = FunSpec::<Python>::builder("my_view");
    fb.annotation(CodeBlock::<Python>::of("@app.route('/hello')", ()).unwrap());
    fb.returns(TypeName::primitive("str"));
    fb.body(CodeBlock::<Python>::of("return 'Hello, World!'", ()).unwrap());

    let mut file_b = FileSpec::builder_with("views.py", Python::new());
    file_b.add_function(fb.build().unwrap());
    let file = file_b.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("python/decorated_function.py", &output);
}
