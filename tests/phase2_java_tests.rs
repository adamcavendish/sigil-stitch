mod golden;

use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::java_lang::JavaLang;
use sigil_stitch::spec::field_spec::FieldSpec;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::{FunSpec, TypeParamSpec};
use sigil_stitch::spec::modifiers::{TypeKind, Visibility};
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::spec::type_spec::TypeSpec;
use sigil_stitch::type_name::TypeName;

#[test]
fn test_java_class_with_methods() {
    let mut tb = TypeSpec::<JavaLang>::builder("UserService", TypeKind::Class);
    tb.visibility(Visibility::Public);
    tb.doc("Service for managing users.");

    // Private fields.
    let mut repo_field = FieldSpec::builder("repo", TypeName::primitive("UserRepository"));
    repo_field.visibility(Visibility::Private);
    tb.add_field(repo_field.build());

    let mut logger_field = FieldSpec::builder("logger", TypeName::primitive("Logger"));
    logger_field.visibility(Visibility::Private);
    logger_field.is_readonly();
    tb.add_field(logger_field.build());

    // Constructor.
    let ctor_body =
        CodeBlock::<JavaLang>::of("this.repo = repo;\nthis.logger = logger;", ()).unwrap();
    let mut ctor = FunSpec::<JavaLang>::builder("UserService");
    ctor.visibility(Visibility::Public);
    ctor.add_param(ParameterSpec::new(
        "repo",
        TypeName::primitive("UserRepository"),
    ));
    ctor.add_param(ParameterSpec::new("logger", TypeName::primitive("Logger")));
    ctor.body(ctor_body);
    tb.add_method(ctor.build());

    // Public method.
    let find_body = CodeBlock::<JavaLang>::of("return this.repo.findById(id);", ()).unwrap();
    let mut find = FunSpec::<JavaLang>::builder("findUser");
    find.visibility(Visibility::Public);
    find.returns(TypeName::primitive("User"));
    find.add_param(ParameterSpec::new("id", TypeName::primitive("String")));
    find.body(find_body);
    tb.add_method(find.build());

    let ts = tb.build();

    let mut fb = FileSpec::builder_with("UserService.java", JavaLang::new());
    fb.add_type(ts);
    let file = fb.build();
    let output = file.render(80).unwrap();

    golden::assert_golden("java/class_with_methods.java", &output);
}

#[test]
fn test_java_interface() {
    let tp = TypeParamSpec::<JavaLang>::new("T");

    let mut tb = TypeSpec::<JavaLang>::builder("Repository", TypeKind::Interface);
    tb.visibility(Visibility::Public);
    tb.add_type_param(tp);
    tb.doc("Generic data repository.");

    // Abstract methods (no body).
    let mut find = FunSpec::<JavaLang>::builder("findById");
    find.returns(TypeName::primitive("T"));
    find.add_param(ParameterSpec::new("id", TypeName::primitive("String")));
    tb.add_method(find.build());

    let mut save = FunSpec::<JavaLang>::builder("save");
    save.returns(TypeName::primitive("void"));
    save.add_param(ParameterSpec::new("entity", TypeName::primitive("T")));
    tb.add_method(save.build());

    let mut delete = FunSpec::<JavaLang>::builder("delete");
    delete.returns(TypeName::primitive("void"));
    delete.add_param(ParameterSpec::new("id", TypeName::primitive("String")));
    tb.add_method(delete.build());

    let ts = tb.build();

    let mut fb = FileSpec::builder_with("Repository.java", JavaLang::new());
    fb.add_type(ts);
    let file = fb.build();
    let output = file.render(80).unwrap();

    golden::assert_golden("java/interface.java", &output);
}

#[test]
fn test_java_abstract_class() {
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
    tb.add_method(desc.build());

    // Abstract method.
    let mut area = FunSpec::<JavaLang>::builder("area");
    area.visibility(Visibility::Public);
    area.is_abstract();
    area.returns(TypeName::primitive("double"));
    tb.add_method(area.build());

    // Mark class abstract via annotation-like prefix.
    tb.is_abstract();

    let ts = tb.build();

    let mut fb = FileSpec::builder_with("Shape.java", JavaLang::new());
    fb.add_type(ts);
    let file = fb.build();
    let output = file.render(80).unwrap();

    golden::assert_golden("java/abstract_class.java", &output);
}

