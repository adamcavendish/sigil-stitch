mod golden;

use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::scala::Scala;
use sigil_stitch::spec::enum_variant_spec::EnumVariantSpec;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::{FunSpec, TypeParamKind, TypeParamSpec};
use sigil_stitch::spec::modifiers::TypeKind;
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::spec::type_spec::TypeSpec;
use sigil_stitch::type_name::TypeName;

#[test]
fn test_scala_case_class() {
    let mut tb = TypeSpec::<Scala>::builder("User", TypeKind::Struct);
    tb.doc("A user case class.");

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

    let mut fb = FileSpec::builder_with("User.scala", Scala::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("scala/case_class.scala", &output);
}

#[test]
fn test_scala_trait_with_type_param() {
    let tp = TypeParamSpec::<Scala>::new("T");

    let mut tb = TypeSpec::<Scala>::builder("Repository", TypeKind::Trait);
    tb.add_type_param(tp);
    tb.doc("Generic data repository.");

    let mut find = FunSpec::<Scala>::builder("findById");
    find.returns(TypeName::primitive("Option[T]"));
    find.add_param(ParameterSpec::new("id", TypeName::primitive("String")).unwrap());
    tb.add_method(find.build().unwrap());

    let mut save = FunSpec::<Scala>::builder("save");
    save.add_param(ParameterSpec::new("entity", TypeName::primitive("T")).unwrap());
    tb.add_method(save.build().unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("Repository.scala", Scala::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("scala/trait_with_type_param.scala", &output);
}

#[test]
fn test_scala_class_extends() {
    let base = TypeName::<Scala>::importable("com.example.base", "BaseService");
    let serial = TypeName::<Scala>::importable("com.example.serial", "Serializable");

    let mut tb = TypeSpec::<Scala>::builder("AdminService", TypeKind::Class);
    tb.extends(base);
    tb.extends(serial);

    let body = CodeBlock::<Scala>::of("true", ()).unwrap();
    let mut is_admin = FunSpec::<Scala>::builder("isAdmin");
    is_admin.returns(TypeName::primitive("Boolean"));
    is_admin.body(body);
    tb.add_method(is_admin.build().unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("AdminService.scala", Scala::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("scala/class_extends.scala", &output);
}

#[test]
fn test_scala_class_extends_multiple() {
    let mut tb = TypeSpec::<Scala>::builder("Worker", TypeKind::Class);
    tb.extends(TypeName::primitive("Actor"));
    tb.extends(TypeName::primitive("Logging"));
    tb.extends(TypeName::primitive("Serializable"));

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("Worker.scala", Scala::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("scala/class_extends_multiple.scala", &output);
}

#[test]
fn test_scala_enum() {
    let mut tb = TypeSpec::<Scala>::builder("Color", TypeKind::Enum);
    tb.doc("Supported colors.");

    tb.add_variant(EnumVariantSpec::new("Red").unwrap());
    tb.add_variant(EnumVariantSpec::new("Green").unwrap());
    tb.add_variant(EnumVariantSpec::new("Blue").unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("Color.scala", Scala::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("scala/enum.scala", &output);
}

#[test]
fn test_scala_hkt_type_param() {
    let tp_f = TypeParamSpec::<Scala>::new("F").with_kind(TypeParamKind::Constructor1);
    let tp_a = TypeParamSpec::<Scala>::new("A");

    let mut fb_fun = FunSpec::<Scala>::builder("traverse");
    fb_fun.add_type_param(tp_f);
    fb_fun.add_type_param(tp_a);
    fb_fun.returns(TypeName::primitive("F[List[A]]"));
    fb_fun.add_param(ParameterSpec::new("list", TypeName::primitive("List[A]")).unwrap());
    fb_fun.add_param(ParameterSpec::new("f", TypeName::primitive("A => F[A]")).unwrap());
    let body = CodeBlock::<Scala>::of("???", ()).unwrap();
    fb_fun.body(body);
    let fun = fb_fun.build().unwrap();

    let mut fb = FileSpec::builder_with("Traverse.scala", Scala::new());
    fb.add_function(fun);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("scala/hkt_type_param.scala", &output);
}

#[test]
fn test_scala_bounded_type_param() {
    let tp = TypeParamSpec::<Scala>::new("T").with_bound(TypeName::primitive("Comparable[T]"));

    let mut fb_fun = FunSpec::<Scala>::builder("max");
    fb_fun.add_type_param(tp);
    fb_fun.returns(TypeName::primitive("T"));
    fb_fun.add_param(ParameterSpec::new("a", TypeName::primitive("T")).unwrap());
    fb_fun.add_param(ParameterSpec::new("b", TypeName::primitive("T")).unwrap());
    let body = CodeBlock::<Scala>::of("if (a.compareTo(b) >= 0) a else b", ()).unwrap();
    fb_fun.body(body);
    let fun = fb_fun.build().unwrap();

    let mut fb = FileSpec::builder_with("Max.scala", Scala::new());
    fb.add_function(fun);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("scala/bounded_type_param.scala", &output);
}

#[test]
fn test_scala_abstract_class() {
    let mut tb = TypeSpec::<Scala>::builder("Shape", TypeKind::Class);
    tb.doc("Abstract shape.");
    tb.is_abstract();

    let desc_body = CodeBlock::<Scala>::of("getClass.getSimpleName", ()).unwrap();
    let mut desc = FunSpec::<Scala>::builder("describe");
    desc.returns(TypeName::primitive("String"));
    desc.body(desc_body);
    tb.add_method(desc.build().unwrap());

    let mut area = FunSpec::<Scala>::builder("area");
    area.is_abstract();
    area.returns(TypeName::primitive("Double"));
    tb.add_method(area.build().unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("Shape.scala", Scala::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("scala/abstract_class.scala", &output);
}

#[test]
fn test_scala_suspend_function() {
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
fn test_scala_context_bound() {
    let body = CodeBlock::<Scala>::of("implicitly[Ordering[T]].compare(a, b)", ()).unwrap();
    let mut fb_fun = FunSpec::<Scala>::builder("sortedPair");
    fb_fun.add_type_param(
        TypeParamSpec::new("T").with_context_bound(TypeName::primitive("Ordering")),
    );
    fb_fun.add_param(ParameterSpec::new("a", TypeName::primitive("T")).unwrap());
    fb_fun.add_param(ParameterSpec::new("b", TypeName::primitive("T")).unwrap());
    fb_fun.returns(TypeName::generic(
        TypeName::primitive("Tuple2"),
        vec![TypeName::primitive("T"), TypeName::primitive("T")],
    ));
    fb_fun.body(body);
    let fun = fb_fun.build().unwrap();

    let mut fb = FileSpec::builder_with("sorted.scala", Scala::new());
    fb.add_function(fun);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("scala/context_bound.scala", &output);
}

#[test]
fn test_scala_newtype() {
    let mut tb = TypeSpec::<Scala>::builder("Meters", TypeKind::Newtype);
    tb.extends(TypeName::primitive("Double"));

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("meters.scala", Scala::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("scala/newtype.scala", &output);
}

#[test]
fn test_scala_multiple_context_bounds() {
    let body = CodeBlock::<Scala>::of("implicitly[Ordering[T]].compare(a, b)", ()).unwrap();
    let mut fb_fun = FunSpec::<Scala>::builder("compare");
    fb_fun.add_type_param(
        TypeParamSpec::new("T")
            .with_context_bound(TypeName::primitive("Ordering"))
            .with_context_bound(TypeName::primitive("Numeric")),
    );
    fb_fun.add_param(ParameterSpec::new("a", TypeName::primitive("T")).unwrap());
    fb_fun.add_param(ParameterSpec::new("b", TypeName::primitive("T")).unwrap());
    fb_fun.returns(TypeName::primitive("Int"));
    fb_fun.body(body);
    let fun = fb_fun.build().unwrap();

    let mut fb = FileSpec::builder_with("compare.scala", Scala::new());
    fb.add_function(fun);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("scala/multiple_context_bounds.scala", &output);
}
