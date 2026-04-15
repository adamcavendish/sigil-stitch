use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::cpp_lang::CppLang;
use sigil_stitch::lang::dart::DartLang;
use sigil_stitch::lang::java_lang::JavaLang;
use sigil_stitch::lang::javascript::JavaScript;
use sigil_stitch::lang::kotlin::Kotlin;
use sigil_stitch::lang::python::Python;
use sigil_stitch::lang::rust_lang::RustLang;
use sigil_stitch::lang::swift::Swift;
use sigil_stitch::lang::typescript::TypeScript;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::modifiers::{DeclarationContext, TypeKind, Visibility};
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::spec::type_spec::TypeSpec;
use sigil_stitch::type_name::TypeName;

// ── Helpers ──────────────────────────────────────────────

fn render_fun<L: sigil_stitch::lang::CodeLang>(
    spec: &FunSpec<L>,
    lang: &L,
    ctx: DeclarationContext,
) -> String {
    let block = spec.emit(lang, ctx);
    let imports = sigil_stitch::import::ImportGroup::new();
    let mut renderer = sigil_stitch::code_renderer::CodeRenderer::new(lang, &imports, 80);
    renderer.render(&block)
}

fn render_type<L: sigil_stitch::lang::CodeLang>(spec: &TypeSpec<L>, lang: &L) -> String {
    let blocks = spec.emit(lang);
    let imports = sigil_stitch::import::ImportGroup::new();
    let mut output = String::new();
    for (i, block) in blocks.iter().enumerate() {
        if i > 0 {
            output.push('\n');
        }
        let mut renderer = sigil_stitch::code_renderer::CodeRenderer::new(lang, &imports, 80);
        output.push_str(&renderer.render(&block));
    }
    output
}

// ── TypeScript ───────────────────────────────────────────

#[test]
fn test_ts_constructor() {
    let ts = TypeScript::new();
    let mut fb = FunSpec::<TypeScript>::builder("constructor");
    fb.is_constructor();
    fb.add_param(ParameterSpec::new("name", TypeName::primitive("string")));
    fb.body(CodeBlock::of("this.name = name", ()).unwrap());
    let output = render_fun(&fb.build(), &ts, DeclarationContext::Member);
    assert!(output.contains("constructor(name: string) {"));
    assert!(output.contains("this.name = name"));
    // Must NOT have "function" keyword.
    assert!(!output.contains("function"));
}

#[test]
fn test_ts_constructor_in_class() {
    let ts = TypeScript::new();
    let mut tb = TypeSpec::<TypeScript>::builder("User", TypeKind::Class);
    tb.visibility(Visibility::Public);
    let mut ctor = FunSpec::builder("constructor");
    ctor.is_constructor();
    ctor.add_param(ParameterSpec::new("name", TypeName::primitive("string")));
    ctor.body(CodeBlock::of("this.name = name", ()).unwrap());
    tb.add_method(ctor.build());
    let output = render_type(&tb.build(), &ts);
    assert!(output.contains("export class User {"));
    assert!(output.contains("constructor(name: string) {"));
}

// ── JavaScript ───────────────────────────────────────────

#[test]
fn test_js_constructor() {
    let js = JavaScript::new();
    let mut fb = FunSpec::<JavaScript>::builder("constructor");
    fb.is_constructor();
    fb.add_param(ParameterSpec::new("name", TypeName::primitive("")));
    fb.body(CodeBlock::of("this.name = name", ()).unwrap());
    let output = render_fun(&fb.build(), &js, DeclarationContext::Member);
    assert!(output.contains("constructor(name) {"));
    assert!(!output.contains("function"));
}

// ── Java ─────────────────────────────────────────────────

#[test]
fn test_java_constructor() {
    let java = JavaLang::new();
    let mut fb = FunSpec::<JavaLang>::builder("UserService");
    fb.is_constructor();
    fb.visibility(Visibility::Public);
    fb.add_param(ParameterSpec::new("repo", TypeName::primitive("UserRepository")));
    fb.body(CodeBlock::of("this.repo = repo;", ()).unwrap());
    let output = render_fun(&fb.build(), &java, DeclarationContext::Member);
    assert!(output.contains("public UserService(UserRepository repo) {"));
    // No return type.
    assert!(!output.contains("void"));
}

