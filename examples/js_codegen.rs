//! Generate a JavaScript module — builder API vs `sigil_quote!` comparison.
//!
//! Demonstrates: class with static factory method, async functions, control flow
//! (if/else, try/catch via `next_control_flow`), default parameter values, and
//! generator functions (`function*`).
//!
//! Run: `cargo run --example js_codegen`

use sigil_stitch::lang::javascript::JavaScript;
use sigil_stitch::prelude::*;

fn param(name: &str) -> ParameterSpec {
    ParameterSpec::new(name, TypeName::primitive("")).unwrap()
}

fn main() {
    println!("=== Builder API ===\n");
    let builder_output = builder_approach();
    println!("{builder_output}");

    println!("=== sigil_quote! Macro ===\n");
    let macro_output = macro_approach();
    println!("{macro_output}");
}

fn builder_approach() -> String {
    let format_msg = TypeName::importable("./utils", "formatMessage");
    let event_emitter = TypeName::importable("events", "EventEmitter");
    let comment_label = "FIXME";
    let comment_reason = "Format log message";
    let comment_note = "console output";
    let v_interp = "service";

    let mut ctor_body = CodeBlock::builder();
    ctor_body.add_comment(&format!("{}: {}", comment_label, comment_reason));
    ctor_body.add_statement("super()", ());
    ctor_body.add_statement(
        "this.name = name; // %V %R",
        (
            VerbatimStrArg(v_interp.to_string()),
            CommentArg(comment_note.to_string()),
        ),
    );

    let mut log_body = CodeBlock::builder();
    log_body.add_statement("const msg = %T(this.name, message)", (format_msg,));
    log_body.add_statement("console.log(msg)", ());

    // Static factory method: Logger.create(name)
    let mut create_body = CodeBlock::builder();
    create_body.add_attribute("deprecated");
    create_body.add_statement("return new Logger(name)", ());

    let logger = TypeSpec::builder("Logger", TypeKind::Class)
        .visibility(Visibility::Public)
        .extends(TypeName::primitive("EventEmitter"))
        .doc("Logger with event emission.")
        .add_method(
            FunSpec::builder("constructor")
                .add_param(param("name"))
                .body(ctor_body.build().unwrap())
                .build()
                .unwrap(),
        )
        .add_method(
            FunSpec::builder("log")
                .add_param(param("message"))
                .body(log_body.build().unwrap())
                .build()
                .unwrap(),
        )
        .add_method(
            FunSpec::builder("create")
                .is_static()
                .add_param(param("name"))
                .body(create_body.build().unwrap())
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    // Async fetchJSON with if-check
    let mut fetch_body = CodeBlock::builder();
    fetch_body.add_statement("const response = await fetch(url)", ());
    fetch_body.begin_control_flow("if (!response.ok)", ());
    fetch_body.add_statement(
        "throw new Error(%S + response.status)",
        (StringLitArg("HTTP error: ".into()),),
    );
    fetch_body.end_control_flow();
    fetch_body.add_statement("return response.json()", ());

    let fetch_fn = FunSpec::builder("fetchJSON")
        .visibility(Visibility::Public)
        .is_async()
        .add_param(param("url"))
        .body(fetch_body.build().unwrap())
        .build()
        .unwrap();

    // Default parameter value: configure(host, port = 8080)
    let mut configure_body = CodeBlock::builder();
    configure_body.add_statement("console.log(host, port)", ());

    let configure_fn = FunSpec::builder("configure")
        .visibility(Visibility::Public)
        .add_param(param("host"))
        .add_param(
            ParameterSpec::builder("port", TypeName::primitive(""))
                .default_value(CodeBlock::of("8080", ()).unwrap())
                .build()
                .unwrap(),
        )
        .body(configure_body.build().unwrap())
        .build()
        .unwrap();

    // try/catch with next_control_flow: safeParse(input)
    let mut parse_body = CodeBlock::builder();
    parse_body.begin_control_flow("try", ());
    parse_body.add_statement("return JSON.parse(input)", ());
    parse_body.next_control_flow("catch (e)", ());
    parse_body.add_statement("return null", ());
    parse_body.end_control_flow();

    let safe_parse_fn = FunSpec::builder("safeParse")
        .visibility(Visibility::Public)
        .add_param(param("input"))
        .body(parse_body.build().unwrap())
        .build()
        .unwrap();

    // Generator function: function* range(start, end)
    let mut range_body = CodeBlock::builder();
    range_body.begin_control_flow("for (let i = start; i < end; i++)", ());
    range_body.add_statement("yield i", ());
    range_body.end_control_flow();

    let range_fn = FunSpec::builder("*range")
        .visibility(Visibility::Public)
        .add_param(param("start"))
        .add_param(param("end"))
        .body(range_body.build().unwrap())
        .build()
        .unwrap();

    let trigger = CodeBlock::of("// %T", (event_emitter,)).unwrap();

    FileSpec::builder_with("app.js", JavaScript::new())
        .add_code(trigger)
        .add_type(logger)
        .add_function(fetch_fn)
        .add_function(configure_fn)
        .add_function(safe_parse_fn)
        .add_function(range_fn)
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

fn macro_approach() -> String {
    let format_msg = TypeName::importable("./utils", "formatMessage");
    let event_emitter = TypeName::importable("events", "EventEmitter");
    let comment_label = "FIXME";
    let comment_reason = "Format log message";
    let comment_note = "console output";
    let v_interp = "service";

    let ctor_body = sigil_quote!(JavaScript {
        $comment("@{comment_label}: @{comment_reason}");
        super();
        this.name = name; $comment(comment_note)
        $V("// @{v_interp} ready");
    })
    .unwrap();

    let log_body = sigil_quote!(JavaScript {
        const msg = $T(format_msg)(this.name, message);
        console.log(msg);
    })
    .unwrap();

    // Static factory method body
    let create_body = sigil_quote!(JavaScript {
        $attr("deprecated");
        return new Logger(name);
    })
    .unwrap();

    let logger = TypeSpec::builder("Logger", TypeKind::Class)
        .visibility(Visibility::Public)
        .extends(TypeName::primitive("EventEmitter"))
        .doc("Logger with event emission.")
        .add_method(
            FunSpec::builder("constructor")
                .add_param(param("name"))
                .body(ctor_body)
                .build()
                .unwrap(),
        )
        .add_method(
            FunSpec::builder("log")
                .add_param(param("message"))
                .body(log_body)
                .build()
                .unwrap(),
        )
        .add_method(
            FunSpec::builder("create")
                .is_static()
                .add_param(param("name"))
                .body(create_body)
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    // Async fetchJSON
    let fetch_body = sigil_quote!(JavaScript {
        const response = await fetch(url);
        if(!response.ok) {
            throw new Error($S("HTTP error: ") + response.status);
        }
        return response.json();
    })
    .unwrap();

    let fetch_fn = FunSpec::builder("fetchJSON")
        .visibility(Visibility::Public)
        .is_async()
        .add_param(param("url"))
        .body(fetch_body)
        .build()
        .unwrap();

    // Default parameter value: configure(host, port = 8080)
    let configure_body = sigil_quote!(JavaScript {
        console.log(host, port);
    })
    .unwrap();

    let configure_fn = FunSpec::builder("configure")
        .visibility(Visibility::Public)
        .add_param(param("host"))
        .add_param(
            ParameterSpec::builder("port", TypeName::primitive(""))
                .default_value(CodeBlock::of("8080", ()).unwrap())
                .build()
                .unwrap(),
        )
        .body(configure_body)
        .build()
        .unwrap();

    // try/catch with sigil_quote!
    let parse_body = sigil_quote!(JavaScript {
        try {
            return JSON.parse(input);
        } catch(e) {
            return null;
        }
    })
    .unwrap();

    let safe_parse_fn = FunSpec::builder("safeParse")
        .visibility(Visibility::Public)
        .add_param(param("input"))
        .body(parse_body)
        .build()
        .unwrap();

    // Generator function: function* range(start, end)
    let range_body = sigil_quote!(JavaScript {
        for(let i = start; i < end; i++) {
            yield i;
        }
    })
    .unwrap();

    let range_fn = FunSpec::builder("*range")
        .visibility(Visibility::Public)
        .add_param(param("start"))
        .add_param(param("end"))
        .body(range_body)
        .build()
        .unwrap();

    let trigger = CodeBlock::of("// %T", (event_emitter,)).unwrap();

    FileSpec::builder_with("app.js", JavaScript::new())
        .add_code(trigger)
        .add_type(logger)
        .add_function(fetch_fn)
        .add_function(configure_fn)
        .add_function(safe_parse_fn)
        .add_function(range_fn)
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}
