use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::bash::Bash;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_source_imports() {
    let utils_fn = TypeName::<Bash>::importable("./lib/utils.sh", "log_info");
    let config_fn = TypeName::<Bash>::importable("./lib/config.sh", "load_config");

    let mut b = CodeBlock::<Bash>::builder();
    b.add_statement("%T", (config_fn,));
    b.add_statement("%T \"starting up\"", (utils_fn,));
    let block = b.build().unwrap();

    let mut fb = FileSpec::<Bash>::builder("main.bash");
    fb.add_code(block);
    let file = fb.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("bash/source_imports.bash", &output);
}
