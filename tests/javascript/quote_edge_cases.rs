use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::javascript::JavaScript;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder_with("test.js", JavaScript::new())
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_arrow_function() {
    let block = sigil_quote!(JavaScript {
        const add = (a, b) => a + b;
        const greet = (name) => {
            return $S("Hello, ") + name;
        };
    })
    .unwrap();
    golden::assert_golden("javascript/quote_arrow_function.js", &render(&block));
}

#[test]
fn test_destructuring() {
    let block = sigil_quote!(JavaScript {
        const { name, age } = user;
        const [first, ...rest] = items;
    })
    .unwrap();
    golden::assert_golden("javascript/quote_destructuring.js", &render(&block));
}

#[test]
fn test_async_await() {
    let block = sigil_quote!(JavaScript {
        async function fetchData(url) {
            const response = await fetch(url);
            const data = await response.json();
            return data;
        }
    })
    .unwrap();
    golden::assert_golden("javascript/quote_async_await.js", &render(&block));
}

#[test]
fn test_spread_operator() {
    let block = sigil_quote!(JavaScript {
        const merged = { ...defaults, ...overrides };
        const combined = [...arr1, ...arr2];
    })
    .unwrap();
    golden::assert_golden("javascript/quote_spread.js", &render(&block));
}

#[test]
fn test_ternary_and_nullish() {
    let block = sigil_quote!(JavaScript {
        const value = x != null ? x : defaultValue;
        const name = user?.name ?? $S("anonymous");
    })
    .unwrap();
    golden::assert_golden("javascript/quote_ternary_nullish.js", &render(&block));
}
