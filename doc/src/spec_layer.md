# The Spec Layer

Specs are structural builders that produce `Vec<CodeBlock<L>>`. They encapsulate common declaration patterns -- classes, functions, fields, enums -- so you work with named concepts instead of raw format strings. Every spec takes a `&L` language reference at emit time, which means the same builder definition renders correctly for any target language.

All spec types live in `src/spec/`. They follow a consistent builder pattern:

- **`&mut Self` for setters** -- chainable configuration methods
- **`self` for `.build()`** -- consumes the builder and returns `Result<Spec, SigilStitchError>`
- **Never chain `.build()` after setters** -- use a `let mut` binding instead

```rust,ignore
// Correct:
let mut fb = FunSpec::<TypeScript>::builder("greet");
fb.returns(TypeName::primitive("string"));
fb.body(body);
let fun = fb.build().unwrap();

// Wrong -- .build() consumes self, so you can't chain it after &mut Self setters
```

Every spec type (including `CodeBlock`, `TypeName`, `FileSpec`, and `ProjectSpec`) derives `serde::Serialize` and `serde::Deserialize`, so you can round-trip specs through JSON, YAML, or any other serde format. This is useful for caching materialized specs, shipping them across process boundaries, or diffing them in tests.

## ParameterSpec

A single function parameter: name, type, optional default value, and variadic flag.

```rust,ignore
use sigil_stitch::prelude::*;
use sigil_stitch::lang::typescript::TypeScript;

// Simple parameter
let p = ParameterSpec::<TypeScript>::new("name", TypeName::primitive("string")).unwrap();

// Parameter with default value
let mut pb = ParameterSpec::builder("count", TypeName::<TypeScript>::primitive("number"));
pb.default_value(CodeBlock::<TypeScript>::of("0", ()).unwrap());
let p = pb.build().unwrap();
// Output: count: number = 0

// Variadic parameter
let mut pb = ParameterSpec::builder("args", TypeName::<TypeScript>::primitive("string"));
pb.variadic();
let p = pb.build().unwrap();
// Output: ...args: string
```

`ParameterSpec` adapts to the target language. TypeScript emits `name: type`, C emits `type name`, and Python omits the type annotation when the type is empty.

## FieldSpec

A struct field or class property: name, type, visibility, static/readonly flags, initializer, annotations, and doc comments.

```rust,ignore
use sigil_stitch::prelude::*;
use sigil_stitch::lang::typescript::TypeScript;
use sigil_stitch::lang::rust_lang::RustLang;

let mut fb = FieldSpec::builder("name", TypeName::<TypeScript>::primitive("string"));
fb.visibility(Visibility::Private);
fb.is_readonly();
let field = fb.build().unwrap();
// TypeScript: private readonly name: string;

let mut fb = FieldSpec::builder("name", TypeName::<RustLang>::primitive("String"));
fb.visibility(Visibility::Public);
let field = fb.build().unwrap();
// Rust: pub name: String,
```

Fields support initializers for default values:

```rust,ignore
let mut fb = FieldSpec::builder("count", TypeName::<TypeScript>::primitive("number"));
fb.initializer(CodeBlock::<TypeScript>::of("0", ()).unwrap());
let field = fb.build().unwrap();
// TypeScript: count: number = 0;
```

For Go, use `.tag()` to attach struct tags:

```rust,ignore
let mut fb = FieldSpec::builder("Name", TypeName::<Go>::primitive("string"));
fb.tag("json:\"name\" db:\"name\"");
let field = fb.build().unwrap();
// Go: Name string `json:"name" db:"name"`
```

### Optional fields

`is_optional()` marks a field whose key may be absent (distinct from a value that
can be `null`). Rendering is language-specific, delegated to
`CodeLang::optional_field_style()`:

```rust,ignore
let mut fb = FieldSpec::builder("email", TypeName::<TypeScript>::primitive("string"));
fb.is_optional();
let field = fb.build().unwrap();
// TypeScript:  email?: string;
// JavaScript:  email;                (marker stripped — no optionality in JS)
// Rust:        email: Option<String>,
// Go:          Email *string
// Python:      email: str | None
// Java:        Optional<String> email;   (caller must import java.util.Optional)
// Kotlin:      name: String?
// Swift:       name: String?
// Dart:        String? name;
// C:           string *email;
// C++:         std::optional<string> email;   (caller must #include <optional>)
```

Use `is_optional()` for "the key might not be there" (e.g., an OpenAPI property
not listed in `required`). Use `TypeName::optional(...)` for "the value might be
null" at the type level.

## FunSpec

