fn main() {
    // Explicit type conversion or casting. There is no implicit type conversion in Rust
    let integer1: i32 = 56;
    let decimal1: f64 = integer1 as f64;
    let decimal2: f64 = 87.4324;
    let integer2: i32 = decimal2 as i32;
    println!("Casting: {0} -> {1}", integer1, decimal1);
    println!("Casting: {0} -> {1}", decimal2, integer2);

    // Parsing a String
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {sum:?}");
}
