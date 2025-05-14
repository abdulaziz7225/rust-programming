fn main() {
    // let mut s1 = String::new();

    let data = "initial contents"; // &str (string slice)
    let s2 = data.to_string();
    let s3 = "initial contents".to_string();
    let s4 = String::from("initial contents");
    println!("s2 = {s2}");
    println!("s3 = {s3}");
    println!("s4 = {s4}");

    // Updating a string
    let mut t1 = String::from("foo ");
    let t2 = "bar";
    t1.push_str(t2);
    println!("t1 = {t1}, t2 = {t2}");

    let mut t3 = String::from("lo");
    t3.push('l');
    println!("t3 = {t3}");

    // Concatenation with + operator or the format! macro
    let a1 = String::from("Hello, ");
    let a2 = String::from("world!");
    let a3 = a1 + &a2; // note a1 has been moved here and can no longer be used

    let b1 = String::from("tick");
    let b2 = String::from("tac");
    let b3 = String::from("toe");

    // let b4 = b1 + "-" + &b2 + "-" + &b3; // note b1 has been moved here and can no longer be used
    let b5 = format!("{b1}-{b2}-{b3}"); // format! macro doesn't take ownership of any of its parameters

    // Methods for Iterating over Strings
    // let hello_in_russian = "Здравствуйте";
    let hello_in_russian = String::from("Здравствуйте");
    for ch in hello_in_russian.chars() {
        println!("char = {ch}");
    }
    println!("{hello_in_russian}");
    for b in hello_in_russian.bytes() {
        println!("bytes = {b}");
    }
}
