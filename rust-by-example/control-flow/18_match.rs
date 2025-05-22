fn main() {
    let number = 30;

    println!("Tell me about {number}");
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime number!"),
        13..=19 => println!("A -teen number"),
        _ => println!("Ain't special"),
    }

    let boolean = true;

    let binary = match boolean {
        true => 1,
        false => 0,
    };
    println!("{boolean} -> {binary}");
}
