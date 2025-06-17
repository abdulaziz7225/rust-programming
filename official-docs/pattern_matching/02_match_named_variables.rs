fn main() {
    // Example 1
    let x = Some(5);
    // let x = None;
    let y = 10;

    match x {
        Some(50) => println!("Got 50!"),
        Some(y) => println!("y = {y}"),
        _ => println!("Default case. x = {x:?}"),
    }

    println!("x = {x:?}, y = {y:?}");

    // Match guards
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    // Example 2
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50!"),
        Some(n) if n == y => println!("n = {n}"),
        _ => println!("Default case. x = {x:?}"),
    }

    // Example 3
    let x = 5;
    let y = false;
    match x {
        4 | 5 | 6 if y == false => println!("Yes"),
        _ => println!("no"),
    }
}
