//! Example: Generate a JavaScript module with sigil-stitch.
//!
//! Demonstrates:
//! - `import { X } from 'module'` (ESM imports, no `import type`)
//! - Class with `#private` fields and constructor
//! - Methods without type annotations
//! - `export class` and `export function`
//! - Class inheritance with `extends`
//! - JSDoc comments
//!
//! Run: `cargo run --example js_codegen`

use sigil_stitch::code_block::{CodeBlock, StringLitArg};
use sigil_stitch::lang::javascript::JavaScript;
use sigil_stitch::spec::field_spec::FieldSpec;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::modifiers::{TypeKind, Visibility};
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::spec::type_spec::TypeSpec;
use sigil_stitch::type_name::TypeName;

/// Shorthand for a JS parameter (no type annotation).
fn param(name: &str) -> ParameterSpec {
    ParameterSpec::new(name, TypeName::primitive("")).unwrap()
}

/// Shorthand for a JS field (no type annotation).
fn field(name: &str) -> FieldSpec {
    FieldSpec::builder(name, TypeName::primitive(""))
        .build()
        .unwrap()
}

fn main() {
    // --- Imports (triggered by usage in code) ---
    let event_emitter = TypeName::importable("events", "EventEmitter");
    let uuid = TypeName::importable("uuid", "v4");
    let format = TypeName::importable("./utils", "formatMessage");

    // --- Base class: Logger ---
    let logger_tb = TypeSpec::builder("Logger", TypeKind::Class);
    let logger_tb = logger_tb.visibility(Visibility::Public);
    let logger_tb = logger_tb.doc("Base logger class.");
    let logger_tb = logger_tb.doc("");
    let logger_tb = logger_tb.doc("@abstract");

    let logger_tb = logger_tb.add_field(field("#name"));
    let logger_tb = logger_tb.add_field(field("#level"));

    // Constructor
    let ctor_body =
        CodeBlock::of("this.#name = name;\nthis.#level = level || 'info';", ()).unwrap();
    let logger_tb = logger_tb.add_method(
        FunSpec::builder("constructor")
            .add_param(param("name"))
            .add_param(param("level"))
            .body(ctor_body)
            .build()
            .unwrap(),
    );

    // getName method
    let get_name_body = CodeBlock::of("return this.#name;", ()).unwrap();
    let logger_tb = logger_tb.add_method(
        FunSpec::builder("getName")
            .body(get_name_body)
            .build()
            .unwrap(),
    );

    let logger = logger_tb.build().unwrap();

    // --- Derived class: ConsoleLogger ---
    let console_tb = TypeSpec::builder("ConsoleLogger", TypeKind::Class);
    let console_tb = console_tb.visibility(Visibility::Public);
    let console_tb = console_tb.extends(TypeName::primitive("Logger"));
    let console_tb = console_tb.doc("Logger that writes formatted messages to the console.");

    // Constructor
    let ctor_body2 = CodeBlock::of("super(name, 'info');", ()).unwrap();
    let console_tb = console_tb.add_method(
        FunSpec::builder("constructor")
            .add_param(param("name"))
            .body(ctor_body2)
            .build()
            .unwrap(),
    );

    // log method — uses imports
    let log_body = CodeBlock::of(
        "const msg = %T(this.getName(), message);\nconsole.log(msg);",
        (format,),
    )
    .unwrap();
    let console_tb = console_tb.add_method(
        FunSpec::builder("log")
            .add_param(param("message"))
            .body(log_body)
            .build()
            .unwrap(),
    );

    let console_logger = console_tb.build().unwrap();

    // --- EventBus class ---
    let bus_tb = TypeSpec::builder("EventBus", TypeKind::Class);
    let bus_tb = bus_tb.visibility(Visibility::Public);
    let bus_tb = bus_tb.extends(TypeName::primitive("EventEmitter"));
    let bus_tb = bus_tb.doc("Publish-subscribe event bus.");

    let bus_tb = bus_tb.add_field(field("#subscribers"));

    let bus_ctor_body = CodeBlock::of("super();\nthis.#subscribers = new Map();", ()).unwrap();
    let bus_tb = bus_tb.add_method(
        FunSpec::builder("constructor")
            .body(bus_ctor_body)
            .build()
            .unwrap(),
    );

    let emit_body = CodeBlock::of(
        "const id = %T();\nthis.emit(event, { id, ...data });\nreturn id;",
        (uuid,),
    )
    .unwrap();
    let bus_tb = bus_tb.add_method(
        FunSpec::builder("publish")
            .add_param(param("event"))
            .add_param(param("data"))
            .body(emit_body)
            .build()
            .unwrap(),
    );

    let event_bus = bus_tb.build().unwrap();

    // --- Standalone exported functions ---
    let create_logger_body = CodeBlock::of("return new ConsoleLogger(name);", ()).unwrap();
    let create_logger_fn = FunSpec::builder("createLogger")
        .visibility(Visibility::Public)
        .add_param(param("name"))
        .body(create_logger_body)
        .build()
        .unwrap();

    let create_bus_body = CodeBlock::of("return new EventBus();", ()).unwrap();
    let create_bus_fn = FunSpec::builder("createEventBus")
        .visibility(Visibility::Public)
        .body(create_bus_body)
        .build()
        .unwrap();

    // Async function
    let fetch_body = CodeBlock::of(
        "const response = await fetch(url);\nif (!response.ok) {\n  throw new Error(%S + response.status);\n}\nreturn response.json();",
        (StringLitArg("HTTP error: ".to_string()),),
    )
    .unwrap();
    let fetch_json = FunSpec::builder("fetchJSON")
        .visibility(Visibility::Public)
        .is_async()
        .add_param(param("url"))
        .body(fetch_body)
        .build()
        .unwrap();

    // --- Trigger imports for extends base types ---
    let import_trigger = CodeBlock::of("// Base classes: %T", (event_emitter,)).unwrap();

    // --- Assemble file ---
    let file = FileSpec::builder_with("app.js", JavaScript::new())
        .add_code(import_trigger)
        .add_type(logger)
        .add_type(console_logger)
        .add_type(event_bus)
        .add_function(create_logger_fn)
        .add_function(create_bus_fn)
        .add_function(fetch_json)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();
    print!("{output}");
}
