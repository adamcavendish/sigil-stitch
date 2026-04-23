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
    let body = CodeBlock::<JavaLang>::of("return \"Hello, \" + name;", ()).unwrap();
    let mut fb = FunSpec::<JavaLang>::builder("greet");
    fb.visibility(Visibility::Public);
    fb.doc("Greet the user by name.");
    fb.add_param(ParameterSpec::new("name", TypeName::primitive("String")).unwrap());
    fb.returns(TypeName::primitive("String"));
    fb.body(body);
    let fun = fb.build().unwrap();

    let mut file_b = FileSpec::builder_with("Greet.java", JavaLang::new());
    file_b.add_function(fun);
    let file = file_b.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("java/function_with_doc.java", &output);
}
