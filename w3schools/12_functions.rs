fn main() {
    // Function initialization
    say_hello();
    greet("Maximilian");

    let sum = add(3, 4);
    println!("Sum is : {}", sum);

    let multiplication = mul(4, 6);
    println!("Multiplication is : {}", multiplication);
}

// Function declaration
fn say_hello() {
    println!("Hello from a function!");
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn mul(a: i32, b: i32) -> i32 {
    a * b
}
