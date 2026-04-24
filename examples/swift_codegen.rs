//! Example: Generate a Swift source file with sigil-stitch.
//!
//! Demonstrates:
//! - `import` statements with Apple framework / third-party grouping
//! - Generic protocol with method requirements
//! - Struct (value type) with `let` properties
//! - Class with inheritance and protocol conformance (single `:`)
//! - Enum with cases
//! - `async func` for concurrency
//! - `override` via annotations
//! - `///` Swift Markup doc comments
//! - `let`/`var` properties
//! - `@objc` and `@discardableResult` attributes
//!
//! Run: `cargo run --example swift_codegen`

use sigil_stitch::code_block::{CodeBlock, StringLitArg};
use sigil_stitch::lang::swift::Swift;
use sigil_stitch::spec::field_spec::FieldSpec;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::{FunSpec, TypeParamSpec};
use sigil_stitch::spec::modifiers::{TypeKind, Visibility};
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::spec::type_spec::TypeSpec;
use sigil_stitch::type_name::TypeName;

fn main() {
    // --- Imports (triggered by usage in code) ---
    let url = TypeName::importable("Foundation", "URL");
    let data = TypeName::importable("Foundation", "Data");
    let json_decoder = TypeName::importable("Foundation", "JSONDecoder");
    let publisher = TypeName::importable("Combine", "AnyPublisher");
    let url_session = TypeName::importable("Foundation", "URLSession");

    // --- Enum: Priority ---
    let priority = TypeSpec::builder("Priority", TypeKind::Enum);
    let priority = priority.visibility(Visibility::Public);
    let priority = priority.doc("Task priority levels.");

    let mut cases = CodeBlock::builder();
    cases.add("case low", ());
    cases.add_line();
    cases.add("case medium", ());
    cases.add_line();
    cases.add("case high", ());
    cases.add_line();
    cases.add("case critical", ());
    cases.add_line();
    let priority = priority.extra_member(cases.build().unwrap());

    let priority_spec = priority.build().unwrap();

    // --- Protocol: TaskRepository ---
    let tp = TypeParamSpec::new("T");

    let repo_spec = TypeSpec::builder("TaskRepository", TypeKind::Interface)
        .add_type_param(tp)
        .doc("Repository for task persistence.")
        .doc("")
        .doc("- Parameter T: the task entity type")
        .add_method(
            FunSpec::builder("findById")
                .returns(TypeName::primitive("T?"))
                .add_param(ParameterSpec::new("id", TypeName::primitive("String")).unwrap())
                .build()
                .unwrap(),
        )
        .add_method(
            FunSpec::builder("findAll")
                .returns(TypeName::primitive("[T]"))
                .build()
                .unwrap(),
        )
        .add_method(
            FunSpec::builder("save")
                .add_param(ParameterSpec::new("entity", TypeName::primitive("T")).unwrap())
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    // --- Struct: Task ---
    let task_spec = TypeSpec::builder("Task", TypeKind::Struct)
        .visibility(Visibility::Public)
        .doc("A task entity.")
        .add_field(
            FieldSpec::builder("id", TypeName::primitive("String"))
                .visibility(Visibility::Public)
                .is_readonly()
                .build()
                .unwrap(),
        )
        .add_field(
            FieldSpec::builder("name", TypeName::primitive("String"))
                .visibility(Visibility::Public)
                .is_readonly()
                .build()
                .unwrap(),
        )
        .add_field(
            FieldSpec::builder("priority", TypeName::primitive("Priority"))
                .visibility(Visibility::Public)
                .is_readonly()
                .build()
                .unwrap(),
        )
        .add_field(
            FieldSpec::builder("completed", TypeName::primitive("Bool"))
                .visibility(Visibility::Public)
                .initializer(CodeBlock::of("false", ()).unwrap())
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    // --- Class: BaseService ---
    let base_svc = TypeSpec::builder("BaseService", TypeKind::Class);
    let base_svc = base_svc.visibility(Visibility::Public);
    let base_svc = base_svc.doc("Base class for services with logging.");

    let base_svc = base_svc.add_field(
        FieldSpec::builder("serviceName", TypeName::primitive("String"))
            .visibility(Visibility::Public)
            .is_readonly()
            .build()
            .unwrap(),
    );

    let log_body = CodeBlock::of("print(\"[\\(serviceName)] \\(message)\")", ()).unwrap();
    let base_svc = base_svc.add_method(
        FunSpec::builder("log")
            .add_param(ParameterSpec::new("message", TypeName::primitive("String")).unwrap())
            .body(log_body)
            .build()
            .unwrap(),
    );

    let base_svc_spec = base_svc.build().unwrap();

    // --- Class: TaskService extends BaseService, conforms to TaskRepository ---
    let task_svc = TypeSpec::builder("TaskService", TypeKind::Class);
    let task_svc = task_svc.visibility(Visibility::Public);
    let task_svc = task_svc.extends(TypeName::primitive("BaseService"));
    let task_svc = task_svc.extends(TypeName::primitive("TaskRepository"));
    let task_svc = task_svc.doc("Task management service.");

    let task_svc = task_svc.add_field(
        FieldSpec::builder("tasks", TypeName::primitive("[Task]"))
            .visibility(Visibility::Private)
            .initializer(CodeBlock::of("[]", ()).unwrap())
            .build()
            .unwrap(),
    );

    // findById
    let find_body = CodeBlock::of("return tasks.first { $0.id == id }", ()).unwrap();
    let task_svc = task_svc.add_method(
        FunSpec::builder("findById")
            .returns(TypeName::primitive("Task?"))
            .add_param(ParameterSpec::new("id", TypeName::primitive("String")).unwrap())
            .body(find_body)
            .build()
            .unwrap(),
    );

    // findAll
    let find_all_body = CodeBlock::of("return tasks", ()).unwrap();
    let task_svc = task_svc.add_method(
        FunSpec::builder("findAll")
            .returns(TypeName::primitive("[Task]"))
            .body(find_all_body)
            .build()
            .unwrap(),
    );

    // save
    let save_body = CodeBlock::of("tasks.append(entity)", ()).unwrap();
    let task_svc = task_svc.add_method(
        FunSpec::builder("save")
            .add_param(ParameterSpec::new("entity", TypeName::primitive("Task")).unwrap())
            .body(save_body)
            .build()
            .unwrap(),
    );

    let task_svc_spec = task_svc.build().unwrap();

    // --- Async function: fetchTasks ---
    let fetch_body = CodeBlock::of(
        "let (responseData, _) = try await %T.shared.data(from: endpoint)\nlet decoder = %T()\nreturn try decoder.decode([Task].self, from: responseData)",
        (url_session, json_decoder),
    )
    .unwrap();
    let fetch_tasks = FunSpec::builder("fetchTasks")
        .visibility(Visibility::Public)
        .is_async()
        .returns(TypeName::primitive("[Task]"))
        .add_param(ParameterSpec::new("endpoint", TypeName::primitive("URL")).unwrap())
        .body(fetch_body)
        .build()
        .unwrap();

    // --- Function using URL + Combine ---
    let create_body = CodeBlock::of(
        "guard let url = %T(string: urlString) else {\n    fatalError(%S)\n}\nreturn url",
        (url, StringLitArg("Invalid URL".to_string())),
    )
    .unwrap();
    let make_url = FunSpec::builder("makeURL")
        .returns(TypeName::primitive("URL"))
        .add_param(ParameterSpec::new("urlString", TypeName::primitive("String")).unwrap())
        .body(create_body)
        .build()
        .unwrap();

    // Trigger Combine import
    let combine_trigger = CodeBlock::of("// Publisher: %T", (publisher,)).unwrap();

    // Trigger Data import
    let data_trigger = CodeBlock::of("// Data: %T", (data,)).unwrap();

    // --- Assemble file ---
    let file = FileSpec::builder_with("TaskApp.swift", Swift::new())
        .add_code(combine_trigger)
        .add_code(data_trigger)
        .add_type(priority_spec)
        .add_type(repo_spec)
        .add_type(task_spec)
        .add_type(base_svc_spec)
        .add_type(task_svc_spec)
        .add_function(fetch_tasks)
        .add_function(make_url)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();
    print!("{output}");
}
