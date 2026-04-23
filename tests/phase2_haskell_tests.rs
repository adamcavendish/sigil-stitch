mod golden;

use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::haskell::Haskell;
use sigil_stitch::spec::enum_variant_spec::EnumVariantSpec;
use sigil_stitch::spec::field_spec::FieldSpec;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::modifiers::TypeKind;
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::spec::type_spec::TypeSpec;
use sigil_stitch::type_name::TypeName;

#[test]
fn test_haskell_data_type_with_record() {
    let mut tb = TypeSpec::<Haskell>::builder("Person", TypeKind::Struct);
    tb.doc("A person record type.");

    tb.add_field(
        FieldSpec::builder("personName", TypeName::primitive("String"))
            .build()
            .unwrap(),
    );
    tb.add_field(
        FieldSpec::builder("personAge", TypeName::primitive("Int"))
            .build()
            .unwrap(),
    );
    tb.add_field(
        FieldSpec::builder("personEmail", TypeName::primitive("String"))
            .build()
            .unwrap(),
    );

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("Person.hs", Haskell::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("haskell/data_type_record.hs", &output);
}

#[test]
fn test_haskell_enum_type() {
    let mut tb = TypeSpec::<Haskell>::builder("Color", TypeKind::Enum);
    tb.doc("Supported colors.");

    tb.add_variant(EnumVariantSpec::new("Red").unwrap());
    tb.add_variant(EnumVariantSpec::new("Green").unwrap());
    tb.add_variant(EnumVariantSpec::new("Blue").unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("Color.hs", Haskell::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("haskell/enum_type.hs", &output);
}

#[test]
fn test_haskell_function_with_params() {
    let body = CodeBlock::<Haskell>::of("x + y", ()).unwrap();
    let mut fb_fun = FunSpec::<Haskell>::builder("add");
    fb_fun.returns(TypeName::primitive("Int"));
    fb_fun.add_param(ParameterSpec::new("x", TypeName::primitive("Int")).unwrap());
    fb_fun.add_param(ParameterSpec::new("y", TypeName::primitive("Int")).unwrap());
    fb_fun.body(body);
    let fun = fb_fun.build().unwrap();

    let mut fb = FileSpec::builder_with("Add.hs", Haskell::new());
    fb.add_function(fun);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("haskell/function_with_params.hs", &output);
}

#[test]
fn test_haskell_type_alias() {
    let mut tb = TypeSpec::<Haskell>::builder("Name", TypeKind::TypeAlias);
    tb.extends(TypeName::primitive("String"));

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("Name.hs", Haskell::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("haskell/type_alias.hs", &output);
}

#[test]
fn test_haskell_type_class_where() {
    let mut b = CodeBlock::<Haskell>::builder();
    b.begin_control_flow_with_open("class Functor f", (), " where");
    b.add_statement("fmap :: (a -> b) -> f a -> f b", ());
    b.end_control_flow();
    let block = b.build().unwrap();

    let mut fb = FileSpec::builder_with("Functor.hs", Haskell::new());
    fb.add_code(block);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("haskell/type_class_where.hs", &output);
}

#[test]
fn test_haskell_function_with_import() {
    let map_type = TypeName::<Haskell>::importable("Data.Map", "Map");

    let body = CodeBlock::<Haskell>::of("Data.Map.empty", ()).unwrap();
    let mut fb_fun = FunSpec::<Haskell>::builder("emptyMap");
    fb_fun.returns(map_type);
    fb_fun.body(body);
    let fun = fb_fun.build().unwrap();

    let mut fb = FileSpec::builder_with("EmptyMap.hs", Haskell::new());
    fb.add_function(fun);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("haskell/function_with_import.hs", &output);
}

#[test]
fn test_haskell_data_with_deriving() {
    let mut tb = TypeSpec::<Haskell>::builder("Color", TypeKind::Enum);
    tb.add_variant(EnumVariantSpec::new("Red").unwrap());
    tb.add_variant(EnumVariantSpec::new("Green").unwrap());
    tb.add_variant(EnumVariantSpec::new("Blue").unwrap());
    tb.implements(TypeName::primitive("Show"));
    tb.implements(TypeName::primitive("Eq"));

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("Color.hs", Haskell::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("haskell/data_with_deriving.hs", &output);
}

#[test]
fn test_haskell_function_with_context() {
    let body = CodeBlock::<Haskell>::of("show x", ()).unwrap();
    let mut fb_fun = FunSpec::<Haskell>::builder("display");
    fb_fun.add_type_param(
        sigil_stitch::spec::fun_spec::TypeParamSpec::new("a")
            .with_bound(TypeName::primitive("Show")),
    );
    fb_fun.add_param(ParameterSpec::new("x", TypeName::primitive("a")).unwrap());
    fb_fun.returns(TypeName::primitive("String"));
    fb_fun.body(body);
    let fun = fb_fun.build().unwrap();

    let mut fb = FileSpec::builder_with("Display.hs", Haskell::new());
    fb.add_function(fun);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("haskell/function_with_context.hs", &output);
}

#[test]
fn test_haskell_function_no_body() {
    let mut fb_fun = FunSpec::<Haskell>::builder("add");
    fb_fun.returns(TypeName::primitive("Int"));
    fb_fun.add_param(ParameterSpec::new("x", TypeName::primitive("Int")).unwrap());
    fb_fun.add_param(ParameterSpec::new("y", TypeName::primitive("Int")).unwrap());
    let fun = fb_fun.build().unwrap();

    let mut fb = FileSpec::builder_with("Add.hs", Haskell::new());
    fb.add_function(fun);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("haskell/function_no_body.hs", &output);
}

#[test]
fn test_haskell_function_with_doc() {
    let body = CodeBlock::<Haskell>::of("x + y", ()).unwrap();
    let mut fb_fun = FunSpec::<Haskell>::builder("add");
    fb_fun.doc("Adds two numbers.");
    fb_fun.returns(TypeName::primitive("Int"));
    fb_fun.add_param(ParameterSpec::new("x", TypeName::primitive("Int")).unwrap());
    fb_fun.add_param(ParameterSpec::new("y", TypeName::primitive("Int")).unwrap());
    fb_fun.body(body);
    let fun = fb_fun.build().unwrap();

    let mut fb = FileSpec::builder_with("Add.hs", Haskell::new());
    fb.add_function(fun);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("haskell/function_with_doc.hs", &output);
}

#[test]
fn test_haskell_multi_constraint_context() {
    let body = CodeBlock::<Haskell>::of("show x", ()).unwrap();
    let mut fb_fun = FunSpec::<Haskell>::builder("display");
    fb_fun.add_type_param(
        sigil_stitch::spec::fun_spec::TypeParamSpec::new("a")
            .with_bound(TypeName::primitive("Show"))
            .with_bound(TypeName::primitive("Eq")),
    );
    fb_fun.add_param(ParameterSpec::new("x", TypeName::primitive("a")).unwrap());
    fb_fun.returns(TypeName::primitive("String"));
    fb_fun.body(body);
    let fun = fb_fun.build().unwrap();

    let mut fb = FileSpec::builder_with("Display.hs", Haskell::new());
    fb.add_function(fun);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("haskell/function_multi_context.hs", &output);
}

#[test]
fn test_haskell_newtype() {
    let mut tb = TypeSpec::<Haskell>::builder("Meters", TypeKind::Newtype);
    tb.extends(TypeName::primitive("Int"));

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("Meters.hs", Haskell::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("haskell/newtype.hs", &output);
}

#[test]
fn test_haskell_type_class_via_type_spec() {
    let mut tb = TypeSpec::<Haskell>::builder("Printable", TypeKind::Trait);
    tb.doc("Things that can be printed.");

    let mut pretty = FunSpec::<Haskell>::builder("prettyPrint");
    pretty.add_param(ParameterSpec::new("x", TypeName::primitive("a")).unwrap());
    pretty.returns(TypeName::primitive("String"));
    tb.add_method(pretty.build().unwrap());

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("Printable.hs", Haskell::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("haskell/type_class_spec.hs", &output);
}

#[test]
fn test_haskell_data_with_deriving_record() {
    let mut tb = TypeSpec::<Haskell>::builder("Person", TypeKind::Struct);
    tb.add_field(
        FieldSpec::builder("personName", TypeName::primitive("String"))
            .build()
            .unwrap(),
    );
    tb.add_field(
        FieldSpec::builder("personAge", TypeName::primitive("Int"))
            .build()
            .unwrap(),
    );
    tb.implements(TypeName::primitive("Show"));
    tb.implements(TypeName::primitive("Eq"));

    let ts = tb.build().unwrap();

    let mut fb = FileSpec::builder_with("Person.hs", Haskell::new());
    fb.add_type(ts);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("haskell/data_deriving_record.hs", &output);
}
