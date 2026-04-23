use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::rust_lang::RustLang;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::{FunSpec, TypeParamSpec};
use sigil_stitch::spec::modifiers::Visibility;
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_top_level_function() {
    let tp =
        TypeParamSpec::<RustLang>::new("T").with_bound(TypeName::primitive("std::fmt::Display"));

    let mut fb = FunSpec::<RustLang>::builder("print_value");
    fb.visibility(Visibility::Public);
    fb.add_type_param(tp);
    fb.add_param(ParameterSpec::new("value", TypeName::primitive("&T")).unwrap());
    let body = CodeBlock::<RustLang>::of("println!(\"{}\", value)", ()).unwrap();
    fb.body(body);

    let mut file = FileSpec::builder_with("utils.rs", RustLang::new());
    file.add_function(fb.build().unwrap());
    let output = file.build().unwrap().render(80).unwrap();

    golden::assert_golden("rust/top_level_function.rs", &output);
}

#[test]
fn test_function_with_doc() {
    let mut fb = FunSpec::<RustLang>::builder("greet");
    fb.visibility(Visibility::Public);
    fb.doc("Greet the user by name.");
    fb.add_param(ParameterSpec::new("name", TypeName::primitive("&str")).unwrap());
    fb.returns(TypeName::primitive("String"));
    let body = CodeBlock::<RustLang>::of("format!(\"Hello, {}!\", name)", ()).unwrap();
    fb.body(body);

    let mut file = FileSpec::builder_with("greet.rs", RustLang::new());
    file.add_function(fb.build().unwrap());
    let output = file.build().unwrap().render(80).unwrap();

    golden::assert_golden("rust/function_with_doc.rs", &output);
}
