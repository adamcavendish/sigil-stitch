//! Token stream parser for `sigil_quote!`.
//!
//! Parses the macro input into a structured `ParsedInput` containing the
//! language type and a list of statements.

use proc_macro2::{Delimiter, Spacing, Span, TokenStream, TokenTree};

/// A parsed `sigil_quote!` invocation.
pub(crate) struct ParsedInput {
    /// The language type tokens (e.g., `TypeScript`).
    pub lang_type: TokenStream,
    /// The parsed body statements.
    pub statements: Vec<Statement>,
}

/// A single statement or directive in the macro body.
#[allow(clippy::enum_variant_names)]
pub(crate) enum Statement {
    /// `add_statement(format, args)` — line ending with `;`.
    Statement { format: String, args: Vec<TypedArg> },
    /// `add(format, args)` + newline — line without `;`.
    Line { format: String, args: Vec<TypedArg> },
    /// `add_line()` — blank line.
    BlankLine,
    /// `add_comment(text)` — `$comment("text")`.
    Comment(String),
    /// Control flow: `begin_control_flow` / `next_control_flow` / `end_control_flow`.
    ControlFlow { branches: Vec<Branch> },
    /// `add("%>", ())` — increase indent.
    Indent,
    /// `add("%<", ())` — decrease indent.
    Dedent,
}

/// A single branch in a control flow chain.
pub(crate) struct Branch {
    /// Format string for the condition (e.g., `"if (x > 0)"`).
    pub condition_format: String,
    /// Interpolation args for the condition.
    pub condition_args: Vec<TypedArg>,
    /// Body statements inside the braces.
    pub body: Vec<Statement>,
    /// Custom block opener override from `$open("...")`, only on the first branch.
    pub block_open_override: Option<String>,
}

/// An interpolation argument with its kind for proper wrapping in codegen.
pub(crate) struct TypedArg {
    pub kind: InterpolationKind,
    pub expr: TokenStream,
}

/// The kind of an interpolation marker.
#[derive(Clone, Copy)]
pub(crate) enum InterpolationKind {
    /// `$T(expr)` — type reference.
    Type,
    /// `$N(expr)` — name identifier.
    Name,
    /// `$S(expr)` — string literal.
    StringLit,
    /// `$L(expr)` — literal value.
    Literal,
    /// `$C(expr)` — nested code block.
    Code,
}

/// Parse the full `sigil_quote!` input.
///
/// Expected form: `LangType { body }`
pub(crate) fn parse_input(input: TokenStream) -> Result<ParsedInput, CompileError> {
    let tokens: Vec<TokenTree> = input.into_iter().collect();
    if tokens.is_empty() {
        return Err(CompileError::new(
            Span::call_site(),
            "sigil_quote! requires a language type and body",
        ));
    }

    // Find the body group (last token must be a brace group).
    let last = &tokens[tokens.len() - 1];
    let body_group = match last {
        TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => g.clone(),
        _ => {
            return Err(CompileError::new(
                last.span(),
                "sigil_quote! body must be enclosed in braces: sigil_quote!(Type { ... })",
            ));
        }
    };

    // Everything before the body group is the language type.
    let lang_tokens: TokenStream = tokens[..tokens.len() - 1].iter().cloned().collect();
    if lang_tokens.is_empty() {
        return Err(CompileError::new(
            Span::call_site(),
            "sigil_quote! requires a language type before the body: sigil_quote!(Type { ... })",
        ));
    }

    let body_tokens: Vec<TokenTree> = body_group.stream().into_iter().collect();
    let statements = parse_body(&body_tokens)?;

    Ok(ParsedInput {
        lang_type: lang_tokens,
        statements,
    })
}

/// A compile error with span information.
pub(crate) struct CompileError {
    span: Span,
    message: String,
}

impl CompileError {
    pub fn new(span: Span, message: impl Into<String>) -> Self {
        CompileError {
            span,
            message: message.into(),
        }
    }

