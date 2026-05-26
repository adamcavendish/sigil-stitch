use sigil_stitch::code_block::CodeBlock;
use sigil_stitch::lang::ruby::Ruby;
use sigil_stitch::prelude::*;
use sigil_stitch::spec::file_spec::FileSpec;

use super::golden;

fn render(block: &CodeBlock) -> String {
    FileSpec::builder_with("test.rb", Ruby::new())
        .add_code(block.clone())
        .build()
        .unwrap()
        .render(80)
        .unwrap()
}

#[test]
fn test_string_interpolation() {
    let block = sigil_quote!(Ruby {
        greeting = "Hello, #{name}!"
        path = "/users/#{id}/posts"
    })
    .unwrap();
    golden::assert_golden("ruby/quote_string_interpolation.rb", &render(&block));
}

#[test]
fn test_instance_variables() {
    let block = sigil_quote!(Ruby {
        @name = $S("Alice")
        @age = 30
        @@count = 1
    })
    .unwrap();
    golden::assert_golden("ruby/quote_instance_vars.rb", &render(&block));
}

#[test]
fn test_method_with_body() {
    let block = sigil_quote!(Ruby {
        def greet(name) {
            $S("Hello, ") + name + $S("!")
        }
    })
    .unwrap();
    golden::assert_golden("ruby/quote_method.rb", &render(&block));
}

#[test]
fn test_unless_control_flow() {
    let block = sigil_quote!(Ruby {
        unless x > 0 {
            $S("not positive")
        } else {
            $S("positive")
        }
    })
    .unwrap();
    golden::assert_golden("ruby/quote_unless.rb", &render(&block));
}

#[test]
fn test_while_loop() {
    let block = sigil_quote!(Ruby {
        while x > 0 {
            x -= 1
        }
    })
    .unwrap();
    golden::assert_golden("ruby/quote_while.rb", &render(&block));
}

#[test]
fn test_for_in_loop() {
    let block = sigil_quote!(Ruby {
        for i in 0..5 {
            puts i
        }
    })
    .unwrap();
    golden::assert_golden("ruby/quote_for_in.rb", &render(&block));
}

#[test]
fn test_name_keyword_escape() {
    let name = "class";
    let block = sigil_quote!(Ruby {
        $N(name) = 1
    })
    .unwrap();

    let output = render(&block);
    assert!(output.contains("class_ = 1"), "got: {output}");
}

#[test]
fn test_class_inheritance() {
    let block = sigil_quote!(Ruby {
        class Dog < Animal {
            def speak {
                $S("woof")
            }
        }
    })
    .unwrap();
    golden::assert_golden("ruby/quote_inheritance.rb", &render(&block));
}
