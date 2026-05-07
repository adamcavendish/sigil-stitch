use sigil_stitch::lang::csharp::CSharp;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_imports() {
    let list_type = TypeName::importable("System.Collections.Generic", "List");
    let task_type = TypeName::importable("System.Threading.Tasks", "Task");

    let block = sigil_quote!(CSharp {
        $T(list_type)<string> items = new $T(task_type)<string>();
    })
    .unwrap();

    let file = FileSpec::builder_with("Test.cs", CSharp::new())
        .add_code(block)
        .build()
        .unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("csharp/macro_imports.cs", &output);
}
