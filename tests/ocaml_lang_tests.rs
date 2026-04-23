mod golden;

use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::ocaml::OCaml;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

#[test]
fn test_ocaml_function_with_imports() {
    let list_mod = TypeName::<OCaml>::importable("List", "t");

    let mut b = CodeBlock::<OCaml>::builder();
    b.add_statement("let result = %T.map f xs", (list_mod,));
    let block = b.build().unwrap();

    let mut fb = FileSpec::builder_with("app.ml", OCaml::new());
    fb.add_code(block);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ocaml/function_with_imports.ml", &output);
}

#[test]
fn test_ocaml_let_binding() {
    let mut b = CodeBlock::<OCaml>::builder();
    b.begin_control_flow("let add x y", ());
    b.add_statement("x + y", ());
    b.end_control_flow();
    let block = b.build().unwrap();

    let mut fb = FileSpec::builder_with("add.ml", OCaml::new());
    fb.add_code(block);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ocaml/let_binding.ml", &output);
}

#[test]
fn test_ocaml_pattern_match() {
    let mut b = CodeBlock::<OCaml>::builder();
    b.begin_control_flow("let describe color", ());
    b.begin_control_flow_with_open("match color with", (), "");
    b.add("| Red -> \"red\"", ());
    b.add_line();
    b.add("| Green -> \"green\"", ());
    b.add_line();
    b.add("| Blue -> \"blue\"", ());
    b.add_line();
    b.end_control_flow();
    b.end_control_flow();
    let block = b.build().unwrap();

    let mut fb = FileSpec::builder_with("match.ml", OCaml::new());
    fb.add_code(block);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ocaml/pattern_match.ml", &output);
}

#[test]
fn test_ocaml_comment_closes() {
    let mut b = CodeBlock::<OCaml>::builder();
    b.add_comment("this is a comment");
    let block = b.build().unwrap();

    let mut fb = FileSpec::builder_with("comment.ml", OCaml::new());
    fb.add_code(block);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ocaml/comment.ml", &output);
}
