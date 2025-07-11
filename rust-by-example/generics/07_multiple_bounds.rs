use std::fmt::{Debug, Display};

// Multiple bounds for a single type can be applied with "+"
fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

// Different bounds for different types are separated with ","
fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}`", t);
    println!("u: `{:?}`", u);
}

fn main() {
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    // compare_prints(&array);

    compare_types(&array, &vec);
}
