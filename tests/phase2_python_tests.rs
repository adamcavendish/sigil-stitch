use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::python::Python;
use sigil_stitch::spec::field_spec::FieldSpec;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::modifiers::TypeKind;
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::spec::type_spec::TypeSpec;
use sigil_stitch::type_name::TypeName;

mod golden;

// === Dataclass with typed fields ===

#[test]
fn test_python_dataclass() {
    let mut tb = TypeSpec::<Python>::builder("Config", TypeKind::Class);
    tb.doc("Application configuration.");
    tb.annotation(CodeBlock::<Python>::of("@dataclass", ()).unwrap());

    tb.add_field(FieldSpec::builder("name", TypeName::primitive("str")).build());
    tb.add_field(FieldSpec::builder("port", TypeName::primitive("int")).build());

    let mut f3 = FieldSpec::builder("debug", TypeName::primitive("bool"));
    f3.initializer(CodeBlock::<Python>::of("False", ()).unwrap());
    tb.add_field(f3.build());

    let mut fb = FileSpec::builder_with("config.py", Python::new());
    fb.add_type(tb.build());
    let file = fb.build();

    let output = file.render(80).unwrap();
    golden::assert_golden("python/dataclass.py", &output);
}

// === Class with methods and self ===

#[test]
fn test_python_class_with_methods() {
    let mut tb = TypeSpec::<Python>::builder("UserService", TypeKind::Class);
    tb.doc("Service for managing users.");

    tb.add_field(FieldSpec::builder("_repo", TypeName::primitive("UserRepository")).build());

    let mut get_user = FunSpec::<Python>::builder("get_user");
    get_user.add_param(ParameterSpec::new("self", TypeName::primitive("")));
    get_user.add_param(ParameterSpec::new("user_id", TypeName::primitive("str")));
    get_user.returns(TypeName::primitive("User"));
    get_user.body(CodeBlock::<Python>::of("return self._repo.find(user_id)", ()).unwrap());
    tb.add_method(get_user.build());

    let mut save_user = FunSpec::<Python>::builder("save_user");
    save_user.add_param(ParameterSpec::new("self", TypeName::primitive("")));
    save_user.add_param(ParameterSpec::new("user", TypeName::primitive("User")));
    save_user.returns(TypeName::primitive("None"));
    save_user.body(CodeBlock::<Python>::of("self._repo.save(user)", ()).unwrap());
    tb.add_method(save_user.build());

    let mut fb = FileSpec::builder_with("service.py", Python::new());
    fb.add_type(tb.build());
    let file = fb.build();

    let output = file.render(80).unwrap();
    golden::assert_golden("python/class_with_methods.py", &output);
}

// === Class with base classes ===

#[test]
fn test_python_class_with_bases() {
    let base = TypeName::<Python>::primitive("BaseService");
    let auth = TypeName::<Python>::primitive("Authenticatable");

    let mut tb = TypeSpec::<Python>::builder("AdminService", TypeKind::Class);
    tb.extends(base);
    tb.implements(auth);

    let mut method = FunSpec::<Python>::builder("is_admin");
    method.add_param(ParameterSpec::new("self", TypeName::primitive("")));
    method.returns(TypeName::primitive("bool"));
    method.body(CodeBlock::<Python>::of("return True", ()).unwrap());
    tb.add_method(method.build());

    let mut fb = FileSpec::builder_with("admin.py", Python::new());
    fb.add_type(tb.build());
    let file = fb.build();

    let output = file.render(80).unwrap();
    golden::assert_golden("python/class_with_bases.py", &output);
}

// === Protocol (TypeKind::Interface) ===

#[test]
fn test_python_protocol() {
    let protocol = TypeName::<Python>::importable("typing", "Protocol");

    let mut tb = TypeSpec::<Python>::builder("Repository", TypeKind::Interface);
    tb.doc("Repository defines data access methods.");
    tb.extends(protocol);

    let mut find = FunSpec::<Python>::builder("find_by_id");
    find.add_param(ParameterSpec::new("self", TypeName::primitive("")));
    find.add_param(ParameterSpec::new("id", TypeName::primitive("str")));
    find.returns(TypeName::primitive("Entity"));
    tb.add_method(find.build());

    let mut save = FunSpec::<Python>::builder("save");
    save.add_param(ParameterSpec::new("self", TypeName::primitive("")));
    save.add_param(ParameterSpec::new("entity", TypeName::primitive("Entity")));
    save.returns(TypeName::primitive("None"));
    tb.add_method(save.build());

    let mut fb = FileSpec::builder_with("repo.py", Python::new());
    fb.add_type(tb.build());
    let file = fb.build();

    let output = file.render(80).unwrap();
    golden::assert_golden("python/protocol.py", &output);
}

