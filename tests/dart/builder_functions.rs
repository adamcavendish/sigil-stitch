use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::dart::DartLang;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::modifiers::TypeKind;
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::spec::type_spec::TypeSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_async_function() {
    let user = TypeName::<DartLang>::importable("package:myapp/models/user.dart", "User");

    let body = CodeBlock::<DartLang>::of("return await api.fetchUser(id);", ()).unwrap();
    let mut fb_fun = FunSpec::<DartLang>::builder("fetchUser");
    fb_fun.returns(TypeName::primitive("Future<User>"));
    fb_fun.add_param(ParameterSpec::new("id", TypeName::primitive("String")).unwrap());
    fb_fun.body(body);
    let fun = fb_fun.build().unwrap();

    // Trigger User import.
    let trigger = CodeBlock::<DartLang>::of("// %T", (user,)).unwrap();

    let mut fb = FileSpec::builder_with("api.dart", DartLang::new());
    fb.add_code(trigger);
    fb.add_function(fun);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("dart/async_function.dart", &output);
}

#[test]
fn test_annotated_method() {
    let mut tb = TypeSpec::<DartLang>::builder("Dog", TypeKind::Class);
    tb.extends(TypeName::primitive("Animal"));

    let body = CodeBlock::<DartLang>::of(
        "return %S;",
        (sigil_stitch::code_block::StringLitArg("Woof!".to_string()),),
    )
    .unwrap();
    let mut speak = FunSpec::<DartLang>::builder("speak");
    speak.returns(TypeName::primitive("String"));
    speak.annotation(CodeBlock::<DartLang>::of("@override", ()).unwrap());
    speak.body(body);
    tb.add_method(speak.build().unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("dog.dart", DartLang::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("dart/annotated_method.dart", &output);
}

#[test]
fn test_function_with_doc() {
    let body = CodeBlock::<DartLang>::of("return 'Hello, $name!';", ()).unwrap();
    let mut fb = FunSpec::<DartLang>::builder("greet");
    fb.doc("Greet the user by name.");
    fb.add_param(ParameterSpec::new("name", TypeName::primitive("String")).unwrap());
    fb.returns(TypeName::primitive("String"));
    fb.body(body);
    let fun = fb.build().unwrap();

    let mut file_b = FileSpec::<DartLang>::builder("greet.dart");
    file_b.add_function(fun);
    let file = file_b.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("dart/function_with_doc.dart", &output);
}