// ── C++ ──────────────────────────────────────────────────

#[test]
fn test_cpp_constructor() {
    let cpp = CppLang::new();
    let mut fb = FunSpec::<CppLang>::builder("Counter");
    fb.is_constructor();
    fb.body(CodeBlock::of("count_ = 0;", ()).unwrap());
    let output = render_fun(&fb.build(), &cpp, DeclarationContext::Member);
    assert!(output.contains("Counter() {"));
    // No return type prefix.
    assert!(!output.contains("void"));
}

// ── Dart ─────────────────────────────────────────────────

#[test]
fn test_dart_constructor() {
    let dart = DartLang::new();
    let mut fb = FunSpec::<DartLang>::builder("Task");
    fb.is_constructor();
    fb.add_param(ParameterSpec::new("title", TypeName::primitive("String")));
    fb.body(CodeBlock::of("this.title = title;", ()).unwrap());
    let output = render_fun(&fb.build(), &dart, DeclarationContext::Member);
    assert!(output.contains("Task(String title) {"));
}

// ── Swift ────────────────────────────────────────────────

#[test]
fn test_swift_constructor() {
    let swift = Swift::new();
    let mut fb = FunSpec::<Swift>::builder("init");
    fb.is_constructor();
    fb.add_param(ParameterSpec::new("name", TypeName::primitive("String")));
    fb.body(CodeBlock::of("self.name = name", ()).unwrap());
    let output = render_fun(&fb.build(), &swift, DeclarationContext::Member);
    // Must be `init(name: String)` NOT `func init(name: String)`.
    assert!(output.contains("init(name: String) {"));
    assert!(!output.contains("func init"));
}

#[test]
fn test_swift_constructor_in_class() {
    let swift = Swift::new();
    let mut tb = TypeSpec::<Swift>::builder("Person", TypeKind::Class);
    let mut ctor = FunSpec::builder("init");
    ctor.is_constructor();
    ctor.add_param(ParameterSpec::new("name", TypeName::primitive("String")));
    ctor.body(CodeBlock::of("self.name = name", ()).unwrap());
    tb.add_method(ctor.build());
    let output = render_type(&tb.build(), &swift);
    assert!(output.contains("class Person {"));
    assert!(output.contains("init(name: String) {"));
    assert!(!output.contains("func init"));
}

// ── Kotlin ───────────────────────────────────────────────

#[test]
fn test_kotlin_secondary_constructor() {
    let kt = Kotlin::new();
    let mut fb = FunSpec::<Kotlin>::builder("constructor");
    fb.is_constructor();
    fb.add_param(ParameterSpec::new("name", TypeName::primitive("String")));
    fb.body(CodeBlock::of("this.name = name", ()).unwrap());
    let output = render_fun(&fb.build(), &kt, DeclarationContext::Member);
    // Must be `constructor(name: String)` NOT `fun constructor(name: String)`.
    assert!(output.contains("constructor(name: String)"));
    assert!(!output.contains("fun constructor"));
}

#[test]
fn test_kotlin_constructor_in_class() {
    let kt = Kotlin::new();
    let mut tb = TypeSpec::<Kotlin>::builder("Person", TypeKind::Class);
    let mut ctor = FunSpec::builder("constructor");
    ctor.is_constructor();
    ctor.add_param(ParameterSpec::new("name", TypeName::primitive("String")));
    ctor.body(CodeBlock::of("this.name = name", ()).unwrap());
    tb.add_method(ctor.build());
    let output = render_type(&tb.build(), &kt);
    assert!(output.contains("class Person {"));
    assert!(output.contains("constructor(name: String)"));
    assert!(!output.contains("fun constructor"));
}

