use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::go_lang::GoLang;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::modifiers::Visibility;
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_top_level_function() {
    let fmt_sprintf = TypeName::<GoLang>::importable("fmt", "Sprintf");

    let mut fb = FunSpec::<GoLang>::builder("Greet");
    fb.add_param(ParameterSpec::new("name", TypeName::primitive("string")).unwrap());
    fb.returns(TypeName::primitive("string"));
    fb.body(CodeBlock::<GoLang>::of("return %T(\"Hello, %%s!\", name)", (fmt_sprintf,)).unwrap());

    let mut file_b = FileSpec::builder_with("greet.go", GoLang::new());
    file_b.header(CodeBlock::<GoLang>::of("package greet", ()).unwrap());
    file_b.add_function(fb.build().unwrap());
    let file = file_b.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("go/top_level_function.go", &output);
}

#[test]
fn test_function_with_doc() {
    let body = CodeBlock::<GoLang>::of("return a + b", ()).unwrap();
    let mut fb = FunSpec::<GoLang>::builder("Add");
    fb.visibility(Visibility::Public);
    fb.doc("Add returns the sum of two integers.");
    fb.add_param(ParameterSpec::new("a", TypeName::primitive("int")).unwrap());
    fb.add_param(ParameterSpec::new("b", TypeName::primitive("int")).unwrap());
    fb.returns(TypeName::primitive("int"));
    fb.body(body);
    let fun = fb.build().unwrap();

    let mut file_b = FileSpec::builder_with("add.go", GoLang::new());
    file_b.add_function(fun);
    let file = file_b.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("go/function_with_doc.go", &output);
}
