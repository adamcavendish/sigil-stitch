use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::haskell::Haskell;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

#[test]
fn test_where_block() {
    let mut b = CodeBlock::builder();
    b.begin_control_flow("circleArea r", ());
    b.add_statement("pi * r * r", ());
    b.end_control_flow();
    let block = b.build().unwrap();

    let file = FileSpec::builder_with("Circle.hs", Haskell::new())
        .add_code(block)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("haskell/where_block.hs", &output);
}

#[test]
fn test_type_class_where() {
    let mut b = CodeBlock::builder();
    b.begin_control_flow_with_open("class Functor f", (), " where");
    b.add_statement("fmap :: (a -> b) -> f a -> f b", ());
    b.end_control_flow();
    let block = b.build().unwrap();

    let file = FileSpec::builder_with("Functor.hs", Haskell::new())
        .add_code(block)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("haskell/type_class_where.hs", &output);
}
