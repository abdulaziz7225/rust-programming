enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

fn main() {
    // Without `use` keyword
    let stage = Stage::Beginner;
    let role = Role::Student;

    match stage {
        // Note the lack of scoping because of the explicit `use` above.
        Stage::Beginner => println!("Beginners are starting their learning journey!"),
        Stage::Advanced => println!("Advanced learners are mastering their subjects..."),
    }

    match role {
        // Note again the lack of scoping.
        Role::Student => println!("Students are acquiring knowledge!"),
        Role::Teacher => println!("Teachers are spreading knowledge!"),
    }

    // Witt `use` keyword
    use crate::Stage::{Beginner, Advanced};
    use crate::Role::*;
    let stage = Advanced;
    let role = Teacher;

    match stage {
        // Note the lack of scoping because of the explicit `use` above.
        Beginner => println!("Beginners are starting their learning journey!"),
        Advanced => println!("Advanced learners are mastering their subjects..."),
    }

    match role {
        // Note again the lack of scoping.
        Student => println!("Students are acquiring knowledge!"),
        Teacher => println!("Teachers are spreading knowledge!"),
    }
}