// ── Python ───────────────────────────────────────────────

#[test]
fn test_python_constructor() {
    let py = Python::new();
    let mut fb = FunSpec::<Python>::builder("__init__");
    fb.is_constructor();
    fb.add_param(ParameterSpec::new("self", TypeName::primitive("")));
    fb.add_param(ParameterSpec::new("name", TypeName::primitive("str")));
    fb.body(CodeBlock::of("self.name = name", ()).unwrap());
    let output = render_fun(&fb.build(), &py, DeclarationContext::Member);
    // Must keep "def" keyword: `def __init__(self, name: str):`
    assert!(output.contains("def __init__(self, name: str):"));
}

// ── Rust ─────────────────────────────────────────────────

#[test]
fn test_rust_constructor() {
    let rs = RustLang::new();
    let mut fb = FunSpec::<RustLang>::builder("new");
    fb.is_constructor();
    fb.visibility(Visibility::Public);
    fb.add_param(ParameterSpec::new("name", TypeName::primitive("&str")));
    fb.returns(TypeName::primitive("Self"));
    fb.body(CodeBlock::of("Self { name: name.to_string() }", ()).unwrap());
    let output = render_fun(&fb.build(), &rs, DeclarationContext::TopLevel);
    // Must keep "fn" keyword: `pub fn new(name: &str) -> Self {`
    assert!(output.contains("pub fn new(name: &str) -> Self {"));
}

// ── Super delegation ─────────────────────────────────────

#[test]
fn test_js_constructor_with_super() {
    let js = JavaScript::new();
    let mut fb = FunSpec::<JavaScript>::builder("constructor");
    fb.is_constructor();
    fb.add_param(ParameterSpec::new("name", TypeName::primitive("")));
    fb.add_param(ParameterSpec::new("breed", TypeName::primitive("")));
    fb.body(CodeBlock::of("super(name);\nthis.breed = breed;", ()).unwrap());
    let output = render_fun(&fb.build(), &js, DeclarationContext::Member);
    assert!(output.contains("constructor(name, breed) {"));
    assert!(output.contains("super(name);"));
    assert!(output.contains("this.breed = breed;"));
}

#[test]
fn test_java_constructor_with_super() {
    let java = JavaLang::new();
    let mut fb = FunSpec::<JavaLang>::builder("Dog");
    fb.is_constructor();
    fb.visibility(Visibility::Public);
    fb.add_param(ParameterSpec::new("name", TypeName::primitive("String")));
    fb.body(CodeBlock::of("super(name);", ()).unwrap());
    let output = render_fun(&fb.build(), &java, DeclarationContext::Member);
    assert!(output.contains("public Dog(String name) {"));
    assert!(output.contains("super(name);"));
}

// ── Backward compatibility ───────────────────────────────

#[test]
fn test_backward_compat_ts_constructor_without_flag() {
    // Existing pattern: FunSpec with name "constructor" and no is_constructor flag
    // should still work because TS function_keyword(Member) already returns "".
    let ts = TypeScript::new();
    let mut fb = FunSpec::<TypeScript>::builder("constructor");
    fb.add_param(ParameterSpec::new("name", TypeName::primitive("string")));
    fb.body(CodeBlock::of("this.name = name", ()).unwrap());
    let output = render_fun(&fb.build(), &ts, DeclarationContext::Member);
    assert!(output.contains("constructor(name: string) {"));
}

#[test]
fn test_backward_compat_java_constructor_without_flag() {
    // Existing pattern: FunSpec with class name and no is_constructor flag.
    let java = JavaLang::new();
    let mut fb = FunSpec::<JavaLang>::builder("UserService");
    fb.visibility(Visibility::Public);
    fb.add_param(ParameterSpec::new("repo", TypeName::primitive("UserRepository")));
    fb.body(CodeBlock::of("this.repo = repo;", ()).unwrap());
    let output = render_fun(&fb.build(), &java, DeclarationContext::Member);
    assert!(output.contains("public UserService(UserRepository repo) {"));
}
