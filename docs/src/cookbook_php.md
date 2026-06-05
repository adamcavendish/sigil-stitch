# PHP Cookbook

## Classes and Methods

```rust
# extern crate sigil_stitch;
# use sigil_stitch::prelude::*;
# fn main() -> Result<(), Box<dyn std::error::Error>> {
sigil_quote!(Php {
    class Calculator {
        public function add(int $$a, int $$b): int {
            return $$a + $$b;
        }
    }
})?;
# Ok(())
# }
```

**Key points:**

- PHP uses `?Type` for nullable type declarations (`?string`, `?User`).
- PHP does not use `<>` for generics — the tokenizer correctly treats `<` as comparison.
- `$` in PHP variable names must be escaped as `$$` in templates: `$$a` produces `$a`.
