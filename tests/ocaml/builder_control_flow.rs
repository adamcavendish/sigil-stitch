use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::ocaml::OCaml;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

#[test]
fn test_let_binding() {
    let mut b = CodeBlock::builder();
    b.begin_control_flow("let add x y", ());
    b.add_statement("x + y", ());
    b.end_control_flow();
    let block = b.build().unwrap();

    let file = FileSpec::builder_with("add.ml", OCaml::new())
        .add_code(block)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ocaml/let_binding.ml", &output);
}

#[test]
fn test_pattern_match() {
    let mut b = CodeBlock::builder();
    b.begin_control_flow("let describe color", ());
    b.begin_control_flow("match color with", ());
    b.add("| Red -> \"red\"", ());
    b.add_line();
    b.add("| Green -> \"green\"", ());
    b.add_line();
    b.add("| Blue -> \"blue\"", ());
    b.add_line();
    b.end_control_flow();
    b.end_control_flow();
    let block = b.build().unwrap();

    let file = FileSpec::builder_with("match.ml", OCaml::new())
        .add_code(block)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("ocaml/pattern_match.ml", &output);
}
