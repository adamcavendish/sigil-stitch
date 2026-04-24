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
    let abc = TypeName::importable("abc", "ABC");
    let abstractmethod = TypeName::importable("abc", "abstractmethod");

    let handle = FunSpec::builder("handle_request")
        .annotation(CodeBlock::of("@%T", (abstractmethod,)).unwrap())
        .add_param(ParameterSpec::new("self", TypeName::primitive("")).unwrap())
        .add_param(ParameterSpec::new("req", TypeName::primitive("Request")).unwrap())
        .returns(TypeName::primitive("Response"));
    // No body — should emit `...`

    let tb = TypeSpec::builder("BaseController", TypeKind::Class)
        .extends(abc)
        .add_method(handle.build().unwrap())
        .add_method(
            FunSpec::builder("log")
                .add_param(ParameterSpec::new("self", TypeName::primitive("")).unwrap())
                .returns(TypeName::primitive("None"))
                .body(CodeBlock::of("print('handled')", ()).unwrap())
                .build()
                .unwrap(),
        );

    let file = FileSpec::builder_with("controller.py", Python::new())
        .add_type(tb.build().unwrap())
        .build()
        .unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("python/abstract_class.py", &output);
}

#[test]
fn test_decorated_function() {
    let file = FileSpec::builder_with("views.py", Python::new())
        .add_function(
            FunSpec::builder("my_view")
                .annotation(CodeBlock::of("@app.route('/hello')", ()).unwrap())
                .returns(TypeName::primitive("str"))
                .body(CodeBlock::of("return 'Hello, World!'", ()).unwrap())
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("python/decorated_function.py", &output);
}
