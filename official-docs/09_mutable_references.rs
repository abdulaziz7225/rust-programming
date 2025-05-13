fn main() {
    let mut s = String::from("Hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{r1}, {r2}");

    // Scoping helps by preventing simultaneous
    {
        let r1 = &mut s;
        r1.push_str(" world,");
        println!("{r1}");
    }
    let r2 = &mut s;
    r2.push_str(" from Rust!");
    println!("{r2}");

    // At any given time, you can have either one mutable reference or any number of immutable references
    let r1 = &s; // No problem
    let r2 = &s; // No problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{r1}, {r2}, and {r3}");
}