    /// Convert to a `compile_error!()` token stream.
    pub fn into_compile_error(self) -> TokenStream {
        let msg = &self.message;
        // Use the span to attach the error to the right location.
        let mut ts = TokenStream::new();
        let err = quote::quote_spanned! { self.span =>
            ::core::compile_error!(#msg)
        };
        ts.extend(err);
        ts
    }
}

/// Parse the body tokens into a list of statements.
fn parse_body(tokens: &[TokenTree]) -> Result<Vec<Statement>, CompileError> {
    let mut statements = Vec::new();
    let mut pos = 0;

    // Track the line of the last consumed token for blank-line detection.
    let mut prev_line: Option<usize> = None;

    while pos < tokens.len() {
        // Detect blank lines via span-location gaps.
        let current_line = tokens[pos].span().start().line;
        if let Some(pl) = prev_line {
            for _ in 0..(current_line.saturating_sub(pl).saturating_sub(1)) {
                statements.push(Statement::BlankLine);
            }
        }

        let (stmt, next_pos) = parse_one_statement(tokens, pos)?;
        // Track the line of the last token consumed.
        if next_pos > pos {
            prev_line = Some(tokens[next_pos - 1].span().end().line);
        }
        statements.push(stmt);
        pos = next_pos;
    }

    Ok(statements)
}

/// Parse a single statement starting at `pos`.
/// Returns the statement and the position after the consumed tokens.
fn parse_one_statement(
    tokens: &[TokenTree],
    start: usize,
) -> Result<(Statement, usize), CompileError> {
    // Check for $comment(...) at current position.
    if let Some((comment_text, next)) = try_parse_comment(tokens, start)? {
        return Ok((Statement::Comment(comment_text), next));
    }

    // Check for $> or $< at current position.
    if let Some((stmt, next)) = try_parse_indent_directive(tokens, start) {
        return Ok((stmt, next));
    }

    // Collect tokens for this statement, looking for `;` or a brace group.
    let mut pos = start;
    let mut collected: Vec<TokenTree> = Vec::new();

    while pos < tokens.len() {
        let tt = &tokens[pos];

        // Check for `;` — statement terminator.
        if is_semicolon(tt) {
            let (format, args) = tokens_to_format(&collected)?;
            return Ok((Statement::Statement { format, args }, pos + 1));
        }

        // Check for brace group — potential control flow.
        if let TokenTree::Group(g) = tt
            && g.delimiter() == Delimiter::Brace
        {
            // Look ahead: if next token is `;`, this is NOT control flow
            // (it's an object literal or struct init in a statement).
            let next = pos + 1;
            if next < tokens.len() && is_semicolon(&tokens[next]) {
                // Part of a statement: `const x = { ... };`
                collected.push(tt.clone());
                pos += 1;
                continue;
            }

            // Check for $open("...") at end of collected tokens.
            let (condition_tokens, block_open_override) = try_extract_open_override(&collected)?;

            // Control flow detected.
            return parse_control_flow(tokens, &condition_tokens, g, pos, block_open_override);
        }

        collected.push(tt.clone());
        pos += 1;
    }

    // End of input without `;` — emit as a Line.
    if collected.is_empty() {
        Ok((Statement::BlankLine, pos))
    } else {
        let (format, args) = tokens_to_format(&collected)?;
        Ok((Statement::Line { format, args }, pos))
    }
}

/// Check if the last tokens in `collected` form `$open("text")`.
/// Returns the remaining condition tokens and the optional override string.
fn try_extract_open_override(
    collected: &[TokenTree],
) -> Result<(Vec<TokenTree>, Option<String>), CompileError> {
    let n = collected.len();
    if n < 3 {
        return Ok((collected.to_vec(), None));
    }

    // Check for pattern: Punct($) Ident(open) Group(Paren containing string literal)
    let dollar = &collected[n - 3];
    let ident = &collected[n - 2];
    let group = &collected[n - 1];

    let is_dollar = matches!(dollar, TokenTree::Punct(p) if p.as_char() == '$');
    let is_open = is_ident(ident, "open");
    let paren_group = if let TokenTree::Group(g) = group
        && g.delimiter() == Delimiter::Parenthesis
    {
        Some(g)
    } else {
        None
    };

    if !is_dollar || !is_open || paren_group.is_none() {
        return Ok((collected.to_vec(), None));
    }

    let g = paren_group.unwrap();
    let inner: Vec<TokenTree> = g.stream().into_iter().collect();
    if inner.len() != 1 {
        return Err(CompileError::new(
            g.span(),
            "$open requires a single string literal: $open(\"text\")",
        ));
    }

    let text = match &inner[0] {
        TokenTree::Literal(lit) => {
            let s = lit.to_string();
            if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
                let raw = &s[1..s.len() - 1];
                match unescape_string(raw) {
                    Ok(text) => text,
                    Err(msg) => return Err(CompileError::new(lit.span(), &msg)),
                }
            } else {
                return Err(CompileError::new(
                    lit.span(),
                    "$open requires a string literal",
                ));
            }
        }
        _ => {
            return Err(CompileError::new(
                inner[0].span(),
                "$open requires a string literal",
            ));
        }
    };

