use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::swift::Swift;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::modifiers::{TypeKind, Visibility};
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::spec::type_spec::TypeSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_async_function() {
    let user = TypeName::<Swift>::importable("MyModule", "User");

    let body = CodeBlock::<Swift>::of("return try await api.fetchUser(id: id)", ()).unwrap();
    let mut fb_fun = FunSpec::<Swift>::builder("fetchUser");
    fb_fun.is_async();
    fb_fun.returns(user);
    fb_fun.add_param(ParameterSpec::new("id", TypeName::primitive("String")).unwrap());
    fb_fun.body(body);
    let fun = fb_fun.build().unwrap();

    let mut fb = FileSpec::builder_with("Api.swift", Swift::new());
    fb.add_function(fun);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("swift/async_function.swift", &output);
}

#[test]
fn test_override_method() {
    let mut tb = TypeSpec::<Swift>::builder("Dog", TypeKind::Class);
    tb.extends(TypeName::primitive("Animal"));

    let body = CodeBlock::<Swift>::of(
        "return %S",
        (sigil_stitch::code_block::StringLitArg("Woof!".to_string()),),
    )
    .unwrap();
    let mut speak = FunSpec::<Swift>::builder("speak");
    speak.returns(TypeName::primitive("String"));
    speak.is_override();
    speak.body(body);
    tb.add_method(speak.build().unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("Dog.swift", Swift::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("swift/override_method.swift", &output);
}

#[test]
fn test_function_with_doc() {
    let body = CodeBlock::<Swift>::of("return \"Hello, \\(name)!\"", ()).unwrap();
    let mut fb = FunSpec::<Swift>::builder("greet");
    fb.visibility(Visibility::Public);
    fb.doc("Greet the user by name.");
    fb.add_param(ParameterSpec::new("name", TypeName::primitive("String")).unwrap());
    fb.returns(TypeName::primitive("String"));
    fb.body(body);
    let fun = fb.build().unwrap();

    let mut file_b = FileSpec::<Swift>::builder("greet.swift");
    file_b.add_function(fun);
    let file = file_b.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("swift/function_with_doc.swift", &output);
}
