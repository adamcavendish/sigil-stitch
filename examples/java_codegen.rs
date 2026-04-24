//! Example: Generate a Java source file with sigil-stitch.
//!
//! Demonstrates:
//! - `import` statements with java/javax/third-party grouping
//! - Generic interface with bounded type parameter
//! - Abstract class with Javadoc
//! - Concrete class extending + implementing
//! - Enum with constants
//! - `@Override` and `@Nullable` annotations
//! - `static final` constants
//! - Constructors (no return type)
//!
//! Run: `cargo run --example java_codegen`

use sigil_stitch::code_block::{CodeBlock, StringLitArg};
use sigil_stitch::lang::java_lang::JavaLang;
use sigil_stitch::spec::field_spec::FieldSpec;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::{FunSpec, TypeParamSpec};
use sigil_stitch::spec::modifiers::{TypeKind, Visibility};
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::spec::type_spec::TypeSpec;
use sigil_stitch::type_name::TypeName;

fn main() {
    // --- Imports (triggered by usage in code) ---
    let list = TypeName::importable("java.util", "List");
    let array_list = TypeName::importable("java.util", "ArrayList");
    let optional = TypeName::importable("java.util", "Optional");
    let nullable = TypeName::importable("javax.annotation", "Nullable");
    let logger = TypeName::importable("org.slf4j", "Logger");
    let logger_factory = TypeName::importable("org.slf4j", "LoggerFactory");

    // --- Enum: Priority ---
    let priority = TypeSpec::builder("Priority", TypeKind::Enum);
    let priority = priority.visibility(Visibility::Public);
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

    // --- Interface: TaskRepository<T> ---
    let tp = TypeParamSpec::new("T").with_bound(TypeName::primitive("Serializable"));

    let repo_spec = TypeSpec::builder("TaskRepository", TypeKind::Interface)
        .visibility(Visibility::Public)
        .add_type_param(tp)
        .doc("Repository for task persistence.")
        .doc("")
        .doc("@param <T> the task entity type")
        .add_method(
            FunSpec::builder("findById")
                .returns(TypeName::primitive("T"))
                .add_param(ParameterSpec::new("id", TypeName::primitive("long")).unwrap())
                .annotation(CodeBlock::of("@%T", (nullable.clone(),)).unwrap())
                .build()
                .unwrap(),
        )
        .add_method(
            FunSpec::builder("findAll")
                .returns(TypeName::primitive("List<T>"))
                .build()
                .unwrap(),
        )
        .add_method(
            FunSpec::builder("save")
                .returns(TypeName::primitive("void"))
                .add_param(ParameterSpec::new("entity", TypeName::primitive("T")).unwrap())
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    // --- Abstract class: BaseTask ---
    let base_task = TypeSpec::builder("BaseTask", TypeKind::Class);
    let base_task = base_task.visibility(Visibility::Public);
    let base_task = base_task.is_abstract();
    let base_task = base_task.doc("Base class for all tasks.");

    let base_task = base_task.add_field(
        FieldSpec::builder("id", TypeName::primitive("long"))
            .visibility(Visibility::Private)
            .is_readonly()
            .build()
            .unwrap(),
    );

    let base_task = base_task.add_field(
        FieldSpec::builder("name", TypeName::primitive("String"))
            .visibility(Visibility::Private)
            .build()
            .unwrap(),
    );

    let base_task = base_task.add_field(
        FieldSpec::builder("priority", TypeName::primitive("Priority"))
            .visibility(Visibility::Private)
            .build()
            .unwrap(),
    );

    // Constructor
    let ctor_body = CodeBlock::of(
        "this.id = id;\nthis.name = name;\nthis.priority = priority;",
        (),
    )
    .unwrap();
    let base_task = base_task.add_method(
        FunSpec::builder("BaseTask")
            .visibility(Visibility::Protected)
            .add_param(ParameterSpec::new("id", TypeName::primitive("long")).unwrap())
            .add_param(ParameterSpec::new("name", TypeName::primitive("String")).unwrap())
            .add_param(ParameterSpec::new("priority", TypeName::primitive("Priority")).unwrap())
            .body(ctor_body)
            .build()
            .unwrap(),
    );

    // Concrete getter
    let get_id_body = CodeBlock::of("return this.id;", ()).unwrap();
    let base_task = base_task.add_method(
        FunSpec::builder("getId")
            .visibility(Visibility::Public)
            .returns(TypeName::primitive("long"))
            .body(get_id_body)
            .build()
            .unwrap(),
    );

    // Abstract method
    let base_task = base_task.add_method(
        FunSpec::builder("execute")
            .visibility(Visibility::Public)
            .is_abstract()
            .returns(TypeName::primitive("void"))
            .build()
            .unwrap(),
    );

    let base_task_spec = base_task.build().unwrap();

    // --- Concrete class: SimpleTask extends BaseTask implements Serializable ---
    let simple_task = TypeSpec::builder("SimpleTask", TypeKind::Class);
    let simple_task = simple_task.visibility(Visibility::Public);
    let simple_task = simple_task.extends(TypeName::primitive("BaseTask"));
    let simple_task = simple_task.implements(TypeName::primitive("Serializable"));
    let simple_task = simple_task.doc("A simple executable task.");

    // Logger constant — uses imports
    let logger_init = CodeBlock::of("%T.getLogger(SimpleTask.class)", (logger_factory,)).unwrap();
    let simple_task = simple_task.add_field(
        FieldSpec::builder("LOG", TypeName::primitive("Logger"))
            .visibility(Visibility::Private)
            .is_static()
            .is_readonly()
            .initializer(logger_init)
            .build()
            .unwrap(),
    );

    // Trigger Logger import
    let logger_trigger = CodeBlock::of("// Logger type: %T", (logger,)).unwrap();

    // Constructor
    let simple_ctor_body = CodeBlock::of("super(id, name, Priority.MEDIUM);", ()).unwrap();
    let simple_task = simple_task.add_method(
        FunSpec::builder("SimpleTask")
            .visibility(Visibility::Public)
            .add_param(ParameterSpec::new("id", TypeName::primitive("long")).unwrap())
            .add_param(ParameterSpec::new("name", TypeName::primitive("String")).unwrap())
            .body(simple_ctor_body)
            .build()
            .unwrap(),
    );

    // execute() override
    let exec_body = CodeBlock::of(
        "LOG.info(%S + this.getId());",
        (StringLitArg("Executing task: ".to_string()),),
    )
    .unwrap();
    let simple_task = simple_task.add_method(
        FunSpec::builder("execute")
            .visibility(Visibility::Public)
            .returns(TypeName::primitive("void"))
            .annotation(CodeBlock::of("@Override", ()).unwrap())
            .body(exec_body)
            .build()
            .unwrap(),
    );

    let simple_task_spec = simple_task.build().unwrap();

    // --- Standalone utility function (wrapped in class body by user) ---
    let create_body = CodeBlock::of(
        "%T<SimpleTask> tasks = new %T<>();\ntasks.add(new SimpleTask(1, %S));\nreturn tasks;",
        (list, array_list, StringLitArg("Default Task".to_string())),
    )
    .unwrap();
    let create_tasks = FunSpec::builder("createDefaultTasks")
        .visibility(Visibility::Public)
        .is_static()
        .returns(TypeName::primitive("List<SimpleTask>"))
        .body(create_body)
        .build()
        .unwrap();

    // findById using Optional — trigger import
    let find_body = CodeBlock::of(
        "return tasks.stream()\n    .filter(t -> t.getId() == id)\n    .findFirst();",
        (),
    )
    .unwrap();
    let find_fn = FunSpec::builder("findTaskById")
        .visibility(Visibility::Public)
        .is_static()
        .returns(TypeName::primitive("Optional<SimpleTask>"))
        .add_param(ParameterSpec::new("tasks", TypeName::primitive("List<SimpleTask>")).unwrap())
        .add_param(ParameterSpec::new("id", TypeName::primitive("long")).unwrap())
        .body(find_body);
    let find_task = find_fn.build().unwrap();

    // Trigger Optional import
    let optional_trigger = CodeBlock::of("// Optional: %T", (optional,)).unwrap();

    // --- Assemble file ---
    let file = FileSpec::builder_with("TaskApp.java", JavaLang::new())
        .add_code(logger_trigger)
        .add_code(optional_trigger)
        .add_type(priority_spec)
        .add_type(repo_spec)
        .add_type(base_task_spec)
        .add_type(simple_task_spec)
        .add_function(create_tasks)
        .add_function(find_task)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();
    print!("{output}");
}
