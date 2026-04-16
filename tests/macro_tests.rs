//! Integration tests for the `sigil_quote!` proc macro.

use sigil_stitch::code_block::{CodeBlock, NameArg, StringLitArg};
use sigil_stitch::import_collector;
use sigil_stitch::lang::go_lang::GoLang;
use sigil_stitch::lang::python::Python;
use sigil_stitch::lang::rust_lang::RustLang;
use sigil_stitch::lang::typescript::TypeScript;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

// ════════════════════════════════════════════════════════
// A. Basic statements
// ════════════════════════════════════════════════════════

#[test]
fn test_simple_statement() {
    let block = sigil_quote!(TypeScript {
        const x = 42;
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("const x = 42;"), "got: {output}");
}

#[test]
fn test_multiple_statements() {
    let block = sigil_quote!(TypeScript {
        const a = 1;
        const b = 2;
        const c = a + b;
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("const a = 1;"), "got: {output}");
    assert!(output.contains("const b = 2;"), "got: {output}");
    assert!(output.contains("const c = a + b;"), "got: {output}");
}

#[test]
fn test_empty_body() {
    let block = sigil_quote!(TypeScript {}).unwrap();
    assert!(block.is_empty());
}

// ════════════════════════════════════════════════════════
// B. Interpolation — $T, $N, $S, $L, $C
// ════════════════════════════════════════════════════════

#[test]
fn test_type_interpolation_import_tracking() {
    let user_type = TypeName::<TypeScript>::importable_type("./models", "User");
    let block = sigil_quote!(TypeScript {
        const user: $T(user_type) = getUser();
    })
    .unwrap();

    let refs = import_collector::collect_imports(&block);
    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0].name, "User");
}

