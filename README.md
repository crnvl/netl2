# NETL2 - Streamlined high-level scripting
NETL2 is a scripting language that takes the no-boilerplate approach of Rust to the extreme while still
maintaining a readability you would expect from high-level languages. Additionally, NETL2 is designed to be concise and
streamlined, so that anti-patterns are difficult to write and the language is easy to learn.

## Features
- Concise syntax
- Absolutely no boilerplate
- Easy to learn

## Learning NETL2
Get started [here](./docs/LEARN.md)

## Code examples
```rs
    f test_fn {
        v test = 20

        w test > 0 {
            p(test)
            test = test - 1

            i test == 10 {
                p("test is 10")
            }
        }
    }

    f nested_fn {
        v test = 3
        p(test)
        test_fn!
    }
    nested_fn!
```