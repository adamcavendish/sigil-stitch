use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::typescript::TypeScript;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_control_flow() {
    let error_type = TypeName::<TypeScript>::importable_type("./errors", "NotFoundError");

    let mut b = CodeBlock::<TypeScript>::builder();
    b.add("export function validate(input: string): boolean {", ());
    b.add_line();
    b.add("%>", ());
    b.begin_control_flow("if (input.length === 0)", ());
    b.add_statement("throw new %T('empty input')", (error_type,));
    b.next_control_flow("else if (input.length > 100)", ());
    b.add_statement("return false", ());
    b.next_control_flow("else", ());
    b.add_statement("return true", ());
    b.end_control_flow();
    b.add("%<", ());
    b.add("}", ());
    b.add_line();
    let block = b.build().unwrap();

    let mut fb = FileSpec::<TypeScript>::builder("validate.ts");
    fb.add_code(block);
    let file = fb.build().unwrap();

    let output = file.render(80).unwrap();
    golden::assert_golden("typescript/control_flow.ts", &output);
}
