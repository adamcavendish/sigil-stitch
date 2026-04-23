use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::zsh::Zsh;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_source_imports() {
    let utils_fn = TypeName::<Zsh>::importable("./lib/utils.zsh", "log_info");
    let config_fn = TypeName::<Zsh>::importable("./lib/config.zsh", "load_config");

    let mut b = CodeBlock::<Zsh>::builder();
    b.add_statement("%T", (config_fn,));
    b.add_statement("%T \"starting up\"", (utils_fn,));
    let block = b.build().unwrap();

    let mut fb = FileSpec::<Zsh>::builder("main.zsh");
    fb.add_code(block);
    let file = fb.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("zsh/source_imports.zsh", &output);
}
