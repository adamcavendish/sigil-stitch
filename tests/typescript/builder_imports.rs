use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_hello_world() {
    let user_type = TypeName::importable_type("./models", "User");

    let mut b = CodeBlock::builder();
    b.add("export class UserService {", ());
    b.add_line();
    b.add("%>", ());
    b.add_statement("private user: %T", (user_type,));
    b.add("%<", ());
    b.add("}", ());
    b.add_line();
    let block = b.build().unwrap();

    let file = FileSpec::builder("UserService.ts")
        .add_code(block)
        .build()
        .unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("typescript/hello_world.ts", &output);
}

#[test]
fn test_conflicting_imports() {
    let user1 = TypeName::importable_type("./models", "User");
    let user2 = TypeName::importable_type("./other", "User");

    let mut b = CodeBlock::builder();
    b.add_statement("const u1: %T = getUser1()", (user1,));
    b.add_statement("const u2: %T = getUser2()", (user2,));
    let block = b.build().unwrap();

    let file = FileSpec::builder("users.ts")
        .add_code(block)
        .build()
        .unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("typescript/conflicting_imports.ts", &output);
}

#[test]
fn test_same_module_multiple_types() {
    let user = TypeName::importable_type("./models", "User");
    let tag = TypeName::importable_type("./models", "Tag");
    let category = TypeName::importable_type("./models", "Category");

    let mut b = CodeBlock::builder();
    b.add_statement("const u: %T = null", (user,));
    b.add_statement("const t: %T = null", (tag,));
    b.add_statement("const c: %T = null", (category,));
    let block = b.build().unwrap();

    let file = FileSpec::builder("types.ts")
        .add_code(block)
        .build()
        .unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("typescript/same_module_types.ts", &output);
}

#[test]
fn test_single_type_reference() {
    let user = TypeName::importable_type("./models", "User");

    let mut b = CodeBlock::builder();
    b.add_statement("type Alias = %T", (user,));
    let block = b.build().unwrap();

    let file = FileSpec::builder("alias.ts")
        .add_code(block)
        .build()
        .unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("typescript/single_type_ref.ts", &output);
}
