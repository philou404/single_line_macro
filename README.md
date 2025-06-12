# single_line_macro

A small procedural macro that lets you define one-line functions and methods using `=> expr` syntax.

## Features

- Define methods with `&self` or `&mut self`, with any number of parameters.
- Define associated functions and free functions, with or without the `fn` keyword.
- Support for doc comments (`///`) — they will appear in generated documentation.
- Automatically expands a single-field expression `x` into `self.x` within methods.
- Zero-dependency (besides `syn` and `quote`).

## Installation

Add this to your crate’s `Cargo.toml`:

```toml
[dependencies]
single_line_macro = "0.2.1"
```

And in your code:
```rust
use single_line_macro::single_line;
```

Or rename it:

```rust
use single_line_macro::single_line as sl;
```

```rust
use single_line_macro::single_line;

struct Foo { x: i32 }

impl Foo {
    single_line![/// Creates a new `Foo` with the given `x`.
        pub new(x: i32) -> Self => Self { x }];
    
    single_line![/// Returns the current `x`.
        pub fn get_x(&self) -> i32 => self.x];

    
    single_line![/// Multiplies `x` by `m`.
        pub fn mult(&self, m: i32) -> i32 => self.x * m];

    
    single_line![/// Resets `x` to zero.
        pub fn reset(&mut self) -> () => { self.x = 0; }];
}

// Free functions:
single_line![pub answer -> i32 => 42];
single_line![greet(name: &str) -> String => format!("Hello, {}", name)];
```

*more examples in the [test folder](https://github.com/philou404/single_line_macro/blob/master/tests/single_line_tests.rs)*