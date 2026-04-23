use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::kotlin::Kotlin;
use sigil_stitch::spec::enum_variant_spec::EnumVariantSpec;
use sigil_stitch::spec::field_spec::FieldSpec;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::{FunSpec, TypeParamSpec};
use sigil_stitch::spec::modifiers::{TypeKind, Visibility};
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::spec::type_spec::TypeSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_class_with_properties() {
    let mut tb = TypeSpec::<Kotlin>::builder("UserService", TypeKind::Class);
    tb.visibility(Visibility::Public);
    tb.doc("Service for managing users.");

    // Properties.
    let mut repo_field = FieldSpec::builder("repo", TypeName::primitive("UserRepository"));
    repo_field.visibility(Visibility::Private);
    tb.add_field(repo_field.build().unwrap());

    let mut logger_field = FieldSpec::builder("logger", TypeName::primitive("Logger"));
    logger_field.visibility(Visibility::Private);
    logger_field.is_readonly();
    tb.add_field(logger_field.build().unwrap());

    // Method.
    let find_body = CodeBlock::<Kotlin>::of("return repo.findById(id)", ()).unwrap();
    let mut find = FunSpec::<Kotlin>::builder("findUser");
    find.returns(TypeName::primitive("User"));
    find.add_param(ParameterSpec::new("id", TypeName::primitive("String")).unwrap());
    find.body(find_body);
    tb.add_method(find.build().unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("UserService.kt", Kotlin::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("kotlin/class_with_properties.kt", &output);
}

#[test]
fn test_data_class() {
    let mut tb = TypeSpec::<Kotlin>::builder("User", TypeKind::Struct);
    tb.visibility(Visibility::Public);
    tb.doc("A user data class.");

    tb.add_primary_constructor_param(
        ParameterSpec::new("name", TypeName::primitive("String")).unwrap(),
    );
    tb.add_primary_constructor_param(
        ParameterSpec::new("age", TypeName::primitive("Int")).unwrap(),
    );
    tb.add_primary_constructor_param(
        ParameterSpec::new("email", TypeName::primitive("String")).unwrap(),
    );

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("User.kt", Kotlin::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("kotlin/data_class.kt", &output);
}

#[test]
fn test_interface() {
    let tp = TypeParamSpec::<Kotlin>::new("T");

    let mut tb = TypeSpec::<Kotlin>::builder("Repository", TypeKind::Interface);
    tb.add_type_param(tp);
    tb.doc("Generic data repository.");

    // Abstract methods (no body).
    let mut find = FunSpec::<Kotlin>::builder("findById");
    find.returns(TypeName::primitive("T?"));
    find.add_param(ParameterSpec::new("id", TypeName::primitive("String")).unwrap());
    tb.add_method(find.build().unwrap());

    let mut save = FunSpec::<Kotlin>::builder("save");
    save.add_param(ParameterSpec::new("entity", TypeName::primitive("T")).unwrap());
    tb.add_method(save.build().unwrap());

    let mut delete = FunSpec::<Kotlin>::builder("delete");
    delete.add_param(ParameterSpec::new("id", TypeName::primitive("String")).unwrap());
    tb.add_method(delete.build().unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("Repository.kt", Kotlin::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("kotlin/interface.kt", &output);
}

#[test]
fn test_abstract_class() {
    let mut tb = TypeSpec::<Kotlin>::builder("Shape", TypeKind::Class);
    tb.doc("Abstract shape.");
    tb.is_abstract();

    // Concrete method.
    let desc_body =
        CodeBlock::<Kotlin>::of("return this::class.simpleName ?: \"Shape\"", ()).unwrap();
    let mut desc = FunSpec::<Kotlin>::builder("describe");
    desc.returns(TypeName::primitive("String"));
    desc.body(desc_body);
    tb.add_method(desc.build().unwrap());

    // Abstract method.
    let mut area = FunSpec::<Kotlin>::builder("area");
    area.is_abstract();
    area.returns(TypeName::primitive("Double"));
    tb.add_method(area.build().unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("Shape.kt", Kotlin::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("kotlin/abstract_class.kt", &output);
}

#[test]
fn test_class_extends_implements() {
    let base = TypeName::<Kotlin>::importable("com.example.base", "BaseService");
    let auth = TypeName::<Kotlin>::importable("com.example.auth", "Authenticatable");
    let serial = TypeName::<Kotlin>::importable("com.example.serial", "Serializable");

    // Kotlin uses `:` for both extends and implements.
    // Put everything in super_types.
    let mut tb = TypeSpec::<Kotlin>::builder("AdminService", TypeKind::Class);
    tb.extends(base);
    tb.extends(auth);
    tb.extends(serial);

    let body = CodeBlock::<Kotlin>::of("return true", ()).unwrap();
    let mut is_admin = FunSpec::<Kotlin>::builder("isAdmin");
    is_admin.returns(TypeName::primitive("Boolean"));
    is_admin.body(body);
    tb.add_method(is_admin.build().unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("AdminService.kt", Kotlin::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("kotlin/class_extends_implements.kt", &output);
}

#[test]
fn test_enum_class() {
    let mut tb = TypeSpec::<Kotlin>::builder("Color", TypeKind::Enum);
    tb.doc("Supported colors.");

    tb.add_variant(EnumVariantSpec::new("RED").unwrap());
    tb.add_variant(EnumVariantSpec::new("GREEN").unwrap());
    tb.add_variant(EnumVariantSpec::new("BLUE").unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("Color.kt", Kotlin::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("kotlin/enum_class.kt", &output);
}

#[test]
fn test_override_method() {
    let mut tb = TypeSpec::<Kotlin>::builder("Dog", TypeKind::Class);
    tb.extends(TypeName::primitive("Animal"));

    let body = CodeBlock::<Kotlin>::of(
        "return %S",
        (sigil_stitch::code_block::StringLitArg("Woof!".to_string()),),
    )
    .unwrap();
    let mut speak = FunSpec::<Kotlin>::builder("speak");
    speak.returns(TypeName::primitive("String"));
    speak.is_override();
    speak.body(body);
    tb.add_method(speak.build().unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("Dog.kt", Kotlin::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("kotlin/override_method.kt", &output);
}
