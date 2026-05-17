use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::cpp_lang::CppLang;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;
use sigil_stitch::type_name::TypeName;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder_with("test.cpp", CppLang::new())
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_includes() {
    let iostream = TypeName::importable("iostream", "cout");
    let memory = TypeName::importable("memory", "unique_ptr");
    let block = sigil_quote!(CppLang {
        auto ptr = std::make_unique<int>(42);
        $T(iostream) << $T(memory)(ptr.get()) << std::endl;
    })
    .unwrap();
    golden::assert_golden("cpp/macro_includes.cpp", &render(&block));
}

#[test]
fn test_lambda() {
    let block = sigil_quote!(CppLang {
        auto fn = [&](int x) {
            return x * 2;
        };
        auto result = fn(21);
    })
    .unwrap();
    golden::assert_golden("cpp/quote_lambda.cpp", &render(&block));
}

#[test]
fn test_template_angle_brackets() {
    let block = sigil_quote!(CppLang {
        std::vector<std::pair<int, std::string>> items;
        std::map<std::string, std::vector<int>> index;
    })
    .unwrap();
    golden::assert_golden("cpp/quote_template_angle.cpp", &render(&block));
}

#[test]
fn test_range_for() {
    let block = sigil_quote!(CppLang {
        for (const auto& item : items) {
            std::cout << item << std::endl;
        }
    })
    .unwrap();
    golden::assert_golden("cpp/quote_range_for.cpp", &render(&block));
}

#[test]
fn test_smart_pointers() {
    let block = sigil_quote!(CppLang {
        auto ptr = std::make_unique<Config>();
        auto shared = std::make_shared<Node>(42);
        std::weak_ptr<Node> weak = shared;
    })
    .unwrap();
    golden::assert_golden("cpp/quote_smart_pointers.cpp", &render(&block));
}
