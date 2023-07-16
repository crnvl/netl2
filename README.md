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
```nl
    v var_name = 2

    w var_name == 2 {
        p("Hello World!")
        var_name = 3
    }

    f main {
        p("Main function!")
    }

    i var_name != 2 {
        main!
    }
```