    let condition_tokens = collected[..n - 3].to_vec();
    Ok((condition_tokens, Some(text)))
}

/// Parse a control flow chain starting from tokens that lead into a brace group.
fn parse_control_flow(
    tokens: &[TokenTree],
    condition_tokens: &[TokenTree],
    first_brace: &proc_macro2::Group,
    brace_pos: usize,
    block_open_override: Option<String>,
) -> Result<(Statement, usize), CompileError> {
    let (cond_format, cond_args) = tokens_to_format(condition_tokens)?;
    let body_tokens: Vec<TokenTree> = first_brace.stream().into_iter().collect();
    let body = parse_body(&body_tokens)?;

    let mut branches = vec![Branch {
        condition_format: cond_format,
        condition_args: cond_args,
        body,
        block_open_override,
    }];

    let mut pos = brace_pos + 1;

    // Check for else chain.
    while pos < tokens.len() {
        if is_ident(&tokens[pos], "else") {
            let else_span = tokens[pos].span();
            pos += 1; // consume `else`

            // Collect tokens until we find a brace group (handles `else if (...) {`).
            let mut else_condition_tokens: Vec<TokenTree> = Vec::new();
            let mut found_brace = false;

            while pos < tokens.len() {
                if let TokenTree::Group(g) = &tokens[pos]
                    && g.delimiter() == Delimiter::Brace
                {
                    let body_toks: Vec<TokenTree> = g.stream().into_iter().collect();
                    let body = parse_body(&body_toks)?;

                    let (cond_format, cond_args) = if else_condition_tokens.is_empty() {
                        ("else".to_string(), Vec::new())
                    } else {
                        let (fmt, args) = tokens_to_format(&else_condition_tokens)?;
                        (format!("else {fmt}"), args)
                    };

                    branches.push(Branch {
                        condition_format: cond_format,
                        condition_args: cond_args,
                        body,
                        block_open_override: None,
                    });
                    pos += 1;
                    found_brace = true;
                    break;
                }
                else_condition_tokens.push(tokens[pos].clone());
                pos += 1;
            }

            if !found_brace {
                return Err(CompileError::new(else_span, "expected `{` after `else`"));
            }
        } else {
            break;
        }
    }

    Ok((Statement::ControlFlow { branches }, pos))
}

