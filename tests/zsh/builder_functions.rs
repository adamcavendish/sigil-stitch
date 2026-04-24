use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_function_spec() {
    let mut body_b = CodeBlock::builder();
    body_b.add("local name=$1\n", ());
    body_b.add_statement("echo \"Hello, $name\"", ());
    let body = body_b.build().unwrap();

    let fun = FunSpec::builder("greet").body(body).build().unwrap();

    let file = FileSpec::builder("funcs.zsh")
        .add_function(fun)
        .build()
        .unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("zsh/function_spec.zsh", &output);
}

#[test]
fn test_complete_script() {
    let utils_fn = TypeName::importable("./lib/utils.zsh", "log_info");

    let mut header_b = CodeBlock::builder();
    header_b.add("#!/usr/bin/env zsh\n", ());
    header_b.add("setopt ERR_EXIT PIPE_FAIL", ());
    let header = header_b.build().unwrap();

    let mut fun_body = CodeBlock::builder();
    fun_body.add("local target=$1\n", ());
    fun_body.add("if [[ -z \"$target\" ]]; then\n", ());
    fun_body.add("%>", ());
    fun_body.add_statement("echo \"error: no target\"", ());
    fun_body.add_statement("return 1", ());
    fun_body.add("%<", ());
    fun_body.add("fi\n", ());
    fun_body.add_statement("%T \"deploying to $target\"", (utils_fn,));
    let fun_body = fun_body.build().unwrap();

    let fun = FunSpec::builder("deploy").body(fun_body).build().unwrap();

    let mut main = CodeBlock::builder();
    main.add_statement("deploy \"$@\"", ());
    let main = main.build().unwrap();

    let file = FileSpec::builder("deploy.zsh")
        .header(header)
        .add_function(fun)
        .add_code(main)
        .build()
        .unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("zsh/complete_script.zsh", &output);
}

#[test]
fn test_function_with_doc() {
    let body = CodeBlock::of("echo \"Hello, $1!\"", ()).unwrap();
    let fun = FunSpec::builder("greet")
        .doc("Greet the user by name.")
        .body(body)
        .build()
        .unwrap();

    let file = FileSpec::builder("greet.zsh")
        .add_function(fun)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("zsh/function_with_doc.zsh", &output);
}
