use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::swift::Swift;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

#[test]
fn test_function_with_imports() {
    let url = TypeName::<Swift>::importable("Foundation", "URL");
    let user = TypeName::<Swift>::importable("MyModule", "User");

    let mut b = CodeBlock::<Swift>::builder();
    b.add_statement("let endpoint: %T = getEndpoint()", (url,));
    b.add_statement("let user: %T = fetchUser(endpoint)", (user,));
    let block = b.build().unwrap();

    let mut fb = FileSpec::builder_with("App.swift", Swift::new());
    fb.add_code(block);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("swift/function_with_imports.swift", &output);
}

#[test]
fn test_import_grouping() {
    let url = TypeName::<Swift>::importable("Foundation", "URL");
    let view = TypeName::<Swift>::importable("SwiftUI", "View");
    let vc = TypeName::<Swift>::importable("UIKit", "UIViewController");
    let combine = TypeName::<Swift>::importable("Combine", "Publisher");
    let alamofire = TypeName::<Swift>::importable("Alamofire", "Session");
    let my_type = TypeName::<Swift>::importable("MyModule", "MyType");

    let mut b = CodeBlock::<Swift>::builder();
    b.add(
        "// %T %T %T %T %T %T",
        (url, view, vc, combine, alamofire, my_type),
    );
    b.add_line();
    let block = b.build().unwrap();

    let mut fb = FileSpec::builder_with("Imports.swift", Swift::new());
    fb.add_code(block);
    let file = fb.build().unwrap();
    let output = file.render(80).unwrap();

    golden::assert_golden("swift/import_grouping.swift", &output);
}
