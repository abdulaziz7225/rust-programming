fn main() {
    let person = ("John", 30, true);
    println!("Name : {}", person.0);
    println!("Age : {}", person.1);
    println!("Is active : {}", person.2);

    // Unpack a Tuple
    let person2 = ("Mirodil", 25, false);
    let (name, age, active) = person2;
    println!("\nName : {name}");
    println!("Age : {age}");
    println!("Active : {active}");

    // Return a Tuple from a Function
    let user = get_user();
    println!("User : {} ({} years old)", user.0, user.1);
}

fn get_user() -> (String, i32) {
    return (String::from("Colt Steele"), 35);
}
