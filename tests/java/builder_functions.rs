use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::java_lang::JavaLang;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::modifiers::Visibility;
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_function_with_doc() {
    let body = CodeBlock::of("return \"Hello, \" + name;", ()).unwrap();
    let fun = FunSpec::builder("greet")
        .visibility(Visibility::Public)
        .doc("Greet the user by name.")
        .add_param(ParameterSpec::new("name", TypeName::primitive("String")).unwrap())
        .returns(TypeName::primitive("String"))
        .body(body)
        .build()
        .unwrap();

    let file = FileSpec::builder_with("Greet.java", JavaLang::new())
        .add_function(fun)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("java/function_with_doc.java", &output);
}
