fn main() {
    // Basic Ownership Example
    let a = String::from("Hello");
    let b = a;
    println!("a = {}", a); // Error: 'a' no longer owns the value
    println!("b = {}", b); // Ok: b now owns the value
    
    /* 
    When we assign a to b, the ownership moves. This means only b can use the value now, because a is no longer valid. 
    But simple types like numbers, characters, and booleans are copied, not moved. This means you can still use the 
    original variable after assigning it to another
    */
    let c = 10;
    let d = c;
    println!("c = {}", c); // Works
    println!("d = {}", d); // Works


    // Clone
    let x = String::from("Rust is a type and memory safe programming language");
    let y = x.clone();
    println!("x = {}", x); // Works
    println!("y = {}", y); // Works
}
