# Attributes

Attributes look like `#[outer_attribute]` or `#![inner_attribute]`, with the difference between them being where they apply.

- `#[outer_attribute]` applies to the item immediately following it. Some examples of items are: a function, a module declaration, a constant, a structure, an enum. Here is an example where attribute `#[derive(Debug)]` applies to the `struct Rectangle`:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

- `#![inner_attribute]` applies to the enclosing item (typically a module or a crate). In other words, this attribute is interpreted as applying to the entire scope in which it's placed. Here is an example where `#![allow(unused_variables)]` applies to the whole crate (if placed in main.rs):

```rust
#![allow(unused_variables)]

fn main() {
    let x = 3; // This would normally warn about an unused variable.
}
```

Attributes can take arguments with different syntaxes:

- `#[attribute = "value"]`
- `#[attribute(key = "value")]`
- `#[attribute(value)]`

Attributes can have multiple values and can be separated over multiple lines, too:

```rust
#[attribute(value, value2)]

#[attribute(value, value2, value3,
            value4, value5)]
```
