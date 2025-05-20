use std::convert::{From, Into};

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

/*
From and Into are designed to be complementary. We do not need to provide an
implementation for both traits. If you have implemented the From trait for your type,
Into will call it when necessary. Note, however, that the converse is not true:
implementing Into for your type will not automatically provide it with an
implementation of From.
*/

// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number { value: self }
//     }
// }

fn main() {
    let num = Number::from(30);
    println!("My num is : {num:#?}");
    
    let integer = 5;
    let number: Number = integer.into();
    println!("My number is : {number:#?}");
}
