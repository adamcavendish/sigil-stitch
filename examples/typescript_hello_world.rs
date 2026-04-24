//! Generate a TypeScript file using structural specs.
//!
//! Run with: `cargo run --example typescript_hello_world`

use sigil_stitch::code_block::{CodeBlock, StringLitArg};
use sigil_stitch::spec::field_spec::FieldSpec;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::spec::fun_spec::FunSpec;
use sigil_stitch::spec::modifiers::{TypeKind, Visibility};
use sigil_stitch::spec::parameter_spec::ParameterSpec;
use sigil_stitch::spec::type_spec::TypeSpec;
use sigil_stitch::type_name::TypeName;

fn main() {
    // Define types that need imports.
    let user_type = TypeName::importable_type("./models", "User");
    let not_found = TypeName::importable_type("./errors", "NotFoundError");

    // Build the class using TypeSpec.
    let tb = TypeSpec::builder("UserService", TypeKind::Class)
        .visibility(Visibility::Public)
        .doc("Service for managing users.")
        .add_field(
            FieldSpec::builder("userRepo", TypeName::primitive("UserRepository"))
                .visibility(Visibility::Private)
                .is_readonly()
                .build()
                .unwrap(),
        );

    // Async method with control flow body.
    let mut body = CodeBlock::builder();
    body.add_statement(
        "const user = await this.userRepo.findById(%S)",
        (StringLitArg("id".to_string()),),
    );
    body.begin_control_flow("if (!user)", ());
    body.add_statement("throw new %T('User not found')", (not_found,));
    body.end_control_flow();
    body.add_statement("return user", ());
    let body_block = body.build().unwrap();

    let tb = tb.add_method(
        FunSpec::builder("getUser")
            .is_async()
            .add_param(ParameterSpec::new("id", TypeName::primitive("string")).unwrap())
            .returns(TypeName::generic(
                TypeName::primitive("Promise"),
                vec![user_type],
            ))
            .body(body_block)
            .build()
            .unwrap(),
    );

    // Build the file.
    let spec = FileSpec::builder("UserService.ts")
        .add_type(tb.build().unwrap())
        .build()
        .unwrap();

    // Render at 80 columns.
    let output = spec.render(80).unwrap();
    println!("{output}");
}
