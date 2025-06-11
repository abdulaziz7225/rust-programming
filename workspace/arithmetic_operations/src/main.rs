fn main() {
    let x = 10;
    let y = 5;
    
    println!("x = {x}, y = {y}");
    println!("x + y = {}", addition::add(x, y));
    println!("x - y = {}", subtraction::subtract(x, y));
    println!("x * y = {}", multiplication::multiply(x, y));
    
    match division::divide(x, y) {
        Ok(result) => println!("x / y = {result}"),
        Err(e) => println!("Error: {e}"),
    }
}