A function or method: parameters, return type, body, modifiers (async, static, abstract, constructor, override), type parameters, annotations, and doc comments.

```rust,ignore
use sigil_stitch::prelude::*;
use sigil_stitch::lang::typescript::TypeScript;

let body = CodeBlock::<TypeScript>::of("return this.name", ()).unwrap();

let mut fb = FunSpec::<TypeScript>::builder("getName");
fb.returns(TypeName::primitive("string"));
fb.body(body);
let fun = fb.build().unwrap();
// function getName(): string {
//     return this.name
// }
```

### Async methods

```rust,ignore
let mut fb = FunSpec::<TypeScript>::builder("fetchUser");
fb.is_async();
fb.visibility(Visibility::Public);
fb.add_param(ParameterSpec::new("id", TypeName::primitive("string")).unwrap());
fb.returns(TypeName::generic(
    TypeName::primitive("Promise"),
    vec![TypeName::primitive("User")],
));
let body = CodeBlock::<TypeScript>::of("return await db.find(id)", ()).unwrap();
fb.body(body);
let fun = fb.build().unwrap();
// public async fetchUser(id: string): Promise<User> {
//     return await db.find(id)
// }
```

### Type parameters

```rust,ignore
let tp = TypeParamSpec::<TypeScript>::new("T")
    .with_bound(TypeName::primitive("Serializable"));

let mut fb = FunSpec::<TypeScript>::builder("serialize");
fb.add_type_param(tp);
fb.add_param(ParameterSpec::new("value", TypeName::primitive("T")).unwrap());
fb.returns(TypeName::primitive("string"));
let body = CodeBlock::<TypeScript>::of("return JSON.stringify(value)", ()).unwrap();
fb.body(body);
let fun = fb.build().unwrap();
// function serialize<T extends Serializable>(value: T): string {
//     return JSON.stringify(value)
// }
```

### Abstract methods

When no body is provided, the function renders as a declaration. Combined with `is_abstract()`, this produces abstract method signatures:

```rust,ignore
let mut fb = FunSpec::<TypeScript>::builder("validate");
fb.is_abstract();
fb.returns(TypeName::primitive("boolean"));
let fun = fb.build().unwrap();
// abstract validate(): boolean;
```

### Constructor delegation

Use `.delegation()` to emit `super(...)` or `this(...)` calls. The placement is language-dependent: body-style (TS, Java, Dart, Swift) emits it as the first statement; signature-style (Kotlin) emits it after the parameter list.

```rust,ignore
let mut fb = FunSpec::<TypeScript>::builder("constructor");
fb.is_constructor();
fb.add_param(ParameterSpec::new("name", TypeName::primitive("string")).unwrap());
fb.delegation(CodeBlock::<TypeScript>::of("super(name)", ()).unwrap());
let body = CodeBlock::<TypeScript>::of("this.name = name", ()).unwrap();
fb.body(body);
let fun = fb.build().unwrap();
// constructor(name: string) {
//     super(name);
//     this.name = name
// }
```

## TypeSpec

The largest spec. Models type declarations: struct, class, interface, trait, or enum. Takes a `TypeKind` to select the declaration form.

`.build()` returns `Err(SigilStitchError::DuplicateFieldName { type_name, field_name })` when two fields in the same type share a name.

### Single-block output (TypeScript class)

When `lang.methods_inside_type_body(kind)` returns `true`, TypeSpec emits a single CodeBlock with fields and methods inside the body:

```rust,ignore
use sigil_stitch::prelude::*;
use sigil_stitch::lang::typescript::TypeScript;

let mut tb = TypeSpec::<TypeScript>::builder("UserService", TypeKind::Class);
tb.visibility(Visibility::Public);

let mut field_b = FieldSpec::builder("name", TypeName::primitive("string"));
field_b.visibility(Visibility::Private);
tb.add_field(field_b.build().unwrap());

let body = CodeBlock::<TypeScript>::of("return this.name", ()).unwrap();
let mut fb = FunSpec::builder("getName");
fb.returns(TypeName::primitive("string"));
fb.body(body);
tb.add_method(fb.build().unwrap());

let type_spec = tb.build().unwrap();
let blocks = type_spec.emit(&TypeScript::new()).unwrap();
// blocks.len() == 1
//
// export class UserService {
//     private name: string;
//
//     getName(): string {
//         return this.name
//     }
// }
```

### Two-block output (Rust struct + impl)

When `methods_inside_type_body(kind)` returns `false` (Rust structs and enums), TypeSpec emits two separate CodeBlocks: one for the data definition, one for the `impl` block:

