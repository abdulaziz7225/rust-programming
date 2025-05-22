fn main() {
    let triple = (0, -2, 6);
    println!("Tell me about {triple:?}");
    match triple {
        (0, y, z) => println!("First is `0`, `y` is {y:?}, `z` is {z:?}"),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        (.., 2) => println!("Last is `2` and the rest doesn't matter"),
        (3, .., 4) => println!("First is `3`, last is `4`, and the rest doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }
}
