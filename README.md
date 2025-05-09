# Useful commands for Rust Prorgramming Language

## How to compile, check and run a Rust code

- `cargo build`- to compile
- `cargo run`- to run compiled binary
- `cargo check`- to check your code to make sure it compiles but doesn’t produce an executable

## Statements and Expressions

- **Statements** are instructions that perform some action and do not return a value
- **Expressions** evaluate to a resultant value

## Traditional vs Named Placeholder Syntax

Traditional Placeholder Syntax `{}` was introduced in Rust 1.0

```rust
let a = String::from("Hello");
let b = &a;
println!("a = {}", a);
println!("b = {}", b);
```

Named Placeholder Syntax `{a}` or `{b}` was introduced in Rust 1.58.0 (released in January 2022)

```rust
let a = String::from("Hello");
let b = &a;
println!("a = {a}");
println!("b = {b}");
```

## Keep dead code and suppress the warning

To keep the unused variants and suppress the warning, you can annotate part of your code with

```rust
#[allow(dead_code)]
```
