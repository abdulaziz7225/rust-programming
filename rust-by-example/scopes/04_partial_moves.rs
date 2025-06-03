#[derive(Debug)]
struct Person {
    name: String,
    age: Box<u8>,
}

// Error! cannot move out of a type which implements the `Drop` trait
impl Drop for Person {
    fn drop(&mut self) {
        println!("Dropping the person struct: {:?}", self);
    }
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person;

    println!("The person's age is {age}");
    println!("The person's name is {name}");

    println!("The person struct is {person:?}");
    println!("The person's age from person struct is {}", person.age);
}
