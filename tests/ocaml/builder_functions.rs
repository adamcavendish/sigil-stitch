use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::ocaml::OCaml;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_function_with_params() {
    let body = CodeBlock::<OCaml>::of("List.map f xs", ()).unwrap();
    let mut fb_fun = FunSpec::<OCaml>::builder("transform");
    fb_fun.returns(TypeName::primitive("'b list"));
    fb_fun.add_param(ParameterSpec::new("f", TypeName::primitive("'a -> 'b")).unwrap());
    fb_fun.add_param(ParameterSpec::new("xs", TypeName::primitive("'a list")).unwrap());
    fb_fun.body(body);
    let fun = fb_fun.build().unwrap();

    let mut fb = FileSpec::builder_with("transform.ml", OCaml::new());
    fb.add_function(fun);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ocaml/function_with_params.ml", &output);
}

#[test]
fn test_function_with_doc() {
    let body = CodeBlock::<OCaml>::of("List.map f xs", ()).unwrap();
    let mut fb_fun = FunSpec::<OCaml>::builder("transform");
    fb_fun.doc("Transform a list using a mapping function.");
    fb_fun.returns(TypeName::primitive("'b list"));
    fb_fun.add_param(ParameterSpec::new("f", TypeName::primitive("'a -> 'b")).unwrap());
    fb_fun.add_param(ParameterSpec::new("xs", TypeName::primitive("'a list")).unwrap());
    fb_fun.body(body);
    let fun = fb_fun.build().unwrap();

    let mut fb = FileSpec::builder_with("transform.ml", OCaml::new());
    fb.add_function(fun);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ocaml/function_with_doc.ml", &output);
}