/// Check for `$>` or `$<` at position `start`.
fn try_parse_indent_directive(tokens: &[TokenTree], start: usize) -> Option<(Statement, usize)> {
    if start + 1 >= tokens.len() {
        return None;
    }
    let is_dollar = matches!(&tokens[start], TokenTree::Punct(p) if p.as_char() == '$');
    if !is_dollar {
        return None;
    }
    if let TokenTree::Punct(p2) = &tokens[start + 1] {
        match p2.as_char() {
            '>' => return Some((Statement::Indent, start + 2)),
            '<' => return Some((Statement::Dedent, start + 2)),
            _ => {}
        }
    }
    None
}

/// Try to parse `$comment("text")` at position `start`.
fn try_parse_comment(
    tokens: &[TokenTree],
    start: usize,
) -> Result<Option<(String, usize)>, CompileError> {
    // Need at least 3 tokens: `$`, `comment`, `("text")`
    if start + 2 >= tokens.len() {
        return Ok(None);
    }

    // Check for `$` punct.
    let _dollar = match &tokens[start] {
        TokenTree::Punct(p) if p.as_char() == '$' => p,
        _ => return Ok(None),
    };

    // Check for `comment` ident.
    if !is_ident(&tokens[start + 1], "comment") {
        return Ok(None);
    }

    // Check for parenthesized string literal.
    let group = match &tokens[start + 2] {
        TokenTree::Group(g) if g.delimiter() == Delimiter::Parenthesis => g,
        _ => {
            return Err(CompileError::new(
                tokens[start + 2].span(),
                "$comment requires parenthesized string: $comment(\"text\")",
            ));
        }
    };

    let inner: Vec<TokenTree> = group.stream().into_iter().collect();
    if inner.len() != 1 {
        return Err(CompileError::new(
            group.span(),
            "$comment requires a single string literal: $comment(\"text\")",
        ));
    }

    let text = match &inner[0] {
        TokenTree::Literal(lit) => {
            let s = lit.to_string();
            // Strip surrounding quotes and unescape.
            if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
                let raw = &s[1..s.len() - 1];
                match unescape_string(raw) {
                    Ok(text) => text,
                    Err(msg) => {
                        return Err(CompileError::new(lit.span(), &msg));
                    }
                }
            } else {
                return Err(CompileError::new(
                    lit.span(),
                    "$comment requires a string literal",
                ));
            }
        }
        _ => {
            return Err(CompileError::new(
                inner[0].span(),
                "$comment requires a string literal",
            ));
        }
    };

    // Skip optional semicolon after $comment("text");
    let mut next = start + 3;
    if next < tokens.len() && is_semicolon(&tokens[next]) {
        next += 1;
    }

    Ok(Some((text, next)))
}

/// Convert a sequence of tokens into a format string and typed argument list.
///
/// Handles interpolation markers (`$T(expr)`, `$W`, `$$`, etc.) and
/// escapes `%` to `%%` in literal text. Recursively handles groups.
pub(crate) fn tokens_to_format(
    tokens: &[TokenTree],
) -> Result<(String, Vec<TypedArg>), CompileError> {
    let mut format = String::new();
    let mut args: Vec<TypedArg> = Vec::new();
    let mut prev_kind = PrevTokenKind::None;

    tokens_to_format_inner(tokens, &mut format, &mut args, &mut prev_kind)?;

    Ok((format, args))
}

/// What kind of token was just emitted (for spacing decisions).
#[derive(Clone, Copy, PartialEq)]
enum PrevTokenKind {
    None,
    Ident,
    Punct(char, Spacing),
    Literal,
    GroupOpen, // just emitted `(` or `[`
    Specifier, // just emitted `%T`, `%N`, etc.
}

