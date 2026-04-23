use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::python::Python;
use sigil_stitch::spec::enum_variant_spec::EnumVariantSpec;
use sigil_stitch::spec::field_spec::FieldSpec;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::modifiers::TypeKind;
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::spec::type_spec::TypeSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_dataclass() {
    let mut tb = TypeSpec::<Python>::builder("Config", TypeKind::Class);
    tb.doc("Application configuration.");
    tb.annotation(CodeBlock::<Python>::of("@dataclass", ()).unwrap());

    tb.add_field(
        FieldSpec::builder("name", TypeName::primitive("str"))
            .build()
            .unwrap(),
    );
    tb.add_field(
        FieldSpec::builder("port", TypeName::primitive("int"))
            .build()
            .unwrap(),
    );

    let mut f3 = FieldSpec::builder("debug", TypeName::primitive("bool"));
    f3.initializer(CodeBlock::<Python>::of("False", ()).unwrap());
    tb.add_field(f3.build().unwrap());

    let mut fb = FileSpec::builder_with("config.py", Python::new());
    fb.add_type(tb.build().unwrap());
    let file = fb.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("python/dataclass.py", &output);
}

#[test]
fn test_class_with_methods() {
    let mut tb = TypeSpec::<Python>::builder("UserService", TypeKind::Class);
    tb.doc("Service for managing users.");

    tb.add_field(
        FieldSpec::builder("_repo", TypeName::primitive("UserRepository"))
            .build()
            .unwrap(),
    );

    let mut get_user = FunSpec::<Python>::builder("get_user");
    get_user.add_param(ParameterSpec::new("self", TypeName::primitive("")).unwrap());
    get_user.add_param(ParameterSpec::new("user_id", TypeName::primitive("str")).unwrap());
    get_user.returns(TypeName::primitive("User"));
    get_user.body(CodeBlock::<Python>::of("return self._repo.find(user_id)", ()).unwrap());
    tb.add_method(get_user.build().unwrap());

    let mut save_user = FunSpec::<Python>::builder("save_user");
    save_user.add_param(ParameterSpec::new("self", TypeName::primitive("")).unwrap());
    save_user.add_param(ParameterSpec::new("user", TypeName::primitive("User")).unwrap());
    save_user.returns(TypeName::primitive("None"));
    save_user.body(CodeBlock::<Python>::of("self._repo.save(user)", ()).unwrap());
    tb.add_method(save_user.build().unwrap());

    let mut fb = FileSpec::builder_with("service.py", Python::new());
    fb.add_type(tb.build().unwrap());
    let file = fb.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("python/class_with_methods.py", &output);
}

#[test]
fn test_class_with_bases() {
    let base = TypeName::<Python>::primitive("BaseService");
    let auth = TypeName::<Python>::primitive("Authenticatable");

    let mut tb = TypeSpec::<Python>::builder("AdminService", TypeKind::Class);
    tb.extends(base);
    tb.implements(auth);

    let mut method = FunSpec::<Python>::builder("is_admin");
    method.add_param(ParameterSpec::new("self", TypeName::primitive("")).unwrap());
    method.returns(TypeName::primitive("bool"));
    method.body(CodeBlock::<Python>::of("return True", ()).unwrap());
    tb.add_method(method.build().unwrap());

    let mut fb = FileSpec::builder_with("admin.py", Python::new());
    fb.add_type(tb.build().unwrap());
    let file = fb.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("python/class_with_bases.py", &output);
}

#[test]
fn test_protocol() {
    let protocol = TypeName::<Python>::importable("typing", "Protocol");

    let mut tb = TypeSpec::<Python>::builder("Repository", TypeKind::Interface);
    tb.doc("Repository defines data access methods.");
    tb.extends(protocol);

    let mut find = FunSpec::<Python>::builder("find_by_id");
    find.add_param(ParameterSpec::new("self", TypeName::primitive("")).unwrap());
    find.add_param(ParameterSpec::new("id", TypeName::primitive("str")).unwrap());
    find.returns(TypeName::primitive("Entity"));
    tb.add_method(find.build().unwrap());

    let mut save = FunSpec::<Python>::builder("save");
    save.add_param(ParameterSpec::new("self", TypeName::primitive("")).unwrap());
    save.add_param(ParameterSpec::new("entity", TypeName::primitive("Entity")).unwrap());
    save.returns(TypeName::primitive("None"));
    tb.add_method(save.build().unwrap());

    let mut fb = FileSpec::builder_with("repo.py", Python::new());
    fb.add_type(tb.build().unwrap());
    let file = fb.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("python/protocol.py", &output);
}

#[test]
fn test_enum() {
    let enum_base = TypeName::<Python>::importable("enum", "Enum");

    let mut tb = TypeSpec::<Python>::builder("Direction", TypeKind::Enum);
    tb.extends(enum_base);

    // Enum members as variants with values.
    let mut v_up = EnumVariantSpec::<Python>::builder("UP");
    v_up.value(CodeBlock::<Python>::of("'UP'", ()).unwrap());
    tb.add_variant(v_up.build().unwrap());

    let mut v_down = EnumVariantSpec::<Python>::builder("DOWN");
    v_down.value(CodeBlock::<Python>::of("'DOWN'", ()).unwrap());
    tb.add_variant(v_down.build().unwrap());

    let mut v_left = EnumVariantSpec::<Python>::builder("LEFT");
    v_left.value(CodeBlock::<Python>::of("'LEFT'", ()).unwrap());
    tb.add_variant(v_left.build().unwrap());

    let mut v_right = EnumVariantSpec::<Python>::builder("RIGHT");
    v_right.value(CodeBlock::<Python>::of("'RIGHT'", ()).unwrap());
    tb.add_variant(v_right.build().unwrap());

    let mut fb = FileSpec::builder_with("direction.py", Python::new());
    fb.add_type(tb.build().unwrap());
    let file = fb.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("python/enum.py", &output);
}
