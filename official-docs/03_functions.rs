fn main() {
    println!("Hello world!");

    another_function();
    print_labeled_measurement(5, 'h');

    println!("{}", plus_one(10));
    println!("{}", plus_two(10));
}

fn another_function() {
    println!("Another function invocation!");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is : {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn plus_two(x: i32) -> i32 {
    x + 2; // implicitly returns `()` as its body has no tail or `return` expression
}
