//! Generate a Bash script — builder API vs `sigil_quote!` comparison.
//!
//! Demonstrates: functions with control flow (`if/then/fi`, `for/do/done`),
//! shebang header, `set -euo pipefail`, `source` imports, string escaping,
//! and `$V` verbatim strings (preserving shell interpolation).
//! The macro approach uses `{ }` blocks that the Bash backend automatically
//! converts to the correct delimiters (`then`/`fi`, `do`/`done`).
//!
//! Run: `cargo run --example bash_codegen`

use sigil_stitch::lang::bash::Bash;
use sigil_stitch::prelude::*;

fn main() {
    println!("=== Builder API ===\n");
    let builder_output = builder_approach();
    println!("{builder_output}");

    println!("=== sigil_quote! Macro ===\n");
    let macro_output = macro_approach();
    println!("{macro_output}");
}

fn builder_approach() -> String {
    let utils = TypeName::importable("./utils.sh", "");

    // --- log function ---
    let mut log_body = CodeBlock::builder();
    log_body.add("local level=$1", ());
    log_body.add_line();
    log_body.add("local message=$2", ());
    log_body.add_line();
    log_body.add("echo \"[$level] $(date): $message\"", ());

    let log_fn = FunSpec::builder("log_message")
        .body(log_body.build().unwrap())
        .build()
        .unwrap();

    // --- deploy function with if/then/fi ---
    let mut deploy_body = CodeBlock::builder();
    deploy_body.add("local env=$1", ());
    deploy_body.add_line();
    deploy_body.add_line();
    deploy_body.add("%<", ());
    deploy_body.add("if [ -z \"$env\" ]; then", ());
    deploy_body.add_line();
    deploy_body.add("%>", ());
    deploy_body.add("log_message \"ERROR\" \"No environment specified\"", ());
    deploy_body.add_line();
    deploy_body.add("return 1", ());
    deploy_body.add_line();
    deploy_body.add("%<", ());
    deploy_body.add("fi", ());
    deploy_body.add_line();
    deploy_body.add("%>", ());
    deploy_body.add_line();
    deploy_body.add("log_message \"INFO\" \"Deploying to $env\"", ());
    deploy_body.add_line();
    deploy_body.add_line();
    deploy_body.add("%<", ());
    deploy_body.add("for service in api worker scheduler; do", ());
    deploy_body.add_line();
    deploy_body.add("%>", ());
    deploy_body.add("log_message \"INFO\" \"Starting $service\"", ());
    deploy_body.add_line();
    deploy_body.add("%<", ());
    deploy_body.add("done", ());
    deploy_body.add_line();
    deploy_body.add("%>", ());

    let deploy_fn = FunSpec::builder("deploy")
        .doc("Deploy services to the given environment.")
        .body(deploy_body.build().unwrap())
        .build()
        .unwrap();

    let trigger = CodeBlock::of("# %T", (utils,)).unwrap();

    FileSpec::builder_with("deploy.bash", Bash::new())
        .header(CodeBlock::of("#!/usr/bin/env bash\nset -euo pipefail", ()).unwrap())
        .add_code(trigger)
        .add_function(log_fn)
        .add_function(deploy_fn)
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

fn macro_approach() -> String {
    let utils = TypeName::importable("./utils.sh", "");

    // $V preserves shell interpolation — $S would escape these.
    let log_body = sigil_quote!(Bash {
        local level=$$1
        local message=$$2
        echo $V("[$level] $(date +%Y-%m-%dT%H:%M:%S): $message")
    })
    .unwrap();

    let log_fn = FunSpec::builder("log_message")
        .body(log_body)
        .build()
        .unwrap();

    // sigil_quote! with { } — Bash backend emits then/fi, do/done.
    // $V is used for strings that need shell expansion at runtime.
    let deploy_body = sigil_quote!(Bash {
        local env=$$1
        local app_name=$V("${APP_NAME:-myapp}")
        local version=$V("$(cat VERSION 2>/dev/null || echo unknown)")

        if [ -z $$env ]; {
            log_message $S("ERROR") $V("No environment specified for ${app_name}")
            return 1
        }

        log_message $S("INFO") $V("Deploying ${app_name} v${version} to ${env}")

        for service in api worker scheduler; {
            log_message $S("INFO") $V("Starting $service on $env")
        }

        log_message $S("INFO") $V("Deploy complete. PID=$$, exit=$?")
    })
    .unwrap();

    let deploy_fn = FunSpec::builder("deploy")
        .doc("Deploy services to the given environment.")
        .body(deploy_body)
        .build()
        .unwrap();

    let trigger = CodeBlock::of("# %T", (utils,)).unwrap();

    FileSpec::builder_with("deploy.bash", Bash::new())
        .header(CodeBlock::of("#!/usr/bin/env bash\nset -euo pipefail", ()).unwrap())
        .add_code(trigger)
        .add_function(log_fn)
        .add_function(deploy_fn)
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}
