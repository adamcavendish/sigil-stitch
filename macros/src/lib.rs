//! Proc macros for sigil-stitch code generation.
//!
//! This crate provides the `sigil_quote!` macro for writing target-language code
//! inline with interpolation markers that expand to `CodeBlockBuilder` calls.

mod codegen;
mod parse;

use proc_macro::TokenStream;

/// Write target-language code inline, expanding to `CodeBlockBuilder` calls.
///
/// # Syntax
///
/// ```ignore
/// sigil_quote!(LangType {
///     statement with $T(type_expr) and $S("string");
///     if (condition) {
///         body;
///     }
/// })
/// ```
///
/// Returns `Result<CodeBlock<LangType>, SigilStitchError>`.
///
/// ## Interpolation Markers
///
/// | Syntax | Specifier | Purpose |
/// |--------|-----------|---------|
/// | `$T(expr)` | `%T` | Type reference (tracks imports) |
/// | `$N(expr)` | `%N` | Name identifier |
/// | `$S(expr)` | `%S` | String literal |
/// | `$L(expr)` | `%L` | Literal or nested code |
/// | `$C(expr)` | `%L` | Nested `CodeBlock` |
/// | `$W` | `%W` | Soft line-break point |
/// | `$$` | `$` | Literal dollar sign |
///
/// ## Statement Rules
///
/// - Lines ending with `;` become `add_statement()` calls
/// - Lines ending with `{ ... }` become control flow (`begin/end_control_flow`)
/// - Blank lines become `add_line()` calls
/// - `$comment("text")` becomes `add_comment("text")`
///
/// # Examples
///
/// ```ignore
/// use sigil_stitch::prelude::*;
/// use sigil_stitch::lang::typescript::TypeScript;
///
/// let user_type = TypeName::<TypeScript>::importable_type("./models", "User");
///
/// let block = sigil_quote!(TypeScript {
///     const user: $T(user_type) = await getUser($S("id"));
///     return user;
/// })?;
/// ```
#[proc_macro]
pub fn sigil_quote(input: TokenStream) -> TokenStream {
    let input2: proc_macro2::TokenStream = input.into();
    match parse::parse_input(input2) {
        Ok(parsed) => codegen::generate(parsed).into(),
        Err(err) => err.into_compile_error().into(),
    }
}
