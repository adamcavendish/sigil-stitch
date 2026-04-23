use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::haskell::Haskell;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_function_with_params() {
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
fn test_function_with_import() {
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
fn test_function_with_context() {
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
fn test_function_no_body() {
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
fn test_function_with_doc() {
    let body = CodeBlock::<Haskell>::of("putStrLn (\"Hello, \" ++ name)", ()).unwrap();
    let mut fb = FunSpec::<Haskell>::builder("greet");
    fb.doc("Greet the user by name.");
    fb.add_param(ParameterSpec::new("name", TypeName::primitive("String")).unwrap());
    fb.returns(TypeName::primitive("IO ()"));
    fb.body(body);
    let fun = fb.build().unwrap();

    let mut file_b = FileSpec::builder_with("greet.hs", Haskell::new());
    file_b.add_function(fun);
    let file = file_b.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("haskell/function_with_doc.hs", &output);
}

#[test]
fn test_multi_constraint_context() {
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