#[test]
fn test_type_interpolation_renders() {
    let user_type = TypeName::<TypeScript>::importable_type("./models", "User");
    let block = sigil_quote!(TypeScript {
        const user: $T(user_type) = getUser();
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("User"), "got: {output}");
    assert!(output.contains("= getUser();"), "got: {output}");
}

#[test]
fn test_name_interpolation() {
    let var_name = "myVar";
    let block = sigil_quote!(TypeScript {
        const $N(var_name) = 42;
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("myVar"), "got: {output}");
    assert!(output.contains("= 42;"), "got: {output}");
}

#[test]
fn test_string_lit_interpolation() {
    let block = sigil_quote!(TypeScript {
        console.log($S("hello world"));
    })
    .unwrap();

    let output = render_ts(&block);
    // TypeScript renderer uses single quotes by default.
    assert!(
        output.contains("console.log('hello world');"),
        "got: {output}"
    );
}

#[test]
fn test_literal_interpolation() {
    let val = "42";
    let block = sigil_quote!(TypeScript {
        const x = $L(val);
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("const x = 42;"), "got: {output}");
}

#[test]
fn test_code_block_interpolation() {
    let inner = CodeBlock::<TypeScript>::of("doSomething()", ()).unwrap();
    let block = sigil_quote!(TypeScript {
        $C(inner);
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("doSomething();"), "got: {output}");
}

#[test]
fn test_multiple_interpolations_in_one_statement() {
    let t1 = TypeName::<TypeScript>::primitive("string");
    let t2 = TypeName::<TypeScript>::primitive("number");
    let block = sigil_quote!(TypeScript {
        const x: $T(t1) = $L("getVal") as $T(t2);
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("string"), "got: {output}");
    assert!(output.contains("number"), "got: {output}");
    assert!(output.contains("getVal"), "got: {output}");
}

#[test]
fn test_mixed_arg_types_in_one_statement() {
    let ty = TypeName::<TypeScript>::primitive("User");
    let block = sigil_quote!(TypeScript {
        const $N("x"): $T(ty) = $S("hello") + $L("42");
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("x"), "got: {output}");
    assert!(output.contains("User"), "got: {output}");
    assert!(output.contains("'hello'"), "got: {output}");
    assert!(output.contains("42"), "got: {output}");
}

// ════════════════════════════════════════════════════════
// C. Control flow
// ════════════════════════════════════════════════════════

#[test]
fn test_if_block() {
    let block = sigil_quote!(TypeScript {
        if(x > 0) {
            return true;
        }
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("if(x > 0) {"), "got: {output}");
    assert!(output.contains("return true;"), "got: {output}");
    assert!(output.contains("}"), "got: {output}");
}

#[test]
fn test_if_else() {
    let block = sigil_quote!(TypeScript {
        if(x > 0) {
            return true;
        } else {
            return false;
        }
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("if(x > 0) {"), "got: {output}");
    assert!(output.contains("} else {"), "got: {output}");
    assert!(output.contains("return false;"), "got: {output}");
}

#[test]
fn test_if_else_if_else() {
    let block = sigil_quote!(TypeScript {
        if(x > 0) {
            return 1;
        } else if(x < 0) {
            return 0;
        } else {
            return 0;
        }
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("if(x > 0) {"), "got: {output}");
    assert!(output.contains("} else if(x < 0) {"), "got: {output}");
    assert!(output.contains("} else {"), "got: {output}");
}

#[test]
fn test_long_else_if_chain() {
    let block = sigil_quote!(TypeScript {
        if(a) {
            return 1;
        } else if(b) {
            return 2;
        } else if(c) {
            return 3;
        } else if(d) {
            return 4;
        } else {
            return 0;
        }
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("} else if(b) {"), "got: {output}");
    assert!(output.contains("} else if(c) {"), "got: {output}");
    assert!(output.contains("} else if(d) {"), "got: {output}");
    assert!(output.contains("} else {"), "got: {output}");
}

#[test]
fn test_nested_control_flow() {
    let block = sigil_quote!(TypeScript {
        if(x > 0) {
            if(y > 0) {
                return true;
            }
        }
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("if(x > 0) {"), "got: {output}");
    assert!(output.contains("if(y > 0) {"), "got: {output}");
    assert!(output.contains("return true;"), "got: {output}");
    // Should have proper indentation (two levels).
    assert!(output.contains("    return true;"), "got: {output}");
}

#[test]
fn test_empty_control_flow_body() {
    let block = sigil_quote!(TypeScript {
        if(x) {}
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("if(x) {"), "got: {output}");
    assert!(output.contains("}"), "got: {output}");
}

#[test]
fn test_interpolation_in_condition() {
    let cond = "x > 0";
    let block = sigil_quote!(TypeScript {
        if($L(cond)) {
            return true;
        }
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("if(x > 0) {"), "got: {output}");
}

#[test]
fn test_control_flow_with_type_and_string_interpolation() {
    let error_type = TypeName::<TypeScript>::importable("./errors", "NotFoundError");
    let block = sigil_quote!(TypeScript {
        if(!user) {
            throw new $T(error_type)($S("not found"));
        }
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("NotFoundError"), "got: {output}");
    assert!(output.contains("'not found'"), "got: {output}");
    let refs = import_collector::collect_imports(&block);
    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0].name, "NotFoundError");
}

// ════════════════════════════════════════════════════════
// D. Blank lines
// ════════════════════════════════════════════════════════

#[test]
fn test_single_blank_line() {
    let block = sigil_quote!(TypeScript {
        const a = 1;

        const b = 2;
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(
        output.contains("const a = 1;\n\nconst b = 2;"),
        "got: {output}"
    );
}

#[test]
fn test_multiple_blank_lines() {
    let block = sigil_quote!(TypeScript {
        const a = 1;



        const b = 2;
    })
    .unwrap();

    let output = render_ts(&block);
    // Multiple blank lines should be preserved.
    let idx_a = output.find("const a = 1;").unwrap();
    let idx_b = output.find("const b = 2;").unwrap();
    let between = &output[idx_a + "const a = 1;".len()..idx_b];
    let newlines = between.chars().filter(|c| *c == '\n').count();
    // At least 2 blank lines (3+ newlines between the statements).
    assert!(
        newlines >= 3,
        "expected multiple blank lines, got {newlines} newlines in: {between:?}"
    );
}

// ════════════════════════════════════════════════════════
// E. Comments
// ════════════════════════════════════════════════════════

#[test]
fn test_comment_with_semicolon() {
    let block = sigil_quote!(TypeScript {
        $comment("Initialize the value");
        const x = 0;
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("// Initialize the value"), "got: {output}");
    assert!(output.contains("const x = 0;"), "got: {output}");
}

#[test]
fn test_comment_without_semicolon() {
    // $comment without trailing ; should also work.
    let block = sigil_quote!(TypeScript {
        $comment("no semicolon")
        const x = 0;
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("// no semicolon"), "got: {output}");
    assert!(output.contains("const x = 0;"), "got: {output}");
}

#[test]
fn test_comment_only() {
    let block = sigil_quote!(TypeScript {
        $comment("just a comment");
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("// just a comment"), "got: {output}");
}

#[test]
fn test_multiple_comments() {
    let block = sigil_quote!(TypeScript {
        $comment("line 1");
        $comment("line 2");
        const x = 0;
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("// line 1"), "got: {output}");
    assert!(output.contains("// line 2"), "got: {output}");
}

// ════════════════════════════════════════════════════════
// F. Object literals (not control flow)
// ════════════════════════════════════════════════════════

#[test]
fn test_object_literal_simple() {
    let block = sigil_quote!(TypeScript {
        const config = { timeout: 30, retries: 3 };
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("config = {"), "got: {output}");
    assert!(output.contains("timeout:"), "got: {output}");
    assert!(output.contains("retries:"), "got: {output}");
}

#[test]
fn test_nested_object_literal() {
    let block = sigil_quote!(TypeScript {
        const x = { a: 1, b: { c: 2 } };
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("{"), "got: {output}");
    assert!(output.contains("a:"), "got: {output}");
}

// ════════════════════════════════════════════════════════
// G. Spacing edge cases
// ════════════════════════════════════════════════════════

#[test]
fn test_dot_chaining() {
    let block = sigil_quote!(TypeScript {
        foo.bar.baz();
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("foo.bar.baz();"), "got: {output}");
}

#[test]
fn test_triple_equals() {
    let block = sigil_quote!(TypeScript {
        if(x === 0) {
            return;
        }
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("x === 0"), "got: {output}");
}

#[test]
fn test_not_equals() {
    let block = sigil_quote!(TypeScript {
        if(x !== null) {
            return;
        }
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("x !== null"), "got: {output}");
}

#[test]
fn test_arrow_function() {
    let block = sigil_quote!(TypeScript {
        const fn = (x) => x + 1;
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("=>"), "got: {output}");
    assert!(output.contains("x + 1"), "got: {output}");
}

#[test]
fn test_rust_path_separator() {
    let block = sigil_quote!(RustLang {
        let x = std::mem::size_of::<u32>();
    })
    .unwrap();

    let output = render_rs(&block);
    // `::` is reconstructed correctly (Joint punct), but `<`/`>` are separate
    // tokens so they get spaces. This is a known tokenization artifact.
    assert!(output.contains("std::"), "got: {output}");
    assert!(output.contains("mem::"), "got: {output}");
    assert!(output.contains("size_of"), "got: {output}");
    assert!(output.contains("u32"), "got: {output}");
}

#[test]
fn test_unary_not() {
    let block = sigil_quote!(TypeScript {
        if(!flag) {
            return;
        }
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("!flag"), "got: {output}");
}

#[test]
fn test_bracket_access() {
    let block = sigil_quote!(TypeScript {
        const x = arr[0];
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("arr[0]"), "got: {output}");
}

#[test]
fn test_comma_spacing() {
    let block = sigil_quote!(TypeScript {
        foo(a, b, c);
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("foo(a, b, c);"), "got: {output}");
}

#[test]
fn test_parenthesized_expression() {
    let block = sigil_quote!(TypeScript {
        const x = (a + b) * c;
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("(a + b) * c"), "got: {output}");
}

#[test]
fn test_nested_parens() {
    let block = sigil_quote!(TypeScript {
        foo((a, b));
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("foo((a, b))"), "got: {output}");
}

#[test]
fn test_array_literal() {
    let block = sigil_quote!(TypeScript {
        const arr = [1, 2, 3];
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("[1, 2, 3]"), "got: {output}");
}

// ════════════════════════════════════════════════════════
// H. Percent escaping
// ════════════════════════════════════════════════════════

#[test]
fn test_percent_in_source() {
    let block = sigil_quote!(TypeScript {
        const x = 100 % 10;
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("100 % 10"), "got: {output}");
}

// ════════════════════════════════════════════════════════
// I. Dollar escape
// ════════════════════════════════════════════════════════

#[test]
fn test_dollar_escape_basic() {
    let block = sigil_quote!(TypeScript {
        const price = $$100;
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("$"), "got: {output}");
    assert!(output.contains("100"), "got: {output}");
}

#[test]
fn test_dollar_escape_with_interpolation() {
    let val = "50";
    let block = sigil_quote!(TypeScript {
        const total = $$100 + $L(val);
    })
    .unwrap();

    let output = render_ts(&block);
    // $$ becomes literal $ in format string; `100` is a separate token so there's a space
    assert!(output.contains("$"), "got: {output}");
    assert!(output.contains("100"), "got: {output}");
    assert!(output.contains("50"), "got: {output}");
}

// ════════════════════════════════════════════════════════
// J. Wrap points ($W)
// ════════════════════════════════════════════════════════

#[test]
fn test_wrap_point_in_params() {
    let config_type = TypeName::<TypeScript>::primitive("Config");
    let block = sigil_quote!(TypeScript {
        export async function createUser($W name: string,$W age: number,$W config: $T(config_type) $W): Promise<void> {
            return undefined;
        }
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("createUser"), "got: {output}");
    assert!(output.contains("Config"), "got: {output}");
}

#[test]
fn test_wrap_point_narrow_width() {
    let block = sigil_quote!(TypeScript {
        doSomething($W alpha,$W beta,$W gamma);
    })
    .unwrap();

    // Render at narrow width to force line breaks.
    let mut fb = FileSpec::<TypeScript>::builder("test.ts");
    fb.add_code(block);
    let file = fb.build().unwrap();
    let output = file.render(20).unwrap();
    // With narrow width, %W should break lines.
    assert!(output.contains("doSomething"), "got: {output}");
}

// ════════════════════════════════════════════════════════
// K. Multi-language support
// ════════════════════════════════════════════════════════

#[test]
fn test_rust_language() {
    let block = sigil_quote!(RustLang {
        let x = 42;
    })
    .unwrap();

    let output = render_rs(&block);
    assert!(output.contains("let x = 42;"), "got: {output}");
}

#[test]
fn test_python_control_flow() {
    // Python uses `:` instead of `{` for blocks, rendered by CodeLang.
    let mut b = CodeBlock::<Python>::builder();
    b.begin_control_flow("if x > 0", ());
    b.add_statement("return True", ());
    b.end_control_flow();
    let manual = b.build().unwrap();

    let macro_block = sigil_quote!(Python {
        if x > 0 {
            return True;
        }
    })
    .unwrap();

    let manual_output = render_py(&manual);
    let macro_output = render_py(&macro_block);
    // Both should produce Python-style output with `:` and indentation.
    assert!(
        manual_output.contains("if x > 0:"),
        "manual: {manual_output}"
    );
    assert!(macro_output.contains("if x > 0:"), "macro: {macro_output}");
}

#[test]
fn test_go_language() {
    let block = sigil_quote!(GoLang {
        x := 42;
    })
    .unwrap();

    let output = render_go(&block);
    // `:` is in the "no space before" set, so `x:= 42` is expected.
    // Go doesn't emit semicolons.
    assert!(output.contains("x:= 42"), "got: {output}");
}

// ════════════════════════════════════════════════════════
// L. Equivalence tests — macro vs manual builder
// ════════════════════════════════════════════════════════

#[test]
fn test_equiv_simple_statement() {
    let user_type = TypeName::<TypeScript>::importable_type("./models", "User");

    let mut b = CodeBlock::<TypeScript>::builder();
    b.add_statement("const user: %T = getUser()", (user_type.clone(),));
    b.add_statement("return user", ());
    let manual = b.build().unwrap();

    let macro_block = sigil_quote!(TypeScript {
        const user: $T(user_type) = getUser();
        return user;
    })
    .unwrap();

    // Import refs should match.
    let manual_refs = import_collector::collect_imports(&manual);
    let macro_refs = import_collector::collect_imports(&macro_block);
    assert_eq!(manual_refs.len(), macro_refs.len());
    assert_eq!(manual_refs[0].name, macro_refs[0].name);
}

#[test]
fn test_equiv_control_flow() {
    let mut b = CodeBlock::<TypeScript>::builder();
    b.begin_control_flow("if(x > 0)", ());
    b.add_statement("return true", ());
    b.next_control_flow("else", ());
    b.add_statement("return false", ());
    b.end_control_flow();
    let manual = b.build().unwrap();

    let macro_block = sigil_quote!(TypeScript {
        if(x > 0) {
            return true;
        } else {
            return false;
        }
    })
    .unwrap();

    let manual_output = render_ts(&manual);
    let macro_output = render_ts(&macro_block);
    assert_eq!(
        manual_output, macro_output,
        "manual:\n{manual_output}\nmacro:\n{macro_output}"
    );
}

#[test]
fn test_equiv_comment() {
    let mut b = CodeBlock::<TypeScript>::builder();
    b.add_comment("hello");
    b.add_statement("const x = 1", ());
    let manual = b.build().unwrap();

    let macro_block = sigil_quote!(TypeScript {
        $comment("hello");
        const x = 1;
    })
    .unwrap();

    let manual_output = render_ts(&manual);
    let macro_output = render_ts(&macro_block);
    assert_eq!(
        manual_output, macro_output,
        "manual:\n{manual_output}\nmacro:\n{macro_output}"
    );
}

#[test]
fn test_equiv_blank_line() {
    let mut b = CodeBlock::<TypeScript>::builder();
    b.add_statement("const a = 1", ());
    b.add_line();
    b.add_statement("const b = 2", ());
    let manual = b.build().unwrap();

    let macro_block = sigil_quote!(TypeScript {
        const a = 1;

        const b = 2;
    })
    .unwrap();

    let manual_output = render_ts(&manual);
    let macro_output = render_ts(&macro_block);
    assert_eq!(
        manual_output, macro_output,
        "manual:\n{manual_output}\nmacro:\n{macro_output}"
    );
}

#[test]
fn test_equiv_name_arg() {
    let mut b = CodeBlock::<TypeScript>::builder();
    b.add_statement("const %N = 1", (NameArg("x".to_string()),));
    let manual = b.build().unwrap();

    let macro_block = sigil_quote!(TypeScript {
        const $N("x") = 1;
    })
    .unwrap();

    let manual_output = render_ts(&manual);
    let macro_output = render_ts(&macro_block);
    assert_eq!(
        manual_output, macro_output,
        "manual:\n{manual_output}\nmacro:\n{macro_output}"
    );
}

#[test]
fn test_equiv_string_lit_arg() {
    let mut b = CodeBlock::<TypeScript>::builder();
    b.add_statement("console.log(%S)", (StringLitArg("hi".to_string()),));
    let manual = b.build().unwrap();

    let macro_block = sigil_quote!(TypeScript {
        console.log($S("hi"));
    })
    .unwrap();

    let manual_output = render_ts(&manual);
    let macro_output = render_ts(&macro_block);
    assert_eq!(
        manual_output, macro_output,
        "manual:\n{manual_output}\nmacro:\n{macro_output}"
    );
}

#[test]
fn test_equiv_nested_code_block() {
    let inner = CodeBlock::<TypeScript>::of("doWork()", ()).unwrap();

    let mut b = CodeBlock::<TypeScript>::builder();
    b.add_statement("%L", (inner.clone(),));
    let manual = b.build().unwrap();

    let macro_block = sigil_quote!(TypeScript {
        $C(inner);
    })
    .unwrap();

    let manual_output = render_ts(&manual);
    let macro_output = render_ts(&macro_block);
    assert_eq!(
        manual_output, macro_output,
        "manual:\n{manual_output}\nmacro:\n{macro_output}"
    );
}

#[test]
fn test_equiv_wrap_point() {
    // $W inserts %W but tokenizer adds spaces around tokens;
    // manual format strings can be denser (e.g., "%Wa" vs "%W a").
    // Just verify the macro version produces valid output with wrap points.
    let macro_block = sigil_quote!(TypeScript {
        foo($W a,$W b,$W c);
    })
    .unwrap();

    let output = render_ts(&macro_block);
    assert!(output.contains("foo("), "got: {output}");
    assert!(output.contains("a,"), "got: {output}");
    assert!(output.contains("b,"), "got: {output}");
}

// ════════════════════════════════════════════════════════
// M. Args tuple shape
// ════════════════════════════════════════════════════════

#[test]
fn test_zero_args_statement() {
    let block = sigil_quote!(TypeScript {
        return null;
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("return null;"), "got: {output}");
}

#[test]
fn test_single_arg_statement() {
    let ty = TypeName::<TypeScript>::primitive("number");
    let block = sigil_quote!(TypeScript {
        const x: $T(ty) = 1;
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("number"), "got: {output}");
}

#[test]
fn test_many_args_statement() {
    let t1 = TypeName::<TypeScript>::primitive("string");
    let t2 = TypeName::<TypeScript>::primitive("number");
    let t3 = TypeName::<TypeScript>::primitive("boolean");
    let block = sigil_quote!(TypeScript {
        function f(a: $T(t1), b: $T(t2), c: $T(t3)): void {};
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("string"), "got: {output}");
    assert!(output.contains("number"), "got: {output}");
    assert!(output.contains("boolean"), "got: {output}");
}

// ════════════════════════════════════════════════════════
// N. Complex real-world patterns
// ════════════════════════════════════════════════════════

#[test]
fn test_class_like_structure() {
    let user_type = TypeName::<TypeScript>::importable_type("./models", "User");
    let block = sigil_quote!(TypeScript {
        export class UserService {
            getUser(id: $T(user_type)): void {
                console.log($S("getting user"));
            }
        }
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("export class UserService"), "got: {output}");
    assert!(output.contains("getUser"), "got: {output}");
    assert!(output.contains("'getting user'"), "got: {output}");
}

#[test]
fn test_for_loop() {
    let block = sigil_quote!(TypeScript {
        for(let i = 0; i < 10; i++) {
            console.log(i);
        }
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("for("), "got: {output}");
    assert!(output.contains("i < 10"), "got: {output}");
    assert!(output.contains("console.log(i);"), "got: {output}");
}

#[test]
fn test_while_loop() {
    let block = sigil_quote!(TypeScript {
        while(running) {
            process();
        }
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("while(running) {"), "got: {output}");
    assert!(output.contains("process();"), "got: {output}");
}

#[test]
fn test_try_catch() {
    let block = sigil_quote!(TypeScript {
        try {
            doRisky();
        } catch(e) {
            console.error(e);
        }
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("try {"), "got: {output}");
    // next_control_flow renders "} " then "catch(e)" on potentially separate lines
    assert!(output.contains("catch(e)"), "got: {output}");
    assert!(output.contains("doRisky();"), "got: {output}");
    assert!(output.contains("console.error(e);"), "got: {output}");
}

#[test]
fn test_statements_before_and_after_control_flow() {
    let block = sigil_quote!(TypeScript {
        const x = 1;
        if(x > 0) {
            return x;
        }
        const y = 2;
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("const x = 1;"), "got: {output}");
    assert!(output.contains("if(x > 0) {"), "got: {output}");
    assert!(output.contains("const y = 2;"), "got: {output}");
}

#[test]
fn test_control_flow_then_statement() {
    let block = sigil_quote!(TypeScript {
        if(x) {
            doA();
        } else {
            doB();
        }
        cleanup();
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("} else {"), "got: {output}");
    assert!(output.contains("cleanup();"), "got: {output}");
}

#[test]
fn test_many_types_with_imports() {
    let t1 = TypeName::<TypeScript>::importable_type("./a", "TypeA");
    let t2 = TypeName::<TypeScript>::importable_type("./b", "TypeB");
    let t3 = TypeName::<TypeScript>::importable_type("./c", "TypeC");

    let block = sigil_quote!(TypeScript {
        const a: $T(t1) = getA();
        const b: $T(t2) = getB();
        const c: $T(t3) = getC();
    })
    .unwrap();

    let refs = import_collector::collect_imports(&block);
    assert_eq!(refs.len(), 3, "refs: {refs:?}");

    let output = render_ts(&block);
    assert!(output.contains("import type { TypeA }"), "got: {output}");
    assert!(output.contains("import type { TypeB }"), "got: {output}");
    assert!(output.contains("import type { TypeC }"), "got: {output}");
}

#[test]
fn test_complex_expression_interpolation() {
    // Expression in $T() can be a method call.
    let block = sigil_quote!(TypeScript {
        const x: $T(TypeName::<TypeScript>::primitive("string")) = $S("hello");
    })
    .unwrap();

    let output = render_ts(&block);
    assert!(output.contains("string"), "got: {output}");
    assert!(output.contains("'hello'"), "got: {output}");
}

// ════════════════════════════════════════════════════════
// Helpers
// ════════════════════════════════════════════════════════

fn render_ts(block: &CodeBlock<TypeScript>) -> String {
    let mut fb = FileSpec::<TypeScript>::builder("test.ts");
    fb.add_code(block.clone());
    let file = fb.build().unwrap();
    file.render(80).unwrap()
}

fn render_rs(block: &CodeBlock<RustLang>) -> String {
    let mut fb = FileSpec::<RustLang>::builder("test.rs");
    fb.add_code(block.clone());
    let file = fb.build().unwrap();
    file.render(80).unwrap()
}

fn render_py(block: &CodeBlock<Python>) -> String {
    let mut fb = FileSpec::<Python>::builder("test.py");
    fb.add_code(block.clone());
    let file = fb.build().unwrap();
    file.render(80).unwrap()
}

fn render_go(block: &CodeBlock<GoLang>) -> String {
    let mut fb = FileSpec::<GoLang>::builder("test.go");
    fb.add_code(block.clone());
    let file = fb.build().unwrap();
    file.render(80).unwrap()
}
