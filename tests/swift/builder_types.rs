use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::swift::Swift;
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
    let mut tb = TypeSpec::<Swift>::builder("UserService", TypeKind::Class);
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
    let find_body = CodeBlock::<Swift>::of("return repo.find(by: id)", ()).unwrap();
    let mut find = FunSpec::<Swift>::builder("findUser");
    find.visibility(Visibility::Public);
    find.returns(TypeName::primitive("User?"));
    find.add_param(ParameterSpec::new("id", TypeName::primitive("String")).unwrap());
    find.body(find_body);
    tb.add_method(find.build().unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("UserService.swift", Swift::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("swift/class_with_properties.swift", &output);
}

#[test]
fn test_struct() {
    let mut tb = TypeSpec::<Swift>::builder("User", TypeKind::Struct);
    tb.visibility(Visibility::Public);
    tb.doc("A user value type.");

    let mut name_field = FieldSpec::builder("name", TypeName::primitive("String"));
    name_field.visibility(Visibility::Public);
    name_field.is_readonly();
    tb.add_field(name_field.build().unwrap());

    let mut age_field = FieldSpec::builder("age", TypeName::primitive("Int"));
    age_field.visibility(Visibility::Public);
    age_field.is_readonly();
    tb.add_field(age_field.build().unwrap());

    let mut email_field = FieldSpec::builder("email", TypeName::primitive("String?"));
    email_field.visibility(Visibility::Public);
    tb.add_field(email_field.build().unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("User.swift", Swift::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("swift/struct.swift", &output);
}

#[test]
fn test_protocol() {
    let tp = TypeParamSpec::<Swift>::new("T");

    let mut tb = TypeSpec::<Swift>::builder("Repository", TypeKind::Interface);
    tb.add_type_param(tp);
    tb.doc("Generic data repository.");

    // Protocol method requirements (no body).
    let mut find = FunSpec::<Swift>::builder("findById");
    find.returns(TypeName::primitive("T?"));
    find.add_param(ParameterSpec::new("id", TypeName::primitive("String")).unwrap());
    tb.add_method(find.build().unwrap());

    let mut save = FunSpec::<Swift>::builder("save");
    save.add_param(ParameterSpec::new("entity", TypeName::primitive("T")).unwrap());
    tb.add_method(save.build().unwrap());

    let mut delete = FunSpec::<Swift>::builder("delete");
    delete.add_param(ParameterSpec::new("id", TypeName::primitive("String")).unwrap());
    tb.add_method(delete.build().unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("Repository.swift", Swift::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("swift/protocol.swift", &output);
}

#[test]
fn test_abstract_class() {
    let mut tb = TypeSpec::<Swift>::builder("Shape", TypeKind::Class);
    tb.doc("Abstract shape base class.");

    // Concrete method.
    let desc_body =
        CodeBlock::<Swift>::of("return String(describing: type(of: self))", ()).unwrap();
    let mut desc = FunSpec::<Swift>::builder("describe");
    desc.returns(TypeName::primitive("String"));
    desc.body(desc_body);
    tb.add_method(desc.build().unwrap());

    // Abstract-like method (fatalError convention).
    let area_body = CodeBlock::<Swift>::of("fatalError(\"Subclasses must override\")", ()).unwrap();
    let mut area = FunSpec::<Swift>::builder("area");
    area.returns(TypeName::primitive("Double"));
    area.body(area_body);
    tb.add_method(area.build().unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("Shape.swift", Swift::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("swift/abstract_class.swift", &output);
}

#[test]
fn test_class_extends_implements() {
    let base = TypeName::<Swift>::importable("MyModule", "BaseService");
    let codable = TypeName::<Swift>::importable("Foundation", "Codable");
    let hashable = TypeName::<Swift>::primitive("Hashable");

    // Swift uses `:` for both superclass and protocol conformance.
    let mut tb = TypeSpec::<Swift>::builder("AdminService", TypeKind::Class);
    tb.extends(base);
    tb.extends(codable);
    tb.extends(hashable);

    let body = CodeBlock::<Swift>::of("return true", ()).unwrap();
    let mut is_admin = FunSpec::<Swift>::builder("isAdmin");
    is_admin.returns(TypeName::primitive("Bool"));
    is_admin.body(body);
    tb.add_method(is_admin.build().unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("AdminService.swift", Swift::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("swift/class_extends_implements.swift", &output);
}

#[test]
fn test_enum() {
    let mut tb = TypeSpec::<Swift>::builder("Color", TypeKind::Enum);
    tb.visibility(Visibility::Public);
    tb.doc("Supported colors.");

    tb.add_variant(EnumVariantSpec::new("red").unwrap());
    tb.add_variant(EnumVariantSpec::new("green").unwrap());
    tb.add_variant(EnumVariantSpec::new("blue").unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("Color.swift", Swift::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("swift/enum.swift", &output);
}

#[test]
fn test_enum_associated_values() {
    let mut tb = TypeSpec::<Swift>::builder("NetworkResult", TypeKind::Enum);
    tb.visibility(Visibility::Public);
    tb.doc("Result of a network request.");

    // case success(Data)
    let mut v_success = EnumVariantSpec::<Swift>::builder("success");
    v_success.associated_type(TypeName::primitive("Data"));
    tb.add_variant(v_success.build().unwrap());

    // case failure(Error, Int) — multi-element associated value
    let mut v_failure = EnumVariantSpec::<Swift>::builder("failure");
    v_failure.associated_type(TypeName::primitive("Error"));
    v_failure.associated_type(TypeName::primitive("Int"));
    tb.add_variant(v_failure.build().unwrap());

    // case loading — simple variant
    tb.add_variant(EnumVariantSpec::new("loading").unwrap());

    let mut fb = FileSpec::builder_with("NetworkResult.swift", Swift::new());
    fb.add_type(tb.build().unwrap());
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("swift/enum_associated.swift", &output);
}
