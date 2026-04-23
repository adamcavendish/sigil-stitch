use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::scala::Scala;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

fn render(block: &CodeBlock<Scala>) -> String {
    let mut fb = FileSpec::<Scala>::builder("test.scala");
    fb.add_code(block.clone());
    fb.build().unwrap().render(80).unwrap()
}

#[test]
fn test_imports() {
    let list_buffer = TypeName::<Scala>::importable("scala.collection.mutable", "ListBuffer");
    let block = sigil_quote!(Scala {
        val buf = new $T(list_buffer)();
    })
    .unwrap();
    golden::assert_golden("scala/macro_imports.scala", &render(&block));
}
