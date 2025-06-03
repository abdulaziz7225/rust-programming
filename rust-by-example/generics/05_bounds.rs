use std::fmt::Debug;
// use std::fmt::Display;

// Define a function "printer" that takes a generic type "T" which must implement trait "Display"
// fn printer<T: Display>(t: T) {
//     println!("{}", t);
// }

// struct S<T: Display>(T);

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

impl HasArea for Triangle {
    fn area(&self) -> f64 {
        self.length * self.height / 2.0
    }
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

// #[allow(dead_code)]
#[derive(Debug)]
struct Triangle {
    length: f64,
    height: f64,
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:#?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn main() {
    // let a = S(vec![1]);

    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    print_debug(&_triangle);
    println!("Area: {}", area(&_triangle));
}