#[test]
fn test_java_class_extends_implements() {
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
    tb.add_method(is_admin.build());

    let ts = tb.build();

    let mut fb = FileSpec::builder_with("AdminService.java", JavaLang::new());
    fb.add_type(ts);
    let file = fb.build();
    let output = file.render(80).unwrap();

    golden::assert_golden("java/class_extends_implements.java", &output);
}

#[test]
fn test_java_enum() {
    let mut tb = TypeSpec::<JavaLang>::builder("Color", TypeKind::Enum);
    tb.visibility(Visibility::Public);
    tb.doc("Supported colors.");

    let mut constants = CodeBlock::<JavaLang>::builder();
    constants.add("RED,", ());
    constants.add_line();
    constants.add("GREEN,", ());
    constants.add_line();
    constants.add("BLUE", ());
    constants.add_line();
    tb.extra_member(constants.build().unwrap());

    let ts = tb.build();

    let mut fb = FileSpec::builder_with("Color.java", JavaLang::new());
    fb.add_type(ts);
    let file = fb.build();
    let output = file.render(80).unwrap();

    golden::assert_golden("java/enum.java", &output);
}

#[test]
fn test_java_generic_class() {
    let tp = TypeParamSpec::<JavaLang>::new("T")
        .with_bound(TypeName::primitive("Comparable"))
        .with_bound(TypeName::primitive("Serializable"));

    let mut tb = TypeSpec::<JavaLang>::builder("SortedContainer", TypeKind::Class);
    tb.visibility(Visibility::Public);
    tb.add_type_param(tp);
    tb.doc("A sorted container with bounded type parameter.");

    let mut items_field = FieldSpec::builder("items", TypeName::primitive("List<T>"));
    items_field.visibility(Visibility::Private);
    tb.add_field(items_field.build());

    let add_body = CodeBlock::<JavaLang>::of("this.items.add(item);", ()).unwrap();
    let mut add = FunSpec::<JavaLang>::builder("add");
    add.visibility(Visibility::Public);
    add.returns(TypeName::primitive("void"));
    add.add_param(ParameterSpec::new("item", TypeName::primitive("T")));
    add.body(add_body);
    tb.add_method(add.build());

    let ts = tb.build();

    let mut fb = FileSpec::builder_with("SortedContainer.java", JavaLang::new());
    fb.add_type(ts);
    let file = fb.build();
    let output = file.render(80).unwrap();

    golden::assert_golden("java/generic_class.java", &output);
}

#[test]
fn test_java_static_final_field() {
    let mut tb = TypeSpec::<JavaLang>::builder("Constants", TypeKind::Class);
    tb.visibility(Visibility::Public);

    let mut max_field = FieldSpec::builder("MAX_SIZE", TypeName::primitive("int"));
    max_field.visibility(Visibility::Public);
    max_field.is_static();
    max_field.is_readonly();
    max_field.initializer(CodeBlock::<JavaLang>::of("100", ()).unwrap());
    tb.add_field(max_field.build());

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
    tb.add_field(name_field.build());

    let ts = tb.build();

    let mut fb = FileSpec::builder_with("Constants.java", JavaLang::new());
    fb.add_type(ts);
    let file = fb.build();
    let output = file.render(80).unwrap();

    golden::assert_golden("java/static_final_field.java", &output);
}

