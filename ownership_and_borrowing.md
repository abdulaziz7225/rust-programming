# Ownership and Borrowing

## Keywords/Constructs that Take Ownership by Default

1. `match` **expressions** (for non-reference patterns)

```rust
let s = String::from("hello");

// Takes ownership
match s {
    string => println!("{}", string)
}
// s is no longer valid here

// To borrow instead:
match & s {
    string => println!("{}", string)
}
// s is still valid here
```

2. `for` **loops** (when iterating over a collection directly)

```rust
let v = vec![1, 2, 3];

// Takes ownership of v
for x in v {
    println!("{}", x);
}
// v is no longer valid here

// To borrow instead:
for x in & v {
    println!("{}", x);
}
// v is still valid here
```

3. **Function parameters** (without `&`)

```rust
fn take_ownership(s: String) {
    println!("{}", s);
}

fn borrow_only(s: &String) {
    println!("{}", s);
}

let s = String::from("hello");
take_ownership(s);     // s is moved
// s is no longer valid here

let s = String::from("hello");
borrow_only( & s);      // s is borrowed
// s is still valid here
```

## Keywords/Constructs that Capture by Reference by Default

1. **Closures** (unless `move` is used)

```rust
let s = String::from("hello");

// Captures by reference
let closure = | | println!("{}", s);
println!("Can still use s: {}", s); // s is still valid

// Takes ownership with move
let move_closure = move | | println!("{}", s);
// println!("{}", s); // Error: s was moved
```

2. `if let` and `while let` (borrow by default)

```rust
let x = Some(5);

// Borrows x
if let Some(value) = & x {
    println!("{}", value);
}
// x is still valid here
```

3. `let` **expressions** (create new bindings, don't take ownership)

```rust
let x = String::from("hello");
let y = & x;  // Reference to x
println!("{}, {}", x, y); // Both x and y are valid
```
