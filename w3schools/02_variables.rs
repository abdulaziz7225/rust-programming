fn main() {
    let name = "John";
    let age = 30;
    println!("{} is {} years old!", name, age);
    println!("{} is {} years old!", age, name);

    let mut x = 5;
    println!("Before : {}", x);
    x = 10;
    println!("After : {}", x);
}
