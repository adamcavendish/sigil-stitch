//! Generate a Lua file — builder API vs `sigil_quote!` comparison.
//!
//! Demonstrates: functions with `end` block close, table constructors via
//! raw CodeBlock, `require` imports, and control flow (`if/then/end`).
//! The macro approach uses `{ }` blocks that the Lua backend automatically
//! converts to the correct delimiters (`then`/`end`, `do`/`end`).
//!
//! Run: `cargo run --example lua_codegen`

use sigil_stitch::lang::lua::Lua;
use sigil_stitch::prelude::*;

fn main() {
    println!("=== Builder API ===\n");
    let builder_output = builder_approach();
    println!("{builder_output}");

    println!("=== sigil_quote! Macro ===\n");
    let macro_output = macro_approach();
    println!("{macro_output}");
}

fn builder_approach() -> String {
    let json = TypeName::importable("cjson", "json");
    let comment_reason = "Initialize module state";
    let comment_label = "INFO";
    let v_interp = "module";
    let comment_note = "validate input";

    // --- Simple function ---
    let mut greet_body = CodeBlock::builder();
    greet_body.add_comment(&format!("{}: {}", comment_label, comment_reason));
    greet_body.add("local mod = %V", (VerbatimStrArg(v_interp.to_string()),));
    greet_body.add_line();
    greet_body.add(
        "return \"Hello, \" .. name .. \"!\" %R",
        (CommentArg(comment_note.to_string()),),
    );

    let greet_fn = FunSpec::builder("greet")
        .doc("Returns a greeting string.")
        .add_param(ParameterSpec::new("name", TypeName::primitive("")).unwrap())
        .body(greet_body.build().unwrap())
        .build()
        .unwrap();

    // --- Function with control flow ---
    let mut create_body = CodeBlock::builder();
    create_body.add("local config = {}", ());
    create_body.add_line();
    create_body.add("config.host = host or \"localhost\"", ());
    create_body.add_line();
    create_body.add("config.port = port or 8080", ());
    create_body.add_line();
    create_body.add_line();
    create_body.add("%<", ());
    create_body.add("if config.port < 1024 then", ());
    create_body.add_line();
    create_body.add("%>", ());
    create_body.add("print(\"Warning: privileged port\")", ());
    create_body.add_line();
    create_body.add("%<", ());
    create_body.add("end", ());
    create_body.add_line();
    create_body.add("%>", ());
    create_body.add_line();
    create_body.add("return config", ());

    let create_fn = FunSpec::builder("create_config")
        .add_param(ParameterSpec::new("host", TypeName::primitive("")).unwrap())
        .add_param(ParameterSpec::new("port", TypeName::primitive("")).unwrap())
        .body(create_body.build().unwrap())
        .build()
        .unwrap();

    // --- Serialize function using imported module ---
    let mut serialize_body = CodeBlock::builder();
    serialize_body.add_statement("return %T.encode(data)", (json,));

    let serialize_fn = FunSpec::builder("serialize")
        .add_param(ParameterSpec::new("data", TypeName::primitive("")).unwrap())
        .body(serialize_body.build().unwrap())
        .build()
        .unwrap();

    FileSpec::builder_with("config.lua", Lua::new())
        .add_function(greet_fn)
        .add_function(create_fn)
        .add_function(serialize_fn)
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

fn macro_approach() -> String {
    let json = TypeName::importable("cjson", "json");
    let comment_reason = "Initialize module state";
    let comment_label = "INFO";
    let v_interp = "module";
    let comment_note = "validate input";

    let greet_body = sigil_quote!(Lua {
        $comment("@{comment_label}: @{comment_reason}");
        local mod = $V("@{v_interp}")
        return "Hello, " .. name .. "!" $comment(comment_note)
    })
    .unwrap();

    let greet_fn = FunSpec::builder("greet")
        .doc("Returns a greeting string.")
        .add_param(ParameterSpec::new("name", TypeName::primitive("")).unwrap())
        .body(greet_body)
        .build()
        .unwrap();

    // With context-aware block delimiters, Lua control flow now uses { }
    // in sigil_quote! — the backend emits then/end, do/end automatically.
    let create_body = sigil_quote!(Lua {
        local config = {}
        config.host = host or "localhost"
        config.port = port or 8080

        if config.port < 1024 {
            print("Warning: privileged port")
        }

        return config
    })
    .unwrap();

    let create_fn = FunSpec::builder("create_config")
        .add_param(ParameterSpec::new("host", TypeName::primitive("")).unwrap())
        .add_param(ParameterSpec::new("port", TypeName::primitive("")).unwrap())
        .body(create_body)
        .build()
        .unwrap();

    let serialize_body = sigil_quote!(Lua {
        return $T(json).encode(data)
    })
    .unwrap();

    let serialize_fn = FunSpec::builder("serialize")
        .add_param(ParameterSpec::new("data", TypeName::primitive("")).unwrap())
        .body(serialize_body)
        .build()
        .unwrap();

    FileSpec::builder_with("config.lua", Lua::new())
        .add_function(greet_fn)
        .add_function(create_fn)
        .add_function(serialize_fn)
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}
