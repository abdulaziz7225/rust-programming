fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}");

    // Mutable Reference
    let s1 = String::from("Rust is the best");
    change1(&s1);
    
    let mut s2 = String::from("Rust is the best");
    change2(&mut s2);
    println!("{s2}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change1(some_string: &String) {
    some_string.push_str(" programming language!");
}

fn change2(some_string: &mut String) {
    some_string.push_str(" programming language!");
}