//! Code generator for `sigil_quote!`.
//!
//! Converts parsed statements into a `TokenStream` of `CodeBlockBuilder` method calls.

use proc_macro2::TokenStream;
use quote::quote;

use crate::parse::{InterpolationKind, ParsedInput, Statement, TypedArg};

/// Generate the output token stream from a parsed `sigil_quote!` input.
pub(crate) fn generate(input: ParsedInput) -> TokenStream {
    let builder_calls = generate_statements(&input.statements);

    quote! {
        {
            let mut __sigil_builder = ::sigil_stitch::code_block::CodeBlock::builder();
            #(#builder_calls)*
            __sigil_builder.build()
        }
    }
}

/// Generate builder calls for a list of statements.
fn generate_statements(statements: &[Statement]) -> Vec<TokenStream> {
    let mut calls = Vec::new();
    for stmt in statements {
        match stmt {
            Statement::BlankLine => {
                calls.push(quote! {
                    __sigil_builder.add_line();
                });
            }
            Statement::Indent => {
                calls.push(quote! {
                    __sigil_builder.add("%>", ());
                });
            }
            Statement::Dedent => {
                calls.push(quote! {
                    __sigil_builder.add("%<", ());
                });
            }
            Statement::Comment(text) => {
                calls.push(quote! {
                    __sigil_builder.add_comment(#text);
                });
            }
            Statement::Statement { format, args } => {
                let args_tuple = build_args_tuple(args);
                calls.push(quote! {
                    __sigil_builder.add_statement(#format, #args_tuple);
                });
            }
            Statement::Line { format, args } => {
                let args_tuple = build_args_tuple(args);
                calls.push(quote! {
                    __sigil_builder.add(#format, #args_tuple);
                    __sigil_builder.add_line();
                });
            }
            Statement::ControlFlow { branches } => {
                for (i, branch) in branches.iter().enumerate() {
                    let fmt = &branch.condition_format;
                    let args_tuple = build_args_tuple(&branch.condition_args);
                    let body_calls = generate_statements(&branch.body);

                    if i == 0 {
                        if let Some(ref custom_open) = branch.block_open_override {
                            calls.push(quote! {
                                __sigil_builder.begin_control_flow_with_open(#fmt, #args_tuple, #custom_open);
                            });
                        } else {
                            calls.push(quote! {
                                __sigil_builder.begin_control_flow(#fmt, #args_tuple);
                            });
                        }
                    } else {
                        calls.push(quote! {
                            __sigil_builder.next_control_flow(#fmt, #args_tuple);
                        });
                    }

                    calls.extend(body_calls);
                }
                calls.push(quote! {
                    __sigil_builder.end_control_flow();
                });
            }
        }
    }
    calls
}

/// Build the args tuple expression from typed args.
///
/// Wraps each arg according to its interpolation kind:
/// - `$T(expr)` -> bare expr (must be `TypeName`)
/// - `$N(expr)` -> `NameArg((expr).to_string())`
/// - `$S(expr)` -> `StringLitArg((expr).to_string())`
/// - `$L(expr)` -> bare expr (via `Into<Arg>`)
/// - `$C(expr)` -> bare expr (must be `CodeBlock`)
fn build_args_tuple(args: &[TypedArg]) -> TokenStream {
    if args.is_empty() {
        quote! { () }
    } else {
        let arg_exprs: Vec<TokenStream> = args
            .iter()
            .map(|arg| {
                let expr = &arg.expr;
                match arg.kind {
                    InterpolationKind::Type
                    | InterpolationKind::Literal
                    | InterpolationKind::Code => {
                        quote! { #expr }
                    }
                    InterpolationKind::Name => {
                        quote! { ::sigil_stitch::code_block::NameArg((#expr).to_string()) }
                    }
                    InterpolationKind::StringLit => {
                        quote! { ::sigil_stitch::code_block::StringLitArg((#expr).to_string()) }
                    }
                }
            })
            .collect();
        quote! { (#(#arg_exprs,)*) }
    }
}
