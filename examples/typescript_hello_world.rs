//! Generate a TypeScript file with imports, types, and control flow.
//!
//! Run with: `cargo run --example typescript_hello_world`

use sigil_stitch::code_block::{CodeBlock, StringLitArg};
use sigil_stitch::lang::typescript::TypeScript;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

fn main() {
    // Define types that need imports.
    let user_type = TypeName::<TypeScript>::importable_type("./models", "User");
    let not_found = TypeName::<TypeScript>::importable_type("./errors", "NotFoundError");

    // Build a function body with control flow.
    let mut body = CodeBlock::<TypeScript>::builder();
    body.add_statement("const user = await this.userRepo.findById(%S)", (StringLitArg("id".to_string()),));
    body.begin_control_flow("if (!user)", ());
    body.add_statement("throw new %T('User not found')", (not_found,));
    body.end_control_flow();
    body.add_statement("return user", ());
    let body_block = body.build().unwrap();

    // Build the class.
    let mut class = CodeBlock::<TypeScript>::builder();
    class.add("export class UserService {", ());
    class.add_line();
    class.add("%>", ());
    class.add("async getUser(id: string): Promise<%T> {", (user_type,));
    class.add_line();
    class.add("%>", ());
    class.add_code(body_block);
    class.add("%<", ());
    class.add("}", ());
    class.add_line();
    class.add("%<", ());
    class.add("}", ());
    class.add_line();
    let class_block = class.build().unwrap();

    // Build the file.
    let mut file = FileSpec::<TypeScript>::builder("UserService.ts");
    file.add_code(class_block);
    let spec = file.build();

    // Render at 80 columns.
    let output = spec.render(80).unwrap();
    println!("{output}");
}
