// An attribute to hide warnings for unused code.
#![allow(dead_code)]

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Type Alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn main() {
    let _x = Operations::Add;
}