```rust,ignore
use sigil_stitch::prelude::*;
use sigil_stitch::lang::rust_lang::RustLang;

let mut tb = TypeSpec::<RustLang>::builder("Config", TypeKind::Struct);
tb.visibility(Visibility::Public);

let mut field_b = FieldSpec::builder("name", TypeName::primitive("String"));
field_b.visibility(Visibility::Public);
tb.add_field(field_b.build().unwrap());

let body = CodeBlock::<RustLang>::of("Self { name: name.to_string() }", ()).unwrap();
let mut fb = FunSpec::<RustLang>::builder("new");
fb.visibility(Visibility::Public);
fb.add_param(ParameterSpec::new("name", TypeName::primitive("&str")).unwrap());
fb.returns(TypeName::primitive("Self"));
fb.body(body);
tb.add_method(fb.build().unwrap());

let type_spec = tb.build().unwrap();
let blocks = type_spec.emit(&RustLang::new()).unwrap();
// blocks.len() == 2
//
// Block 0:
// pub struct Config {
//     pub name: String,
// }
//
// Block 1:
// impl Config {
//     pub fn new(name: &str) -> Self {
//         Self { name: name.to_string() }
//     }
// }
```

This split is the key structural decision. It is fully automatic -- you build one TypeSpec, and the language's `methods_inside_type_body()` determines whether the output is one block or two.

### Extends and implements

```rust,ignore
let mut tb = TypeSpec::<TypeScript>::builder("AdminService", TypeKind::Class);
tb.visibility(Visibility::Public);
tb.extends(TypeName::primitive("BaseService"));
tb.implements(TypeName::primitive("Serializable"));
let type_spec = tb.build().unwrap();
// export class AdminService extends BaseService implements Serializable {
// }
```

### Enums with EnumVariantSpec

