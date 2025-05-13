struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_contract: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_contract: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    // Struct Update Syntax
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_contract: user1.sign_in_contract,
    };

    // Same as above
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // Tuple Structs without Named Fields
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit-like Structs without any fields
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}

// Functions can return an instance of Struct
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_contract: 1,
    }
}
// Same as above
fn build_user2(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_contract: 1,
    }
}
