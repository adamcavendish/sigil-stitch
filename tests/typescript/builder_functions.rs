use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::typescript::TypeScript;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::{FunSpec, TypeParamSpec};
use sigil_stitch::spec::modifiers::Visibility;
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_top_level_function() {
    let tp = TypeParamSpec::<TypeScript>::new("T").with_bound(TypeName::primitive("Serializable"));

    let mut fb = FunSpec::<TypeScript>::builder("serialize");
    fb.visibility(Visibility::Public);
    fb.add_type_param(tp);
    fb.add_param(ParameterSpec::new("value", TypeName::primitive("T")).unwrap());
    fb.returns(TypeName::primitive("string"));
    let body = CodeBlock::<TypeScript>::of("return JSON.stringify(value)", ()).unwrap();
    fb.body(body);

    let mut file = FileSpec::<TypeScript>::builder("serialize.ts");
    file.add_function(fb.build().unwrap());
    let output = file.build().unwrap().render(80).unwrap();

    golden::assert_golden("typescript/top_level_function.ts", &output);
}

#[test]
fn test_function_with_doc() {
    let mut fb = FunSpec::<TypeScript>::builder("greet");
    fb.visibility(Visibility::Public);
    fb.doc("Greet the user by name.");
    fb.add_param(ParameterSpec::new("name", TypeName::primitive("string")).unwrap());
    fb.returns(TypeName::primitive("string"));
    let body = CodeBlock::<TypeScript>::of("return `Hello, ${name}!`", ()).unwrap();
    fb.body(body);

    let mut file = FileSpec::<TypeScript>::builder("greet.ts");
    file.add_function(fb.build().unwrap());
    let output = file.build().unwrap().render(80).unwrap();

    golden::assert_golden("typescript/function_with_doc.ts", &output);
}
