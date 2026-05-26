use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::ruby::Ruby;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use crate::shared::LanguageTestSuite;

pub struct RubySuite;

impl LanguageTestSuite for RubySuite {
    fn control_flow_block() -> CodeBlock {
        sigil_quote!(Ruby {
            if x > 0 {
                return $S("positive")
            } else {
                return $S("negative")
            }
        })
        .unwrap()
    }

    fn control_flow_golden_path() -> &'static str {
        "ruby/macro_control_flow.rb"
    }

    fn basic_block() -> CodeBlock {
        sigil_quote!(Ruby {
            x = 42
            name = $S("Alice")
            puts name, x
        })
        .unwrap()
    }

    fn basic_golden_path() -> &'static str {
        "ruby/macro_basic.rb"
    }

    fn render(block: CodeBlock) -> String {
        FileSpec::builder_with("test.rb", Ruby::new())
            .add_code(block)
            .build()
            .unwrap()
            .render(80)
            .unwrap()
    }

    fn file_spec_name() -> &'static str {
        "test.rb"
    }
}