#[test]
fn test_java_annotated_method() {
    let mut tb = TypeSpec::<JavaLang>::builder("Dog", TypeKind::Class);
    tb.visibility(Visibility::Public);
    tb.extends(TypeName::primitive("Animal"));

    let body = CodeBlock::<JavaLang>::of("return \"Woof!\";", ()).unwrap();
    let mut speak = FunSpec::<JavaLang>::builder("speak");
    speak.visibility(Visibility::Public);
    speak.returns(TypeName::primitive("String"));
    speak.annotation(CodeBlock::<JavaLang>::of("@Override", ()).unwrap());
    speak.body(body);
    tb.add_method(speak.build());

    let ts = tb.build();

    let mut fb = FileSpec::builder_with("Dog.java", JavaLang::new());
    fb.add_type(ts);
    let file = fb.build();
    let output = file.render(80).unwrap();

    golden::assert_golden("java/annotated_method.java", &output);
}

#[test]
fn test_java_full_module() {
    let list = TypeName::<JavaLang>::importable("java.util", "List");
    let array_list = TypeName::<JavaLang>::importable("java.util", "ArrayList");
    let nullable = TypeName::<JavaLang>::importable("javax.annotation", "Nullable");

    // Interface.
    let mut iface = TypeSpec::<JavaLang>::builder("UserRepository", TypeKind::Interface);
    iface.visibility(Visibility::Public);

    let mut find = FunSpec::<JavaLang>::builder("findById");
    find.returns(TypeName::primitive("User"));
    find.add_param(ParameterSpec::new("id", TypeName::primitive("String")));
    find.annotation(CodeBlock::<JavaLang>::of("@%T", (nullable.clone(),)).unwrap());
    iface.add_method(find.build());

    let mut find_all = FunSpec::<JavaLang>::builder("findAll");
    find_all.returns(TypeName::primitive("List<User>"));
    iface.add_method(find_all.build());

    let iface_spec = iface.build();

    // Implementation class.
    let mut cls = TypeSpec::<JavaLang>::builder("InMemoryUserRepository", TypeKind::Class);
    cls.visibility(Visibility::Public);
    cls.implements(TypeName::primitive("UserRepository"));
    cls.doc("In-memory implementation of UserRepository.");

    let mut users_field = FieldSpec::builder("users", TypeName::primitive("List<User>"));
    users_field.visibility(Visibility::Private);
    users_field.is_readonly();
    cls.add_field(users_field.build());

    // Constructor — use imports.
    let ctor_body = CodeBlock::<JavaLang>::of("this.users = new %T<>();", (array_list,)).unwrap();
    let mut ctor = FunSpec::<JavaLang>::builder("InMemoryUserRepository");
    ctor.visibility(Visibility::Public);
    ctor.body(ctor_body);
    cls.add_method(ctor.build());

    // findById with @Nullable.
    let find_body = CodeBlock::<JavaLang>::of(
        "return this.users.stream()\n    .filter(u -> u.getId().equals(id))\n    .findFirst()\n    .orElse(null);",
        (),
    )
    .unwrap();
    let mut find_impl = FunSpec::<JavaLang>::builder("findById");
    find_impl.visibility(Visibility::Public);
    find_impl.returns(TypeName::primitive("User"));
    find_impl.add_param(ParameterSpec::new("id", TypeName::primitive("String")));
    find_impl.annotation(CodeBlock::<JavaLang>::of("@Override", ()).unwrap());
    find_impl.annotation(CodeBlock::<JavaLang>::of("@%T", (nullable,)).unwrap());
    find_impl.body(find_body);
    cls.add_method(find_impl.build());

    // findAll — trigger List import.
    let find_all_body =
        CodeBlock::<JavaLang>::of("return new %T<>(this.users);", (list.clone(),)).unwrap();
    let mut find_all_impl = FunSpec::<JavaLang>::builder("findAll");
    find_all_impl.visibility(Visibility::Public);
    find_all_impl.returns(TypeName::primitive("List<User>"));
    find_all_impl.annotation(CodeBlock::<JavaLang>::of("@Override", ()).unwrap());
    find_all_impl.body(find_all_body);
    cls.add_method(find_all_impl.build());

    let cls_spec = cls.build();

    let mut fb = FileSpec::builder_with("UserRepo.java", JavaLang::new());
    fb.add_type(iface_spec);
    fb.add_type(cls_spec);
    let file = fb.build();
    let output = file.render(80).unwrap();

    golden::assert_golden("java/full_module.java", &output);
}