fn tokens_to_format_inner(
    tokens: &[TokenTree],
    format: &mut String,
    args: &mut Vec<TypedArg>,
    prev_kind: &mut PrevTokenKind,
) -> Result<(), CompileError> {
    let mut pos = 0;

    while pos < tokens.len() {
        let tt = &tokens[pos];

        // Check for `$` interpolation.
        if let TokenTree::Punct(p) = tt
            && p.as_char() == '$'
        {
            pos += 1;
            if pos >= tokens.len() {
                return Err(CompileError::new(
                    p.span(),
                    "unexpected `$` at end of input",
                ));
            }

            let next = &tokens[pos];

            // `$$` -> literal `$`
            if let TokenTree::Punct(p2) = next
                && p2.as_char() == '$'
            {
                maybe_space(format, *prev_kind, PrevTokenKind::Literal);
                format.push('$');
                *prev_kind = PrevTokenKind::Literal;
                pos += 1;
                continue;
            }

            // `$>` -> `%>` (indent)
            if let TokenTree::Punct(p2) = next
                && p2.as_char() == '>'
            {
                format.push_str("%>");
                *prev_kind = PrevTokenKind::Specifier;
                pos += 1;
                continue;
            }

            // `$<` -> `%<` (dedent)
            if let TokenTree::Punct(p2) = next
                && p2.as_char() == '<'
            {
                format.push_str("%<");
                *prev_kind = PrevTokenKind::Specifier;
                pos += 1;
                continue;
            }

            // `$W` -> `%W` (no arg, no parens)
            if is_ident(next, "W") {
                // %W is a non-arg specifier, no space logic needed.
                format.push_str("%W");
                *prev_kind = PrevTokenKind::Specifier;
                pos += 1;
                continue;
            }

            // `$comment(...)` should have been caught earlier.
            if is_ident(next, "comment") {
                return Err(CompileError::new(
                    next.span(),
                    "$comment() must appear at the start of a line",
                ));
            }

            // `$T(expr)`, `$N(expr)`, `$S(expr)`, `$L(expr)`, `$C(expr)`
            if let TokenTree::Ident(id) = next {
                let kind_str = id.to_string();
                let kind = match kind_str.as_str() {
                    "T" => InterpolationKind::Type,
                    "N" => InterpolationKind::Name,
                    "S" => InterpolationKind::StringLit,
                    "L" => InterpolationKind::Literal,
                    "C" => InterpolationKind::Code,
                    _ => {
                        return Err(CompileError::new(
                            id.span(),
                            format!(
                                "unknown interpolation kind `${kind_str}`. \
                                     Expected $T, $N, $S, $L, $C, or $W"
                            ),
                        ));
                    }
                };

                pos += 1;
                if pos >= tokens.len() {
                    return Err(CompileError::new(
                        id.span(),
                        format!(
                            "${kind_str} requires a parenthesized expression: ${kind_str}(expr)"
                        ),
                    ));
                }

                let group = match &tokens[pos] {
                    TokenTree::Group(g) if g.delimiter() == Delimiter::Parenthesis => g,
                    _ => {
                        return Err(CompileError::new(
                            tokens[pos].span(),
                            format!(
                                "${kind_str} requires a parenthesized expression: ${kind_str}(expr)"
                            ),
                        ));
                    }
                };

                let specifier = match kind {
                    InterpolationKind::Type => "%T",
                    InterpolationKind::Name => "%N",
                    InterpolationKind::StringLit => "%S",
                    InterpolationKind::Literal | InterpolationKind::Code => "%L",
                };

                maybe_space(format, *prev_kind, PrevTokenKind::Specifier);
                format.push_str(specifier);
                *prev_kind = PrevTokenKind::Specifier;

                args.push(TypedArg {
                    kind,
                    expr: group.stream(),
                });

                pos += 1;
                continue;
            }

            return Err(CompileError::new(
                next.span(),
                "expected interpolation kind after `$`: $T, $N, $S, $L, $C, $W, or $$",
            ));
        }

        // Regular tokens.
        match tt {
            TokenTree::Ident(id) => {
                maybe_space(format, *prev_kind, PrevTokenKind::Ident);
                let s = id.to_string();
                format.push_str(&s.replace('%', "%%"));
                *prev_kind = PrevTokenKind::Ident;
            }
            TokenTree::Punct(p) => {
                let ch = p.as_char();
                let new_kind = PrevTokenKind::Punct(ch, p.spacing());
                maybe_space(format, *prev_kind, new_kind);
                if ch == '%' {
                    format.push_str("%%");
                } else {
                    format.push(ch);
                }
                *prev_kind = new_kind;
            }
            TokenTree::Literal(lit) => {
                maybe_space(format, *prev_kind, PrevTokenKind::Literal);
                let s = lit.to_string();
                format.push_str(&s.replace('%', "%%"));
                *prev_kind = PrevTokenKind::Literal;
            }
            TokenTree::Group(g) => {
                let (open, close) = match g.delimiter() {
                    Delimiter::Parenthesis => ("(", ")"),
                    Delimiter::Bracket => ("[", "]"),
                    Delimiter::Brace => ("{", "}"),
                    Delimiter::None => ("", ""),
                };
                // Space before opening delimiter depends on context.
                let new_kind = PrevTokenKind::GroupOpen;
                maybe_space(format, *prev_kind, new_kind);
                format.push_str(open);
                *prev_kind = PrevTokenKind::GroupOpen;

                // Recursively process inner tokens (handles interpolation inside groups).
                let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                tokens_to_format_inner(&inner, format, args, prev_kind)?;

                format.push_str(close);
                // After closing delimiter, act like a literal/ident for spacing.
                *prev_kind = PrevTokenKind::Literal;
            }
        }
        pos += 1;
    }

    Ok(())
}

