use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::csharp::CSharp;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder_with("Test.cs", CSharp::new())
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_null_conditional() {
    let block = sigil_quote!(CSharp {
        var name = user?.Profile?.DisplayName;
        var count = items?.Count ?? 0;
    })
    .unwrap();
    golden::assert_golden("csharp/quote_null_conditional.cs", &render(&block));
}

#[test]
fn test_async_await() {
    let block = sigil_quote!(CSharp {
        async Task<string> FetchDataAsync(string url) {
            using var client = new HttpClient();
            var response = await client.GetAsync(url);
            return await response.Content.ReadAsStringAsync();
        }
    })
    .unwrap();
    golden::assert_golden("csharp/quote_async_await.cs", &render(&block));
}

#[test]
fn test_linq_query() {
    let block = sigil_quote!(CSharp {
        var results = items
            .Where(x => x.IsActive)
            .Select(x => x.Name)
            .ToList();
    })
    .unwrap();
    golden::assert_golden("csharp/quote_linq.cs", &render(&block));
}

#[test]
fn test_pattern_matching() {
    let block = sigil_quote!(CSharp {
        var result = shape switch {
            Circle(c) => c.Radius * c.Radius * Math.PI,
            Rectangle(r) => r.Width * r.Height,
            _ => 0,
        };
    })
    .unwrap();
    golden::assert_golden("csharp/quote_pattern_matching.cs", &render(&block));
}

#[test]
fn test_nullable_type() {
    let block = sigil_quote!(CSharp {
        string? name = null;
        int? count = GetCount();
        var value = name ?? $S("default");
    })
    .unwrap();
    golden::assert_golden("csharp/quote_nullable.cs", &render(&block));
}
