fn main() {
    // Positional Arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Named Arguments
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "The quick brown fox",
        verb = "jumps over"
    );
    
    // Different formatting can be invoked by specifying the format character
    println!("Base 10: {}", 69240);
    println!("Base 2 (binary): {:b}", 69240);
    println!("Base 8 (octal): {:o}", 69240);
    println!("Base 16 (hexadecimal): {:x}", 69240);
}