/// Insert a space between the previous and current tokens if needed.
fn maybe_space(format: &mut String, prev: PrevTokenKind, current: PrevTokenKind) {
    if prev == PrevTokenKind::None || prev == PrevTokenKind::GroupOpen {
        return;
    }

    // No space before certain punctuation.
    if let PrevTokenKind::Punct(ch, _) = current {
        match ch {
            ',' | ';' | ')' | ']' | '.' => return,
            ':' => {
                // No space before `:` when previous is ident (field: Type, key: value).
                // But we DO want space before `:` in other contexts? Actually in most
                // target languages, `:` has no space before it. Keep no space.
                return;
            }
            _ => {}
        }
    }

    // No space between joint punctuation (===, !==, ->, ::, etc.).
    if let PrevTokenKind::Punct(_, Spacing::Joint) = prev {
        return;
    }

    // No space after opening punctuation.
    if let PrevTokenKind::Punct('(' | '[' | '.' | '!' | '~' | '@' | '#', _) = prev {
        return;
    }

    // No space before `(` when preceded by ident, specifier, or literal.
    // Treats all ident-before-paren as function calls: `getUser(...)`.
    // Keywords like `if`/`for`/`while` would ideally get a space, but proc macros
    // cannot distinguish keywords from identifiers. Both forms are valid in all
    // supported target languages.
    if let PrevTokenKind::GroupOpen = current
        && matches!(
            prev,
            PrevTokenKind::Ident | PrevTokenKind::Specifier | PrevTokenKind::Literal
        )
    {
        return;
    }

    // Default: add a space between tokens.
    format.push(' ');
}

fn is_semicolon(tt: &TokenTree) -> bool {
    matches!(tt, TokenTree::Punct(p) if p.as_char() == ';')
}

fn is_ident(tt: &TokenTree, name: &str) -> bool {
    matches!(tt, TokenTree::Ident(id) if *id == name)
}

fn unescape_string(s: &str) -> Result<String, String> {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some('r') => out.push('\r'),
                Some('0') => out.push('\0'),
                Some('\\') => out.push('\\'),
                Some('"') => out.push('"'),
                Some(other) => {
                    return Err(format!("unknown escape sequence: \\{other}"));
                }
                None => {
                    return Err("unexpected end of string after \\".to_string());
                }
            }
        } else {
            out.push(c);
        }
    }
    Ok(out)
}
