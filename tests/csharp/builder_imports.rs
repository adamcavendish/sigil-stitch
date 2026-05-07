use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::csharp::CSharp;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_import_grouping() {
    let list = TypeName::importable("System.Collections.Generic", "List");
    let task = TypeName::importable("System.Threading.Tasks", "Task");
    let logger = TypeName::importable("Microsoft.Extensions.Logging", "ILogger");
    let user = TypeName::importable("MyApp.Models", "User");

    let mut b = CodeBlock::builder();
    b.add_statement("// %T %T %T %T", (list, task, logger, user));
    let block = b.build().unwrap();

    let file = FileSpec::builder_with("App.cs", CSharp::new())
        .add_code(block)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("csharp/import_grouping.cs", &output);
}

#[test]
fn test_import_dedup() {
    let list = TypeName::importable("System.Linq", "Enumerable");
    let queryable = TypeName::importable("System.Linq", "Queryable");

    let mut b = CodeBlock::builder();
    b.add_statement("// %T %T", (list, queryable));
    let block = b.build().unwrap();

    let file = FileSpec::builder_with("App.cs", CSharp::new())
        .add_code(block)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("csharp/import_dedup.cs", &output);
}
