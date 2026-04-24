use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_source_imports() {
    let utils_fn = TypeName::importable("./lib/utils.zsh", "log_info");
    let config_fn = TypeName::importable("./lib/config.zsh", "load_config");

    let mut b = CodeBlock::builder();
    b.add_statement("%T", (config_fn,));
    b.add_statement("%T \"starting up\"", (utils_fn,));
    let block = b.build().unwrap();

    let file = FileSpec::builder("main.zsh")
        .add_code(block)
        .build()
        .unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("zsh/source_imports.zsh", &output);
}