TypeSpec with `TypeKind::Enum` uses `add_variant()` instead of `add_field()`. See the [EnumVariantSpec](#enumvariantspec) section below for variant forms.

```rust,ignore
use sigil_stitch::prelude::*;
use sigil_stitch::spec::enum_variant_spec::EnumVariantSpec;
use sigil_stitch::lang::typescript::TypeScript;

let mut tb = TypeSpec::<TypeScript>::builder("Direction", TypeKind::Enum);
let mut v = EnumVariantSpec::builder("Up");
v.value(CodeBlock::<TypeScript>::of("'UP'", ()).unwrap());
tb.add_variant(v.build().unwrap());
let mut v = EnumVariantSpec::builder("Down");
v.value(CodeBlock::<TypeScript>::of("'DOWN'", ()).unwrap());
tb.add_variant(v.build().unwrap());
let type_spec = tb.build().unwrap();
// enum Direction {
//     Up = 'UP',
//     Down = 'DOWN',
// }
```

## PropertySpec

Computed properties with getter and/or setter. Rendering depends on `lang.property_style()`:

- **Accessor** (TypeScript, JavaScript): emits separate `get name(): T { ... }` and `set name(v: T) { ... }` methods
- **Field** (Swift, Kotlin): emits a field with inline `get`/`set` blocks

```rust,ignore
use sigil_stitch::prelude::*;
use sigil_stitch::spec::property_spec::PropertySpec;
use sigil_stitch::lang::typescript::TypeScript;

let getter_body = CodeBlock::<TypeScript>::of("return this._name", ()).unwrap();
let setter_body = CodeBlock::<TypeScript>::of("this._name = value", ()).unwrap();

let mut pb = PropertySpec::builder("name", TypeName::<TypeScript>::primitive("string"));
pb.getter(getter_body);
pb.setter("value", setter_body);
let prop = pb.build().unwrap();
// TypeScript (Accessor style):
// get name(): string {
//     return this._name
// }
// set name(value: string) {
//     this._name = value
// }
```

For Swift and Kotlin, the same PropertySpec renders as a field with inline body blocks instead.

## AnnotationSpec

Structured annotations that render with language-appropriate syntax. The prefix and suffix adapt automatically:

| Language       | Syntax                          |
|----------------|---------------------------------|
| Java, Kotlin, TS | `@Name(args)`                |
| Rust           | `#[name(args)]`                 |
| C++            | `[[name(args)]]`                |
| C              | `__attribute__((name(args)))`   |

```rust,ignore
use sigil_stitch::spec::annotation_spec::AnnotationSpec;
use sigil_stitch::lang::rust_lang::RustLang;

// Simple annotation: #[allow(dead_code)]
let ann = AnnotationSpec::<RustLang>::new("allow").arg("dead_code");

// Multiple arguments: #[cfg(test, feature = "nightly")]
let ann = AnnotationSpec::<RustLang>::new("cfg")
    .arg("test")
    .arg("feature = \"nightly\"");
```

For import-tracked annotations, use `importable()` with a `TypeName`:

```rust,ignore
use sigil_stitch::spec::annotation_spec::AnnotationSpec;
use sigil_stitch::lang::typescript::TypeScript;
use sigil_stitch::type_name::TypeName;

let type_name = TypeName::<TypeScript>::importable("./decorators", "Component");
let ann = AnnotationSpec::importable(type_name);
// TS: @Component (with import { Component } from './decorators')
```

If `AnnotationSpec` does not cover your annotation format, every builder also has an `.annotation(CodeBlock)` escape hatch that accepts a raw CodeBlock.

## EnumVariantSpec

Individual enum variants. Four forms are supported:

### Simple variant

```rust,ignore
use sigil_stitch::spec::enum_variant_spec::EnumVariantSpec;
use sigil_stitch::lang::rust_lang::RustLang;

let v = EnumVariantSpec::<RustLang>::new("Red").unwrap();
// Rust: Red,
```

### Valued variant

```rust,ignore
use sigil_stitch::spec::enum_variant_spec::EnumVariantSpec;
use sigil_stitch::lang::typescript::TypeScript;

let mut v = EnumVariantSpec::<TypeScript>::builder("Up");
v.value(CodeBlock::<TypeScript>::of("'UP'", ()).unwrap());
let variant = v.build().unwrap();
// TypeScript: Up = 'UP',
```

### Tuple variant (Rust, Swift)

```rust,ignore
use sigil_stitch::spec::enum_variant_spec::EnumVariantSpec;
use sigil_stitch::lang::rust_lang::RustLang;

let mut v = EnumVariantSpec::<RustLang>::builder("Literal");
v.associated_type(TypeName::primitive("i64"));
let variant = v.build().unwrap();
// Rust: Literal(i64),

// Multi-element tuple
let mut v = EnumVariantSpec::<RustLang>::builder("Pair");
v.associated_type(TypeName::primitive("String"));
v.associated_type(TypeName::primitive("i32"));
let variant = v.build().unwrap();
// Rust: Pair(String, i32),
```

### Struct variant (Rust)

```rust,ignore
use sigil_stitch::spec::enum_variant_spec::EnumVariantSpec;
use sigil_stitch::spec::field_spec::FieldSpec;
use sigil_stitch::lang::rust_lang::RustLang;

let mut v = EnumVariantSpec::<RustLang>::builder("Move");
v.add_field(
    FieldSpec::builder("x", TypeName::primitive("i32")).build().unwrap(),
);
v.add_field(
    FieldSpec::builder("y", TypeName::primitive("i32")).build().unwrap(),
);
let variant = v.build().unwrap();
// Rust:
// Move {
//     x: i32,
//     y: i32,
// },
```

Variants are added to a TypeSpec via `add_variant()`. The language controls separators (`enum_variant_separator`), trailing separators (`enum_variant_trailing_separator`), and prefixes (Swift's `case`).

## ImportSpec

Explicit import control for cases where `%T` / `TypeName::Importable` is not sufficient. Add to a FileSpec via `add_import()`.

```rust,ignore
use sigil_stitch::spec::import_spec::ImportSpec;
use sigil_stitch::lang::typescript::TypeScript;

// Forced named import (even without %T usage in code)
let spec = ImportSpec::<TypeScript>::named("./models", "User");

// Aliased import: import { User as MyUser } from './models'
let spec = ImportSpec::<TypeScript>::named_as("./models", "User", "MyUser");

// Type-only import: import type { User } from './models'
let spec = ImportSpec::<TypeScript>::named_type("./models", "User");

// Side-effect import: import './polyfill'
let spec = ImportSpec::<TypeScript>::side_effect("./polyfill");

// Wildcard import: import * from './utils'
let spec = ImportSpec::<TypeScript>::wildcard("./utils");
```

Most of the time you do not need `ImportSpec` -- imports driven by `%T` and `TypeName::importable()` handle the common case. Use `ImportSpec` for forced imports, side-effect imports, and wildcard imports.

## FileSpec

The top-level file orchestrator. Combines code blocks, type declarations, and functions, then drives the three-pass render pipeline:

1. **Materialize** -- Specs (`TypeSpec`, `FunSpec`) emit CodeBlocks
2. **Collect imports** -- Walk all blocks, extract import references from `%T` types
3. **Render** -- Emit the import header, then the body with resolved names and pretty printing

```rust,ignore
use sigil_stitch::prelude::*;
use sigil_stitch::lang::typescript::TypeScript;

let user = TypeName::<TypeScript>::importable_type("./models", "User");

let mut cb = CodeBlock::<TypeScript>::builder();
cb.add_statement("const u: %T = getUser()", (user,));
let block = cb.build().unwrap();

let mut fb = FileSpec::<TypeScript>::builder("user.ts");
fb.add_code(block);
let file = fb.build().unwrap();

let output = file.render(80).unwrap();
// import type { User } from './models'
//
// const u: User = getUser();
```

You can mix member types freely: `add_code()` for raw CodeBlocks, `add_type()` for TypeSpec, `add_function()` for FunSpec, `add_raw()` for escape-hatch strings with no import tracking.

A file header (license comment, package declaration) can be set with `.header()`:

```rust,ignore
let mut header_b = CodeBlock::<TypeScript>::builder();
header_b.add("// License: MIT", ());
let header = header_b.build().unwrap();

let mut fb = FileSpec::<TypeScript>::builder("service.ts");
fb.header(header);
fb.add_type(service_type);
let file = fb.build().unwrap();
```

## ProjectSpec

Multi-file generation. Wraps multiple FileSpecs, renders them all, and can optionally write to the filesystem.

```rust,ignore
use sigil_stitch::prelude::*;
use sigil_stitch::lang::typescript::TypeScript;

// Build individual files
let mut models_b = FileSpec::<TypeScript>::builder("src/models.ts");
models_b.add_type(
    TypeSpec::builder("User", TypeKind::Interface).build().unwrap(),
);
let models = models_b.build().unwrap();

let mut index_b = FileSpec::<TypeScript>::builder("src/index.ts");
index_b.add_code(CodeBlock::of("export {}", ()).unwrap());
let index = index_b.build().unwrap();

// Combine into a project
let mut pb = ProjectSpec::<TypeScript>::builder();
pb.add_file(models);
pb.add_file(index);
let project = pb.build();

// Render all files in memory
let rendered = project.render(80).unwrap();
for file in &rendered {
    println!("--- {} ---\n{}", file.path, file.content);
}

// Or write directly to disk
// project.write_to(Path::new("./output"), 80).unwrap();
```

Each file resolves imports independently. `render()` returns `Vec<RenderedFile>` with `path` and `content` fields. `write_to()` creates parent directories as needed.

## End-to-End Example

A complete TypeScript class with imports, fields, a constructor, and a method -- from builder calls to rendered output.

```rust,ignore
use sigil_stitch::prelude::*;
use sigil_stitch::lang::typescript::TypeScript;

// Define an imported type
let repo_type = TypeName::<TypeScript>::importable_type("./repository", "UserRepository");

// Build the class
let mut tb = TypeSpec::<TypeScript>::builder("UserService", TypeKind::Class);
tb.visibility(Visibility::Public);

// Field: private readonly repo: UserRepository;
let mut field_b = FieldSpec::builder("repo", repo_type.clone());
field_b.visibility(Visibility::Private);
field_b.is_readonly();
tb.add_field(field_b.build().unwrap());

// Constructor
let mut ctor = FunSpec::<TypeScript>::builder("constructor");
ctor.is_constructor();
ctor.add_param(ParameterSpec::new("repo", repo_type.clone()).unwrap());
let ctor_body = CodeBlock::<TypeScript>::of("this.repo = repo", ()).unwrap();
ctor.body(ctor_body);
tb.add_method(ctor.build().unwrap());

// Method: async getUser(id: string): Promise<User>
let user_type = TypeName::<TypeScript>::importable_type("./models", "User");
let mut method = FunSpec::<TypeScript>::builder("getUser");
method.is_async();
method.add_param(ParameterSpec::new("id", TypeName::primitive("string")).unwrap());
method.returns(TypeName::generic(
    TypeName::primitive("Promise"),
    vec![user_type],
));
let method_body = CodeBlock::<TypeScript>::of("return this.repo.findById(id)", ()).unwrap();
method.body(method_body);
tb.add_method(method.build().unwrap());

let type_spec = tb.build().unwrap();

// Build the file
let mut fb = FileSpec::<TypeScript>::builder("user_service.ts");
fb.add_type(type_spec);
let file = fb.build().unwrap();

let output = file.render(80).unwrap();
```

Rendered output:

```typescript
import type { User } from './models'
import { UserRepository } from './repository'

export class UserService {
    private readonly repo: UserRepository;

    constructor(repo: UserRepository) {
        this.repo = repo
    }

    async getUser(id: string): Promise<User> {
        return this.repo.findById(id)
    }
}
```

The import header is fully automatic. `UserRepository` and `User` are collected from the `%T` references inside the emitted CodeBlocks, deduplicated, and rendered as import statements. No manual import management required.
