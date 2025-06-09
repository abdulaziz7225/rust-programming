# Useful commands for Rust Programming Language

## Statements and Expressions

- **Statements** are instructions that perform some action and do not return a value
- **Expressions** evaluate to a resultant value

## Traditional vs Named Placeholder Syntax

Traditional Placeholder Syntax `{}` was introduced in Rust 1.0

```rust
let apple: i32 = 50;
let banana: i32 = 100;
println!("apple = {}", apple);
println!("banana = {}", banana);
```

Named Placeholder Syntax `{apple}` or `{banana}` was introduced in Rust 1.58.0 (released in January 2022)

```rust
let apple: i32 = 50;
let banana: i32 = 100;
println!("apple = {apple}");
println!("banana = {banana}");
```

Positional Arguments

```rust
let apple: i32 = 50;
let banana: i32 = 100;
println!("I have {0} apples and {1} bananas. I can't believe that I have {0} apples", apple, banana);
```

## Keep dead code and suppress the warning

To keep the unused variants and suppress the warning, you can annotate part of your code with

```rust
#[allow(dead_code)]
```

## Type Aliases

```rust
type Kilometers = i32;

fn main() {
    let half_marathon: Kilometers = 21;
    let full_marathon: Kilometers = 42;
}
```

## Using nested paths to clean up large `use` lists

```rust
use std::cmp::Ordering;
use std::io;

// Combining two use statements that share a subpath
use std::io;
use std::io::Write;
```

Instead of the multiple lines in the example above, we can use nested paths to bring the same items into scope with one line

```rust
use std::{cmp::Ordering, io};

// Combining two use statements that share a subpath
use std::io::{self, Write};
```
