//! Test helpers for exact rendered-code assertions.

use crate::code_block::CodeBlock;
use crate::error::SigilStitchError;
use crate::lang::CodeLang;
use crate::spec::file_spec::FileSpec;

/// Render a `CodeBlock` through `FileSpec` for tests.
///
/// This exercises the same import collection and language rendering path as a
/// real file render. The filename is synthetic and based on the language's file
/// extension.
pub fn render_block_for_test(
    lang: impl CodeLang,
    block: &CodeBlock,
    width: usize,
) -> Result<String, SigilStitchError> {
    let filename = format!("test.{}", lang.file_extension());
    FileSpec::builder_with(&filename, lang)
        .add_code(block.clone())
        .build()?
        .render(width)
}

/// Assert that a `CodeBlock` renders exactly through `FileSpec`.
///
/// The assertion is exact: leading whitespace, trailing whitespace, and final
/// newlines are significant.
#[macro_export]
macro_rules! assert_rendered {
    ($lang:expr, width = $width:expr, $block:expr, $expected:expr $(,)?) => {{
        let __sigil_block = $block;
        let __sigil_actual = $crate::testing::render_block_for_test($lang, &__sigil_block, $width)
            .expect("failed to render CodeBlock in assert_rendered!");
        let __sigil_expected = $expected;
        assert_eq!(
            __sigil_actual, __sigil_expected,
            "rendered code mismatch\n\n--- expected ---\n{}\n--- actual ---\n{}",
            __sigil_expected, __sigil_actual,
        );
    }};
    ($lang:expr, $block:expr, $expected:expr $(,)?) => {{
        $crate::assert_rendered!($lang, width = 80, $block, $expected);
    }};
}

/// Assert that an inline `sigil_quote!` body renders exactly.
///
/// The language type must be in scope and provide `new()`. For configured
/// language instances, build the quote separately and use [`assert_rendered!`].
#[macro_export]
macro_rules! assert_quote {
    ($lang:ident, width = $width:expr, { $($body:tt)* }, $expected:expr $(,)?) => {{
        let __sigil_block = $crate::prelude::sigil_quote!($lang { $($body)* })
            .expect("failed to build CodeBlock in assert_quote!");
        $crate::assert_rendered!($lang::new(), width = $width, __sigil_block, $expected);
    }};
    ($lang:ident, { $($body:tt)* }, $expected:expr $(,)?) => {{
        $crate::assert_quote!($lang, width = 80, { $($body)* }, $expected);
    }};
}
