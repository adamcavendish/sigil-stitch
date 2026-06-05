# Ruby Cookbook

## Classes and Modules

```rust
# extern crate sigil_stitch;
# use sigil_stitch::prelude::*;
# fn main() -> Result<(), Box<dyn std::error::Error>> {
sigil_quote!(Ruby {
    class Greeter {
        attr_reader :name

        def initialize(name) {
            @name = name
        }

        def greet {
            $V("Hello, #{@name}!")
        }
    }
})?;
# Ok(())
# }
```

**Key points:**

- Ruby uses `{ }` blocks in `sigil_quote!` — the Ruby backend translates them to `do`/`end` or indent/dedent as appropriate.
- Symbol literals like `:name` get correct spacing (space before `:`, none after).
- Inheritance uses `<` with space before it: `class Dog < Animal`.
- `$V` passes strings through for Ruby interpolation (`#{...}`).
