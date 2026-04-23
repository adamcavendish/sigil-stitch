use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::java_lang::JavaLang;
use sigil_stitch::spec::field_spec::FieldSpec;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::modifiers::{TypeKind, Visibility};
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::spec::type_spec::TypeSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_static_final_field() {
    let mut tb = TypeSpec::<JavaLang>::builder("Constants", TypeKind::Class);
    tb.visibility(Visibility::Public);

    let mut max_field = FieldSpec::builder("MAX_SIZE", TypeName::primitive("int"));
    max_field.visibility(Visibility::Public);
    max_field.is_static();
    max_field.is_readonly();
    max_field.initializer(CodeBlock::<JavaLang>::of("100", ()).unwrap());
    tb.add_field(max_field.build().unwrap());

    let mut name_field = FieldSpec::builder("APP_NAME", TypeName::primitive("String"));
    name_field.visibility(Visibility::Public);
    name_field.is_static();
    name_field.is_readonly();
    name_field.initializer(
        CodeBlock::<JavaLang>::of(
            "%S",
            (sigil_stitch::code_block::StringLitArg("MyApp".to_string()),),
        )
        .unwrap(),
    );
    tb.add_field(name_field.build().unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("Constants.java", JavaLang::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("java/static_final_field.java", &output);
}

#[test]
fn test_annotated_method() {
    let mut tb = TypeSpec::<JavaLang>::builder("Dog", TypeKind::Class);
    tb.visibility(Visibility::Public);
    tb.extends(TypeName::primitive("Animal"));

    let body = CodeBlock::<JavaLang>::of("return \"Woof!\";", ()).unwrap();
    let mut speak = FunSpec::<JavaLang>::builder("speak");
    speak.visibility(Visibility::Public);
    speak.returns(TypeName::primitive("String"));
    speak.annotation(CodeBlock::<JavaLang>::of("@Override", ()).unwrap());
    speak.body(body);
    tb.add_method(speak.build().unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("Dog.java", JavaLang::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("java/annotated_method.java", &output);
}

#[test]
fn test_full_module() {
    let list = TypeName::<JavaLang>::importable("java.util", "List");
    let array_list = TypeName::<JavaLang>::importable("java.util", "ArrayList");
    let nullable = TypeName::<JavaLang>::importable("javax.annotation", "Nullable");

    // Interface.
    let mut iface = TypeSpec::<JavaLang>::builder("UserRepository", TypeKind::Interface);
    iface.visibility(Visibility::Public);

    let mut find = FunSpec::<JavaLang>::builder("findById");
    find.returns(TypeName::primitive("User"));
    find.add_param(ParameterSpec::new("id", TypeName::primitive("String")).unwrap());
    find.annotation(CodeBlock::<JavaLang>::of("@%T", (nullable.clone(),)).unwrap());
    iface.add_method(find.build().unwrap());

    let mut find_all = FunSpec::<JavaLang>::builder("findAll");
    find_all.returns(TypeName::primitive("List<User>"));
    iface.add_method(find_all.build().unwrap());

    let iface_spec = iface.build().unwrap();

    // Implementation class.
    let mut cls = TypeSpec::<JavaLang>::builder("InMemoryUserRepository", TypeKind::Class);
    cls.visibility(Visibility::Public);
    cls.implements(TypeName::primitive("UserRepository"));
    cls.doc("In-memory implementation of UserRepository.");

    let mut users_field = FieldSpec::builder("users", TypeName::primitive("List<User>"));
    users_field.visibility(Visibility::Private);
    users_field.is_readonly();
    cls.add_field(users_field.build().unwrap());

    // Constructor — use imports.
    let ctor_body = CodeBlock::<JavaLang>::of("this.users = new %T<>();", (array_list,)).unwrap();
    let mut ctor = FunSpec::<JavaLang>::builder("InMemoryUserRepository");
    ctor.visibility(Visibility::Public);
    ctor.body(ctor_body);
    cls.add_method(ctor.build().unwrap());

    // findById with @Nullable.
    let find_body = CodeBlock::<JavaLang>::of(
        "return this.users.stream()\n    .filter(u -> u.getId().equals(id))\n    .findFirst()\n    .orElse(null);",
        (),
    )
    .unwrap();
    let mut find_impl = FunSpec::<JavaLang>::builder("findById");
    find_impl.visibility(Visibility::Public);
    find_impl.returns(TypeName::primitive("User"));
    find_impl.add_param(ParameterSpec::new("id", TypeName::primitive("String")).unwrap());
    find_impl.annotation(CodeBlock::<JavaLang>::of("@Override", ()).unwrap());
    find_impl.annotation(CodeBlock::<JavaLang>::of("@%T", (nullable,)).unwrap());
    find_impl.body(find_body);
    cls.add_method(find_impl.build().unwrap());

    // findAll — trigger List import.
    let find_all_body =
        CodeBlock::<JavaLang>::of("return new %T<>(this.users);", (list.clone(),)).unwrap();
    let mut find_all_impl = FunSpec::<JavaLang>::builder("findAll");
    find_all_impl.visibility(Visibility::Public);
    find_all_impl.returns(TypeName::primitive("List<User>"));
    find_all_impl.annotation(CodeBlock::<JavaLang>::of("@Override", ()).unwrap());
    find_all_impl.body(find_all_body);
    cls.add_method(find_all_impl.build().unwrap());

    let cls_spec = cls.build().unwrap();

    let mut fb = FileSpec::builder_with("UserRepo.java", JavaLang::new());
    fb.add_type(iface_spec);
    fb.add_type(cls_spec);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("java/full_module.java", &output);
}
