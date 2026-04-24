//! Example: Generate a Kotlin source file with sigil-stitch.
//!
//! Demonstrates:
//! - `import` statements with kotlin/kotlinx/java/javax/third-party grouping
//! - Generic interface with type parameter
//! - Abstract class with KDoc
//! - Data class
//! - Concrete class extending + implementing (all via `:`)
//! - Enum class with constants
//! - `override` annotations
//! - `suspend` functions (coroutines)
//! - `val`/`var` properties
//! - `@JvmStatic` and `@JvmField` annotations
//!
//! Run: `cargo run --example kotlin_codegen`

use sigil_stitch::code_block::{CodeBlock, StringLitArg};
use sigil_stitch::lang::kotlin::Kotlin;
use sigil_stitch::spec::field_spec::FieldSpec;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::{FunSpec, TypeParamSpec};
use sigil_stitch::spec::modifiers::{TypeKind, Visibility};
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::spec::type_spec::TypeSpec;
use sigil_stitch::type_name::TypeName;

fn main() {
    // --- Imports (triggered by usage in code) ---
    let list = TypeName::importable("kotlin.collections", "List");
    let mutable_list = TypeName::importable("kotlin.collections", "MutableList");
    let array_list = TypeName::importable("kotlin.collections", "ArrayList");
    let coroutine_scope = TypeName::importable("kotlinx.coroutines", "CoroutineScope");
    let uuid = TypeName::importable("java.util", "UUID");

    // --- Enum class: Priority ---
    let priority = TypeSpec::builder("Priority", TypeKind::Enum);
    let priority = priority.doc("Task priority levels.");

    let mut constants = CodeBlock::builder();
    constants.add("LOW,", ());
    constants.add_line();
    constants.add("MEDIUM,", ());
    constants.add_line();
    constants.add("HIGH,", ());
    constants.add_line();
    constants.add("CRITICAL", ());
    constants.add_line();
    let priority = priority.extra_member(constants.build().unwrap());

    let priority_spec = priority.build().unwrap();

    // --- Interface: Repository<T> ---
    let tp = TypeParamSpec::new("T");

    let repo_spec = TypeSpec::builder("Repository", TypeKind::Interface)
        .add_type_param(tp)
        .doc("Generic repository for data persistence.")
        .doc("")
        .doc("@param T the entity type")
        .add_method(
            FunSpec::builder("findById")
                .returns(TypeName::primitive("T?"))
                .add_param(ParameterSpec::new("id", TypeName::primitive("String")).unwrap())
                .build()
                .unwrap(),
        )
        .add_method(
            FunSpec::builder("findAll")
                .returns(list.clone())
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

    // --- Data class: Task ---
    let task_spec = TypeSpec::builder("Task", TypeKind::Struct)
        .doc("A task entity.")
        .add_field(
            FieldSpec::builder("id", TypeName::primitive("String"))
                .is_readonly()
                .build()
                .unwrap(),
        )
        .add_field(
            FieldSpec::builder("name", TypeName::primitive("String"))
                .is_readonly()
                .build()
                .unwrap(),
        )
        .add_field(
            FieldSpec::builder("priority", TypeName::primitive("Priority"))
                .is_readonly()
                .build()
                .unwrap(),
        )
        .add_field(
            FieldSpec::builder("completed", TypeName::primitive("Boolean"))
                .initializer(CodeBlock::of("false", ()).unwrap())
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    // --- Abstract class: BaseService ---
    let base_svc = TypeSpec::builder("BaseService", TypeKind::Class);
    let base_svc = base_svc.is_abstract();
    let base_svc = base_svc.doc("Base class for services with logging.");

    let base_svc = base_svc.add_field(
        FieldSpec::builder("serviceName", TypeName::primitive("String"))
            .visibility(Visibility::Protected)
            .is_readonly()
            .build()
            .unwrap(),
    );

    // Concrete method
    let log_body = CodeBlock::of("println(\"[$serviceName] $message\")", ()).unwrap();
    let base_svc = base_svc.add_method(
        FunSpec::builder("log")
            .visibility(Visibility::Protected)
            .add_param(ParameterSpec::new("message", TypeName::primitive("String")).unwrap())
            .body(log_body)
            .build()
            .unwrap(),
    );

    // Abstract method
    let base_svc = base_svc.add_method(
        FunSpec::builder("initialize")
            .is_abstract()
            .build()
            .unwrap(),
    );

    let base_svc_spec = base_svc.build().unwrap();

    // --- Concrete class: TaskService extends BaseService, implements Repository<Task> ---
    let task_svc = TypeSpec::builder("TaskService", TypeKind::Class);
    // All supertypes go into extends() for Kotlin's single `:` syntax
    let task_svc = task_svc.extends(TypeName::primitive("BaseService"));
    let task_svc = task_svc.extends(TypeName::primitive("Repository<Task>"));
    let task_svc = task_svc.doc("Task management service.");

    let task_svc = task_svc.add_field(
        FieldSpec::builder("tasks", mutable_list)
            .visibility(Visibility::Private)
            .is_readonly()
            .initializer(CodeBlock::of("%T()", (array_list,)).unwrap())
            .build()
            .unwrap(),
    );

    // initialize override
    let init_body = CodeBlock::of(
        "log(%S)",
        (StringLitArg("TaskService initialized".to_string()),),
    )
    .unwrap();
    let task_svc = task_svc.add_method(
        FunSpec::builder("initialize")
            .is_override()
            .body(init_body)
            .build()
            .unwrap(),
    );

    // findById override — trigger UUID import
    let find_body = CodeBlock::of("return tasks.firstOrNull { it.id == id }", ()).unwrap();
    let task_svc = task_svc.add_method(
        FunSpec::builder("findById")
            .returns(TypeName::primitive("Task?"))
            .add_param(ParameterSpec::new("id", TypeName::primitive("String")).unwrap())
            .is_override()
            .body(find_body)
            .build()
            .unwrap(),
    );

    // findAll override
    let find_all_body = CodeBlock::of("return %T(tasks)", (list.clone(),)).unwrap();
    let task_svc = task_svc.add_method(
        FunSpec::builder("findAll")
            .returns(list)
            .is_override()
            .body(find_all_body)
            .build()
            .unwrap(),
    );

    // save override
    let save_body = CodeBlock::of("tasks.add(entity)", ()).unwrap();
    let task_svc = task_svc.add_method(
        FunSpec::builder("save")
            .add_param(ParameterSpec::new("entity", TypeName::primitive("Task")).unwrap())
            .is_override()
            .body(save_body)
            .build()
            .unwrap(),
    );

    let task_svc_spec = task_svc.build().unwrap();

    // --- Suspend function: fetchTasks ---
    let fetch_body = CodeBlock::of(
        "val service = TaskService()\nservice.initialize()\nreturn service.findAll()",
        (),
    )
    .unwrap();
    let fetch_tasks = FunSpec::builder("fetchTasks")
        .is_async()
        .returns(TypeName::primitive("List<Task>"))
        .body(fetch_body)
        .build()
        .unwrap();

    // --- Standalone function using UUID + CoroutineScope ---
    let create_body = CodeBlock::of(
        "return Task(\n    id = %T.randomUUID().toString(),\n    name = name,\n    priority = priority\n)",
        (uuid,),
    )
    .unwrap();
    let create_task = FunSpec::builder("createTask")
        .returns(TypeName::primitive("Task"))
        .add_param(ParameterSpec::new("name", TypeName::primitive("String")).unwrap())
        .add_param(ParameterSpec::new("priority", TypeName::primitive("Priority")).unwrap())
        .body(create_body)
        .build()
        .unwrap();

    // Trigger CoroutineScope import
    let scope_trigger = CodeBlock::of("// CoroutineScope: %T", (coroutine_scope,)).unwrap();

    // --- Assemble file ---
    let file = FileSpec::builder_with("TaskApp.kt", Kotlin::new())
        .add_code(scope_trigger)
        .add_type(priority_spec)
        .add_type(repo_spec)
        .add_type(task_spec)
        .add_type(base_svc_spec)
        .add_type(task_svc_spec)
        .add_function(fetch_tasks)
        .add_function(create_task)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();
    print!("{output}");
}
