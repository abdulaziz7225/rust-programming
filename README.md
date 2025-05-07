# Useful commands for Rust Prorgramming Language

## How to compile, check and run a Rust code

- `cargo build`- to compile
- `cargo run`- to run compiled binary
- `cargo check`- to check your code to make sure it compiles but doesn’t produce an executable

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
