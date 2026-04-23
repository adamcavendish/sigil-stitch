use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::kotlin::Kotlin;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::modifiers::Visibility;
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_suspend_function() {
    let user = TypeName::<Kotlin>::importable("com.example.model", "User");

    let body = CodeBlock::<Kotlin>::of("return api.fetchUser(id)", ()).unwrap();
    let mut fb_fun = FunSpec::<Kotlin>::builder("fetchUser");
    fb_fun.is_async();
    fb_fun.returns(user);
    fb_fun.add_param(ParameterSpec::new("id", TypeName::primitive("String")).unwrap());
    fb_fun.body(body);
    let fun = fb_fun.build().unwrap();

    let mut fb = FileSpec::builder_with("Api.kt", Kotlin::new());
    fb.add_function(fun);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("kotlin/suspend_function.kt", &output);
}

#[test]
fn test_function_with_doc() {
    let body = CodeBlock::<Kotlin>::of("return \"Hello, $name!\"", ()).unwrap();
    let mut fb = FunSpec::<Kotlin>::builder("greet");
    fb.visibility(Visibility::Public);
    fb.doc("Greet the user by name.");
    fb.add_param(ParameterSpec::new("name", TypeName::primitive("String")).unwrap());
    fb.returns(TypeName::primitive("String"));
    fb.body(body);
    let fun = fb.build().unwrap();

    let mut file_b = FileSpec::<Kotlin>::builder("greet.kt");
    file_b.add_function(fun);
    let file = file_b.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("kotlin/function_with_doc.kt", &output);
}
