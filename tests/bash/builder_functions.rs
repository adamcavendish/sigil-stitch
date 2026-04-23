use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::bash::Bash;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_function_spec() {
    let mut body_b = CodeBlock::<Bash>::builder();
    body_b.add("local name=$1\n", ());
    body_b.add_statement("echo \"Hello, $name\"", ());
    let body = body_b.build().unwrap();

    let mut fb = FunSpec::<Bash>::builder("greet");
    fb.body(body);
    let fun = fb.build().unwrap();

    let mut file_b = FileSpec::<Bash>::builder("funcs.bash");
    file_b.add_function(fun);
    let file = file_b.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("bash/function_spec.bash", &output);
}

#[test]
fn test_complete_script() {
    let utils_fn = TypeName::<Bash>::importable("./lib/utils.sh", "log_info");

    let mut header_b = CodeBlock::<Bash>::builder();
    header_b.add("#!/usr/bin/env bash\n", ());
    header_b.add("set -euo pipefail", ());
    let header = header_b.build().unwrap();

    // Function
    let mut fun_body = CodeBlock::<Bash>::builder();
    fun_body.add("local target=$1\n", ());
    fun_body.add("if [ -z \"$target\" ]; then\n", ());
    fun_body.add("%>", ());
    fun_body.add_statement("echo \"error: no target\"", ());
    fun_body.add_statement("return 1", ());
    fun_body.add("%<", ());
    fun_body.add("fi\n", ());
    fun_body.add_statement("%T \"deploying to $target\"", (utils_fn,));
    let fun_body = fun_body.build().unwrap();

    let mut fun = FunSpec::<Bash>::builder("deploy");
    fun.body(fun_body);
    let fun = fun.build().unwrap();

    // Main body
    let mut main = CodeBlock::<Bash>::builder();
    main.add_statement("deploy \"$@\"", ());
    let main = main.build().unwrap();

    let mut fb = FileSpec::<Bash>::builder("deploy.bash");
    fb.header(header);
    fb.add_function(fun);
    fb.add_code(main);
    let file = fb.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("bash/complete_script.bash", &output);
}

#[test]
fn test_function_with_doc() {
    let body = CodeBlock::<Bash>::of("echo \"Hello, $1!\"", ()).unwrap();
    let mut fb = FunSpec::<Bash>::builder("greet");
    fb.doc("Greet the user by name.");
    fb.body(body);
    let fun = fb.build().unwrap();

    let mut file_b = FileSpec::<Bash>::builder("greet.bash");
    file_b.add_function(fun);
    let file = file_b.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("bash/function_with_doc.bash", &output);
}
