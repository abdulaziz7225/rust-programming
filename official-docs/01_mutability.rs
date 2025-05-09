fn main() {
    let x = 5;
    println!("The value of x is : {x}");
    // x = 6;
    // println!("The value of x is : {x}"); // cannot assign twice to immutable variable

    let mut y = 10;
    println!("The value of y is : {y}");
    y = 734;
    println!("The value of y is : {y}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is equal to : {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let a = 5;
    let a = a + 1;
    {
        let a = a * 2;
        println!("The value of a in the inner scope is : {a}");
    }
    println!("The value of a is : {a}");

    let spaces = "     "; // &str
    let spaces = spaces.len(); // integer

    let mut spaces2 = " "; // &str
    spaces2 = spaces2.len(); // expected &str, found usize
}
