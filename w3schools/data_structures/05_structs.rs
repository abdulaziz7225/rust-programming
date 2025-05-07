fn main() {
    // Create a Struct called Person
    struct Person {
        name: String,
        age: u32,
        can_vote: bool,
    }

    // Create a Person object
    let user = Person {
        name: String::from("Davide"),
        age: 23,
        can_vote: true,
    };

    // Access and print the values
    println!("Name : {}", user.name);
    println!("Age : {}", user.age);
    println!("Can vote? {}", user.can_vote);

    // Change a field
    struct Bird {
        name: String,
        can_fly: bool,
    }

    let mut bird = Bird {
        name: String::from("sparrow"),
        can_fly: true,
    };

    println!("\nName : {}", bird.name);
    println!("Can fly? {}", bird.can_fly);
    bird.name = String::from("ostrich");
    bird.can_fly = false;
    println!("\nName : {}", bird.name);
    println!("Can fly? {}", bird.can_fly);
}
