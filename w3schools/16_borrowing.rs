fn main() {
    let a = String::from("Hello");
    let b = &a;
    println!("a = {a}");
    println!("b = {b}");

    // Mutable Reference
    let mut name = String::from("John");
    let name_ref = &mut name;
    name_ref.push_str(" Doe");
    println!("{name_ref}"); // John Doe
}
