fn main() {
    let mut s: &str = "hello";
    println!("s = {s}");
    s = "world!";
    println!("s = {s}");

    let mut s2 = String::from("hello");
    s2.push_str(", world!");
    println!("{s2}");

    // Memory Allocation
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    let txt1 = String::from("hello");
    let txt2 = txt1; // Moves value from txt1 to txt2
    println!("{txt1}, world!");

    // Cloning
    let txt1 = String::from("hello");
    let txt2 = txt1.clone(); // Copies value from txt1 to txt2
    println!("{txt1}, world!");
}
