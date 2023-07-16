# Learning NETL2

NETL2 has a very strict design pattern with only a few keywords.
If you're coming from a
high-level language like Python or JavaScript, you may
find that NETL2 does not include the `else` keyword for `if` statements. That is, because `else` can often lead to spaghetti code, and NETL2 is designed to be as streamlined as possible.

Another missing keyword is `return`. To modify a variable, you simply assign it a new value. Functions are called using `!`.

Functions need to be declared first, and then called with the `!` operator. Functions also do not have parameters. If you want to pass a value to a function, you can simply assign it to a variable before calling the function.

# Installation

To use NETL2, you can simply download one of the release builds or, if your platform is not supported, build it yourself.
NETL2 is written in Rust, so you will need to install the Rust compiler to build it yourself.
You can find the Rust compiler [here](https://www.rust-lang.org/tools/install).

Once you have the Rust compiler installed, you can clone the repository and build NETL2 with the following commands:

```bash
    git clone
    cd NETL2
    cargo build --release
```

# Usage

To use NETL2, you can simply run the executable with the path to the file you want to run as an argument. NETL2 files have the extension `.nl`.

```bash
    ./NETL2 path/to/file.nl
```

## Scope
Each file is its own scope, and variables are global to the file. This means that you can access variables from anywhere in the file.

```rs
    v var_name = 2
    p(var_name)
```

Variables within a function are not global to the file, but are global to the function. This means that you can access variables from anywhere in the function.

```rs
    f main {
        v var_name = 2
        p(var_name)
    }
```

## Declaring variables

Variables are declared with the `v` keyword, followed by the variable name, an equals sign, and the value.

```rs
    v var_name = 2
```

## If statements

If statements are declared with the `i` keyword, followed by the condition, and then the code block.

```rs
    i var_name == 2 {
        p("Hello World!")
    }
```

## While loops

While loops are declared with the `w` keyword, followed by the condition, and then the code block.

```rs
    w var_name == 2 {
        p("Hello World!")
        var_name = 3
    }
```

## Functions

Functions are declared with the `f` keyword, followed by the function name, and then the code block.

```rs
    f main {
        p("Main function!")
    }
```

## Calling functions

Functions are called with the `!` operator.

```rs
    main!
```

## Printing

Printing is done with the `p` keyword, followed by the value to print.

```rs
    p("Hello World!")
```
