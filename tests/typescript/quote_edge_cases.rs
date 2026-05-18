use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder("test.ts")
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_object_literal() {
    let block = sigil_quote!(TypeScript {
        const config = { timeout: 5000, retries: 3 };
        const nested = { a: 1, b: { c: 2 } };
    })
    .unwrap();
    golden::assert_golden("typescript/macro_object_literal.ts", &render(&block));
}

#[test]
fn test_generic_function() {
    let block = sigil_quote!(TypeScript {
        function identity<T>(arg: T): T {
            return arg;
        }
        const result = identity<string>($S("hello"));
    })
    .unwrap();
    golden::assert_golden("typescript/quote_generic_function.ts", &render(&block));
}

#[test]
fn test_async_await() {
    let block = sigil_quote!(TypeScript {
        async function fetchData(url: string): Promise<Response> {
            const response = await fetch(url);
            return response.json();
        }
    })
    .unwrap();
    golden::assert_golden("typescript/quote_async_await.ts", &render(&block));
}

#[test]
fn test_union_and_intersection() {
    let block = sigil_quote!(TypeScript {
        type StringOrNumber = string | number;
        type Combined = TypeA & TypeB;
        function process(value: string | number | null): void {
            console.log(value);
        }
    })
    .unwrap();
    golden::assert_golden("typescript/quote_union_intersection.ts", &render(&block));
}

#[test]
fn test_destructuring() {
    let block = sigil_quote!(TypeScript {
        const { name, age } = person;
        const [first, ...rest] = items;
        const { data: { users } } = response;
    })
    .unwrap();
    golden::assert_golden("typescript/quote_destructuring.ts", &render(&block));
}

#[test]
fn test_mapped_type() {
    let block = sigil_quote!(TypeScript {
        type Readonly<T> = {
            readonly [K in keyof T]: T[K];
        };
    })
    .unwrap();
    golden::assert_golden("typescript/quote_mapped_type.ts", &render(&block));
}

#[test]
fn test_name_keyword_escape_in_macro() {
    let name = "class";
    let block = sigil_quote!(TypeScript {
        const $N(name) = 1;
    })
    .unwrap();

    let output = render(&block);
    assert!(output.contains("const class_ = 1;"), "got: {output}");
}
