// #[allow(dead_code)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum LoginStatus {
    Success(String),
    Error(String),
}

fn main() {
    let directions = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    for direction in directions {
        match direction {
            Direction::Up => println!("Going up"),
            Direction::Down => println!("Going down"),
            Direction::Left => println!("Going left"),
            Direction::Right => println!("Going right"),
        }
    }

    let results = [
        LoginStatus::Success(String::from("Welcome, John!")),
        LoginStatus::Error(String::from("Incorrect password!")),
    ];
    for result in results {
        match result {
            LoginStatus::Success(message) => println!("Success : {message}"),
            LoginStatus::Error(message) => println!("Error : {message}"),
        }
    }
}
