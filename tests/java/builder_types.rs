use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::java_lang::JavaLang;
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
fn test_class_with_methods() {
    let mut tb = TypeSpec::<JavaLang>::builder("UserService", TypeKind::Class);
    tb.visibility(Visibility::Public);
    tb.doc("Service for managing users.");

    // Private fields.
    let mut repo_field = FieldSpec::builder("repo", TypeName::primitive("UserRepository"));
    repo_field.visibility(Visibility::Private);
    tb.add_field(repo_field.build().unwrap());

    let mut logger_field = FieldSpec::builder("logger", TypeName::primitive("Logger"));
    logger_field.visibility(Visibility::Private);
    logger_field.is_readonly();
    tb.add_field(logger_field.build().unwrap());

    // Constructor.
    let ctor_body =
        CodeBlock::<JavaLang>::of("this.repo = repo;\nthis.logger = logger;", ()).unwrap();
    let mut ctor = FunSpec::<JavaLang>::builder("UserService");
    ctor.visibility(Visibility::Public);
    ctor.add_param(ParameterSpec::new("repo", TypeName::primitive("UserRepository")).unwrap());
    ctor.add_param(ParameterSpec::new("logger", TypeName::primitive("Logger")).unwrap());
    ctor.body(ctor_body);
    tb.add_method(ctor.build().unwrap());

    // Public method.
    let find_body = CodeBlock::<JavaLang>::of("return this.repo.findById(id);", ()).unwrap();
    let mut find = FunSpec::<JavaLang>::builder("findUser");
    find.visibility(Visibility::Public);
    find.returns(TypeName::primitive("User"));
    find.add_param(ParameterSpec::new("id", TypeName::primitive("String")).unwrap());
    find.body(find_body);
    tb.add_method(find.build().unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("UserService.java", JavaLang::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("java/class_with_methods.java", &output);
}

#[test]
fn test_interface() {
    let tp = TypeParamSpec::<JavaLang>::new("T");

    let mut tb = TypeSpec::<JavaLang>::builder("Repository", TypeKind::Interface);
    tb.visibility(Visibility::Public);
    tb.add_type_param(tp);
    tb.doc("Generic data repository.");

    // Abstract methods (no body).
    let mut find = FunSpec::<JavaLang>::builder("findById");
    find.returns(TypeName::primitive("T"));
    find.add_param(ParameterSpec::new("id", TypeName::primitive("String")).unwrap());
    tb.add_method(find.build().unwrap());

    let mut save = FunSpec::<JavaLang>::builder("save");
    save.returns(TypeName::primitive("void"));
    save.add_param(ParameterSpec::new("entity", TypeName::primitive("T")).unwrap());
    tb.add_method(save.build().unwrap());

    let mut delete = FunSpec::<JavaLang>::builder("delete");
    delete.returns(TypeName::primitive("void"));
    delete.add_param(ParameterSpec::new("id", TypeName::primitive("String")).unwrap());
    tb.add_method(delete.build().unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("Repository.java", JavaLang::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("java/interface.java", &output);
}

#[test]
fn test_abstract_class() {
    let mut tb = TypeSpec::<JavaLang>::builder("Shape", TypeKind::Class);
    tb.visibility(Visibility::Public);
    tb.doc("Abstract shape.");

    // Concrete method.
    let desc_body =
        CodeBlock::<JavaLang>::of("return this.getClass().getSimpleName();", ()).unwrap();
    let mut desc = FunSpec::<JavaLang>::builder("describe");
    desc.visibility(Visibility::Public);
    desc.returns(TypeName::primitive("String"));
    desc.body(desc_body);
    tb.add_method(desc.build().unwrap());

    // Abstract method.
    let mut area = FunSpec::<JavaLang>::builder("area");
    area.visibility(Visibility::Public);
    area.is_abstract();
    area.returns(TypeName::primitive("double"));
    tb.add_method(area.build().unwrap());

    // Mark class abstract via annotation-like prefix.
    tb.is_abstract();

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("Shape.java", JavaLang::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("java/abstract_class.java", &output);
}

#[test]
fn test_class_extends_implements() {
    let base = TypeName::<JavaLang>::importable("com.example.base", "BaseService");
    let auth = TypeName::<JavaLang>::importable("com.example.auth", "Authenticatable");
    let serial = TypeName::<JavaLang>::importable("com.example.serial", "Serializable");

    let mut tb = TypeSpec::<JavaLang>::builder("AdminService", TypeKind::Class);
    tb.visibility(Visibility::Public);
    tb.extends(base);
    tb.implements(auth);
    tb.implements(serial);

    let body = CodeBlock::<JavaLang>::of("return true;", ()).unwrap();
    let mut is_admin = FunSpec::<JavaLang>::builder("isAdmin");
    is_admin.visibility(Visibility::Public);
    is_admin.returns(TypeName::primitive("boolean"));
    is_admin.body(body);
    tb.add_method(is_admin.build().unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("AdminService.java", JavaLang::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("java/class_extends_implements.java", &output);
}

#[test]
fn test_enum() {
    let mut tb = TypeSpec::<JavaLang>::builder("Color", TypeKind::Enum);
    tb.visibility(Visibility::Public);
    tb.doc("Supported colors.");

    tb.add_variant(EnumVariantSpec::new("RED").unwrap());
    tb.add_variant(EnumVariantSpec::new("GREEN").unwrap());
    tb.add_variant(EnumVariantSpec::new("BLUE").unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("Color.java", JavaLang::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("java/enum.java", &output);
}

#[test]
fn test_generic_class() {
    let tp = TypeParamSpec::<JavaLang>::new("T")
        .with_bound(TypeName::primitive("Comparable"))
        .with_bound(TypeName::primitive("Serializable"));

    let mut tb = TypeSpec::<JavaLang>::builder("SortedContainer", TypeKind::Class);
    tb.visibility(Visibility::Public);
    tb.add_type_param(tp);
    tb.doc("A sorted container with bounded type parameter.");

    let mut items_field = FieldSpec::builder("items", TypeName::primitive("List<T>"));
    items_field.visibility(Visibility::Private);
    tb.add_field(items_field.build().unwrap());

    let add_body = CodeBlock::<JavaLang>::of("this.items.add(item);", ()).unwrap();
    let mut add = FunSpec::<JavaLang>::builder("add");
    add.visibility(Visibility::Public);
    add.returns(TypeName::primitive("void"));
    add.add_param(ParameterSpec::new("item", TypeName::primitive("T")).unwrap());
    add.body(add_body);
    tb.add_method(add.build().unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("SortedContainer.java", JavaLang::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("java/generic_class.java", &output);
}
