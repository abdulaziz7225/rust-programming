pub fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        Err(String::from("Division by zero is not allowed"))
    } else {
        Ok(x / y)
    }
}

