use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::scala::Scala;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::modifiers::Visibility;
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_function_with_return() {
    let user = TypeName::<Scala>::importable("com.example.model", "User");

    let body = CodeBlock::<Scala>::of("api.fetchUser(id)", ()).unwrap();
    let mut fb_fun = FunSpec::<Scala>::builder("fetchUser");
    fb_fun.returns(user);
    fb_fun.add_param(ParameterSpec::new("id", TypeName::primitive("String")).unwrap());
    fb_fun.body(body);
    let fun = fb_fun.build().unwrap();

    let mut fb = FileSpec::builder_with("Api.scala", Scala::new());
    fb.add_function(fun);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("scala/function_with_return.scala", &output);
}

#[test]
fn test_function_with_doc() {
    let body = CodeBlock::<Scala>::of("s\"Hello, $name!\"", ()).unwrap();
    let mut fb = FunSpec::<Scala>::builder("greet");
    fb.visibility(Visibility::Public);
    fb.doc("Greet the user by name.");
    fb.add_param(ParameterSpec::new("name", TypeName::primitive("String")).unwrap());
    fb.returns(TypeName::primitive("String"));
    fb.body(body);
    let fun = fb.build().unwrap();

    let mut file_b = FileSpec::<Scala>::builder("greet.scala");
    file_b.add_function(fun);
    let file = file_b.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("scala/function_with_doc.scala", &output);
}
