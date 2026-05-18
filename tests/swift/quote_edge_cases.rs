use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::swift::Swift;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder_with("test.swift", Swift::new())
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_optional_chaining() {
    let block = sigil_quote!(Swift {
        let name = user?.profile?.displayName;
        let count = items?.count ?? 0;
    })
    .unwrap();
    golden::assert_golden("swift/quote_optional_chain.swift", &render(&block));
}

#[test]
fn test_guard_let() {
    let block = sigil_quote!(Swift {
        func process(value: Int?) {
            guard let unwrapped = value else {
                return;
            }
            print(unwrapped);
        }
    })
    .unwrap();
    golden::assert_golden("swift/quote_guard_let.swift", &render(&block));
}

#[test]
fn test_closure() {
    let block = sigil_quote!(Swift {
        let transform: (Int) -> Int = { (x: Int) -> Int in
            return x * 2
        };
        let sorted = items.sorted(by: { a, b in a.name < b.name });
    })
    .unwrap();
    golden::assert_golden("swift/quote_closure.swift", &render(&block));
}

#[test]
fn test_protocol_extension() {
    let block = sigil_quote!(Swift {
        protocol Drawable {
            func draw();
        }
        extension Drawable {
            func draw() {
                print($S("default draw"));
            }
        }
    })
    .unwrap();
    golden::assert_golden("swift/quote_protocol_extension.swift", &render(&block));
}

#[test]
fn test_enum_with_associated_values() {
    let block = sigil_quote!(Swift {
        enum Result<T> {
            case success(T)
            case failure(Error)
        }
    })
    .unwrap();
    golden::assert_golden("swift/quote_enum_associated.swift", &render(&block));
}

#[test]
fn test_name_keyword_escape_in_macro() {
    let name = "class";
    let block = sigil_quote!(Swift {
        let $N(name) = 1
    })
    .unwrap();

    let output = render(&block);
    assert!(output.contains("let `class` = 1"), "got: {output}");
}
