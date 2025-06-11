# Publishing a Crate to Crates.io

- `cargo doc` - builds the HTML for the current crate’s documentation (as well as the documentation for all of the crate’s dependencies)
- `cargo doc --open` - builds the HTML for the current crate’s documentation and open the result in a web browser
- `cargo test` - runs the code examples in the documentation as tests
- `cargo publish` - publishes the crate to crates.io
- `cargo yank --vers 1.0.1` - yanks a version of the crate. Yanking a version prevents new projects from depending on that version while allowing all existing projects that depend on it to continue
- `cargo yank --vers 1.0.1 --undo` - undoes a yank and allow projects to start depending on a version again

## Making Useful Documentation Comments

Documentation comments use three slashes, `///`, and support Markdown notation for formatting the text

Filename: `src/lib.rs`

````rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
````

## Commenting Contained Items

The style of doc comment `//!` adds documentation to the item that contains the comments. We typically use these doc comments inside the crate root file (`src/lib.rs` by convention) or inside a module to document the crate or the module as a whole.

Filename: `src/lib.rs`

```rust
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--
```
