//! Example: Generate a Dart source file with sigil-stitch.
//!
//! Demonstrates:
//! - `import 'dart:...'` / `import 'package:...'` with grouped imports
//! - `abstract class` as interface with method signatures
//! - Concrete `class` with `extends` and `implements`
//! - `final` (readonly) and mutable fields with initializers
//! - `enum` with cases
//! - Generic class with `<T extends Bound>`
//! - `@override` annotations via `annotation()`
//! - `static final` constants with `%S` string literals
//! - `///` dartdoc comments
//! - Type-before-name declarations (`String name`, `User? findById(...)`)
//!
//! Run: `cargo run --example dart_codegen`

use sigil_stitch::code_block::{CodeBlock, StringLitArg};
use sigil_stitch::lang::dart::DartLang;
use sigil_stitch::spec::field_spec::FieldSpec;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::{FunSpec, TypeParamSpec};
use sigil_stitch::spec::modifiers::TypeKind;
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::spec::type_spec::TypeSpec;
use sigil_stitch::type_name::TypeName;

fn main() {
    // --- Imports (triggered by usage in code) ---
    let future = TypeName::importable("dart:async", "Future");
    let convert = TypeName::importable("dart:convert", "jsonDecode");
    let http_client = TypeName::importable("package:http/http.dart", "Client");

    // --- Enum: Priority ---
    let priority = TypeSpec::builder("Priority", TypeKind::Enum);
    let priority = priority.doc("Task priority levels.");

    let mut cases = CodeBlock::builder();
    cases.add("low,", ());
    cases.add_line();
    cases.add("medium,", ());
    cases.add_line();
    cases.add("high,", ());
    cases.add_line();
    cases.add("critical", ());
    cases.add_line();
    let priority = priority.extra_member(cases.build().unwrap());

    let priority_spec = priority.build().unwrap();

    // --- Abstract class (interface): TaskRepository ---
    let tp = TypeParamSpec::new("T");

    let repo_spec = TypeSpec::builder("TaskRepository", TypeKind::Interface)
        .add_type_param(tp)
        .doc("Repository for task persistence.")
        .add_method(
            FunSpec::builder("findById")
                .returns(TypeName::primitive("T?"))
                .add_param(ParameterSpec::new("id", TypeName::primitive("String")).unwrap())
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
                .add_param(ParameterSpec::new("entity", TypeName::primitive("T")).unwrap())
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    // --- Class: Task ---
    let task_cls = TypeSpec::builder("Task", TypeKind::Class);
    let task_cls = task_cls.doc("A task entity.");

    let id_field = FieldSpec::builder("id", TypeName::primitive("String"));
    let task_cls = task_cls.add_field(id_field.build().unwrap());

    let name_field = FieldSpec::builder("name", TypeName::primitive("String"));
    let task_cls = task_cls.add_field(name_field.build().unwrap());

    let task_cls = task_cls.add_field(
        FieldSpec::builder("priority", TypeName::primitive("Priority"))
            .is_readonly()
            .build()
            .unwrap(),
    );

    let task_cls = task_cls.add_field(
        FieldSpec::builder("completed", TypeName::primitive("bool"))
            .initializer(CodeBlock::of("false", ()).unwrap())
            .build()
            .unwrap(),
    );

    // Constructor.
    let ctor_body = CodeBlock::of(
        "this.id = id;\nthis.name = name;\nthis.priority = priority;",
        (),
    )
    .unwrap();
    let task_cls = task_cls.add_method(
        FunSpec::builder("Task")
            .add_param(ParameterSpec::new("id", TypeName::primitive("String")).unwrap())
            .add_param(ParameterSpec::new("name", TypeName::primitive("String")).unwrap())
            .add_param(ParameterSpec::new("priority", TypeName::primitive("Priority")).unwrap())
            .body(ctor_body)
            .build()
            .unwrap(),
    );

    let task_spec = task_cls.build().unwrap();

    // --- Class: Constants with static final ---
    let constants_spec = TypeSpec::builder("Constants", TypeKind::Class)
        .doc("Application constants.")
        .add_field(
            FieldSpec::builder("maxRetries", TypeName::primitive("int"))
                .is_static()
                .is_readonly()
                .initializer(CodeBlock::of("3", ()).unwrap())
                .build()
                .unwrap(),
        )
        .add_field(
            FieldSpec::builder("apiUrl", TypeName::primitive("String"))
                .is_static()
                .is_readonly()
                .initializer(
                    CodeBlock::of("%S", (StringLitArg("https://api.example.com".to_string()),))
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    // --- Class: InMemoryTaskRepository extends nothing, implements TaskRepository ---
    let impl_cls = TypeSpec::builder("InMemoryTaskRepository", TypeKind::Class);
    let impl_cls = impl_cls.implements(TypeName::primitive("TaskRepository<Task>"));
    let impl_cls = impl_cls.doc("In-memory implementation of TaskRepository.");

    let impl_cls = impl_cls.add_field(
        FieldSpec::builder("_tasks", TypeName::primitive("List<Task>"))
            .is_readonly()
            .initializer(CodeBlock::of("[]", ()).unwrap())
            .build()
            .unwrap(),
    );

    // findById with @override.
    let find_body = CodeBlock::of(
        "return _tasks.cast<Task?>().firstWhere(\n  (t) => t?.id == id,\n  orElse: () => null,\n);",
        (),
    )
    .unwrap();
    let impl_cls = impl_cls.add_method(
        FunSpec::builder("findById")
            .returns(TypeName::primitive("Task?"))
            .add_param(ParameterSpec::new("id", TypeName::primitive("String")).unwrap())
            .annotation(CodeBlock::of("@override", ()).unwrap())
            .body(find_body)
            .build()
            .unwrap(),
    );

    // findAll with @override.
    let find_all_body = CodeBlock::of("return List.unmodifiable(_tasks);", ()).unwrap();
    let impl_cls = impl_cls.add_method(
        FunSpec::builder("findAll")
            .returns(TypeName::primitive("List<Task>"))
            .annotation(CodeBlock::of("@override", ()).unwrap())
            .body(find_all_body)
            .build()
            .unwrap(),
    );

    // save with @override.
    let save_body = CodeBlock::of("_tasks.add(entity);", ()).unwrap();
    let impl_cls = impl_cls.add_method(
        FunSpec::builder("save")
            .add_param(ParameterSpec::new("entity", TypeName::primitive("Task")).unwrap())
            .annotation(CodeBlock::of("@override", ()).unwrap())
            .body(save_body)
            .build()
            .unwrap(),
    );

    let impl_spec = impl_cls.build().unwrap();

    // --- Generic class: SortedList<T extends Comparable> ---
    let sorted_tp = TypeParamSpec::new("T").with_bound(TypeName::primitive("Comparable"));

    let sorted = TypeSpec::builder("SortedList", TypeKind::Class);
    let sorted = sorted.add_type_param(sorted_tp);
    let sorted = sorted.doc("A sorted list backed by a type-bounded generic.");

    let sorted = sorted.add_field(
        FieldSpec::builder("_items", TypeName::primitive("List<T>"))
            .is_readonly()
            .initializer(CodeBlock::of("[]", ()).unwrap())
            .build()
            .unwrap(),
    );

    let add_body = CodeBlock::of("_items.add(item);\n_items.sort();", ()).unwrap();
    let sorted = sorted.add_method(
        FunSpec::builder("add")
            .returns(TypeName::primitive("void"))
            .add_param(ParameterSpec::new("item", TypeName::primitive("T")).unwrap())
            .body(add_body)
            .build()
            .unwrap(),
    );

    let get_body = CodeBlock::of("return List.unmodifiable(_items);", ()).unwrap();
    let sorted = sorted.add_method(
        FunSpec::builder("items")
            .returns(TypeName::primitive("List<T>"))
            .body(get_body)
            .build()
            .unwrap(),
    );

    let sorted_spec = sorted.build().unwrap();

    // --- Standalone function using imports ---
    let parse_body = CodeBlock::of(
        "final data = %T(json);\nreturn Task.fromMap(data);",
        (convert,),
    )
    .unwrap();
    let parse_task = FunSpec::builder("parseTask")
        .returns(TypeName::primitive("Task"))
        .add_param(ParameterSpec::new("json", TypeName::primitive("String")).unwrap())
        .body(parse_body)
        .build()
        .unwrap();

    // Trigger Future + http imports.
    let future_trigger = CodeBlock::of("// %T", (future,)).unwrap();
    let http_trigger = CodeBlock::of("// %T", (http_client,)).unwrap();

    // --- Assemble file ---
    let file = FileSpec::builder_with("task_app.dart", DartLang::new())
        .add_code(future_trigger)
        .add_code(http_trigger)
        .add_type(priority_spec)
        .add_type(repo_spec)
        .add_type(task_spec)
        .add_type(constants_spec)
        .add_type(impl_spec)
        .add_type(sorted_spec)
        .add_function(parse_task)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();
    print!("{output}");
}