// === Top-level function with type hints and imports ===

#[test]
fn test_python_top_level_function() {
    let json_dumps = TypeName::<Python>::importable("json", "dumps");

    let mut fb = FunSpec::<Python>::builder("serialize");
    fb.doc("Serialize an object to JSON.");
    fb.add_param(ParameterSpec::new("value", TypeName::primitive("object")));
    fb.returns(TypeName::primitive("str"));
    fb.body(CodeBlock::<Python>::of("return %T(value)", (json_dumps,)).unwrap());

    let mut file_b = FileSpec::builder_with("utils.py", Python::new());
    file_b.add_function(fb.build());
    let file = file_b.build();

    let output = file.render(80).unwrap();
    golden::assert_golden("python/top_level_function.py", &output);
}

// === Enum ===

#[test]
fn test_python_enum() {
    let enum_base = TypeName::<Python>::importable("enum", "Enum");

    let mut tb = TypeSpec::<Python>::builder("Direction", TypeKind::Enum);
    tb.extends(enum_base);

    // Enum members as fields with initializers.
    let mut up = FieldSpec::builder("UP", TypeName::primitive(""));
    up.initializer(CodeBlock::<Python>::of("'UP'", ()).unwrap());
    tb.add_field(up.build());

    let mut down = FieldSpec::builder("DOWN", TypeName::primitive(""));
    down.initializer(CodeBlock::<Python>::of("'DOWN'", ()).unwrap());
    tb.add_field(down.build());

    let mut left = FieldSpec::builder("LEFT", TypeName::primitive(""));
    left.initializer(CodeBlock::<Python>::of("'LEFT'", ()).unwrap());
    tb.add_field(left.build());

    let mut right = FieldSpec::builder("RIGHT", TypeName::primitive(""));
    right.initializer(CodeBlock::<Python>::of("'RIGHT'", ()).unwrap());
    tb.add_field(right.build());

    let mut fb = FileSpec::builder_with("direction.py", Python::new());
    fb.add_type(tb.build());
    let file = fb.build();

    let output = file.render(80).unwrap();
    golden::assert_golden("python/enum.py", &output);
}

// === Abstract method (empty body with `...`) ===

#[test]
fn test_python_abstract_method() {
    let abc = TypeName::<Python>::importable("abc", "ABC");
    let abstractmethod = TypeName::<Python>::importable("abc", "abstractmethod");

    let mut tb = TypeSpec::<Python>::builder("BaseController", TypeKind::Class);
    tb.extends(abc);

    let mut handle = FunSpec::<Python>::builder("handle_request");
    handle.annotation(CodeBlock::<Python>::of("@%T", (abstractmethod,)).unwrap());
    handle.add_param(ParameterSpec::new("self", TypeName::primitive("")));
    handle.add_param(ParameterSpec::new("req", TypeName::primitive("Request")));
    handle.returns(TypeName::primitive("Response"));
    // No body — should emit `...`
    tb.add_method(handle.build());

    let mut log_fn = FunSpec::<Python>::builder("log");
    log_fn.add_param(ParameterSpec::new("self", TypeName::primitive("")));
    log_fn.returns(TypeName::primitive("None"));
    log_fn.body(CodeBlock::<Python>::of("print('handled')", ()).unwrap());
    tb.add_method(log_fn.build());

    let mut fb = FileSpec::builder_with("controller.py", Python::new());
    fb.add_type(tb.build());
    let file = fb.build();

    let output = file.render(80).unwrap();
    golden::assert_golden("python/abstract_class.py", &output);
}

// === Decorated function ===

#[test]
fn test_python_decorated_function() {
    let mut fb = FunSpec::<Python>::builder("my_view");
    fb.annotation(CodeBlock::<Python>::of("@app.route('/hello')", ()).unwrap());
    fb.returns(TypeName::primitive("str"));
    fb.body(CodeBlock::<Python>::of("return 'Hello, World!'", ()).unwrap());

    let mut file_b = FileSpec::builder_with("views.py", Python::new());
    file_b.add_function(fb.build());
    let file = file_b.build();

    let output = file.render(80).unwrap();
    golden::assert_golden("python/decorated_function.py", &output);
}
