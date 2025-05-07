fn main() {
    let mut fruits = vec!["apple", "banana", "orange"];
    println!("Fruits : {:?}", fruits);
    println!("First fruit : {}", fruits[0]);

    fruits[0] = "grape";
    println!("Fruits after change : {:?}", fruits);
    println!("New first fruit : {}", fruits[0]);

    fruits.pop();
    println!("Remove orange : {:?}", fruits);

    fruits.push("cherry");
    println!("Add cherry : {:?}", fruits);

    fruits.remove(1);
    println!("Remove banana from fruits : {:?}", fruits);

    fruits.insert(0, "pear");
    println!("Insert pear : {:?}", fruits);

    // Vector length and loop through a vector
    println!("\nThere are {} fruits in total", fruits.len());
    // Use &fruits to borrow the vector instead of taking ownership of it
    for fruit in &fruits {
        println!("I like {}", fruit);
    }
    println!("{:?}", fruits);
}
