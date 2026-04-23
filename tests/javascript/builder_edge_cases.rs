use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::javascript::JavaScript;
use sigil_stitch::spec::field_spec::FieldSpec;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::modifiers::{TypeKind, Visibility};
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::spec::type_spec::TypeSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

/// Shorthand for a JS parameter (no type annotation).
fn param(name: &str) -> ParameterSpec<JavaScript> {
    ParameterSpec::new(name, TypeName::primitive("")).unwrap()
}

/// Shorthand for a JS field (no type annotation).
fn field(name: &str) -> FieldSpec<JavaScript> {
    FieldSpec::builder(name, TypeName::primitive(""))
        .build()
        .unwrap()
}

#[test]
fn test_full_module() {
    let event_emitter = TypeName::<JavaScript>::importable("events", "EventEmitter");
    let uuid = TypeName::<JavaScript>::importable("uuid", "v4");

    // EventBus class extending EventEmitter.
    let mut tb = TypeSpec::<JavaScript>::builder("EventBus", TypeKind::Class);
    tb.visibility(Visibility::Public);
    tb.extends(TypeName::primitive("EventEmitter"));
    tb.doc("Application event bus.");

    tb.add_field(field("#handlers"));

    let ctor_body =
        CodeBlock::<JavaScript>::of("super();\nthis.#handlers = new Map();", ()).unwrap();
    let mut ctor = FunSpec::<JavaScript>::builder("constructor");
    ctor.body(ctor_body);
    tb.add_method(ctor.build().unwrap());

    let pub_body = CodeBlock::<JavaScript>::of(
        "const id = %T();\nthis.emit(event, data);\nreturn id;",
        (uuid,),
    )
    .unwrap();
    let mut publish = FunSpec::<JavaScript>::builder("publish");
    publish.add_param(param("event"));
    publish.add_param(param("data"));
    publish.body(pub_body);
    tb.add_method(publish.build().unwrap());

    let ts = tb.build().unwrap();

    // Trigger EventEmitter import.
    let import_trigger = CodeBlock::<JavaScript>::of("// extends %T", (event_emitter,)).unwrap();

    // Standalone exported function.
    let create_body = CodeBlock::<JavaScript>::of("return new EventBus();", ()).unwrap();
    let mut create_fn = FunSpec::<JavaScript>::builder("createEventBus");
    create_fn.visibility(Visibility::Public);
    create_fn.body(create_body);
    let create = create_fn.build().unwrap();

    let mut fb = FileSpec::builder_with("event-bus.js", JavaScript::new());
    fb.add_code(import_trigger);
    fb.add_type(ts);
    fb.add_function(create);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("javascript/full_module.js", &output);
}
