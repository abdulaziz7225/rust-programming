use division::divide;

#[test]
fn divide_two_positive_integers() {
    let result = divide(25, 10);
    assert_eq!(result, Ok(2));
}

#[test]
fn divide_negative_and_positive_integers() {
    let result = divide(42, -6);
    assert_eq!(result, Ok(-7));
}

#[test]
fn divide_two_negative_integers() {
    let result = divide(-40, -5);
    assert_eq!(result, Ok(8));
}

#[test]
fn divide_by_zero() {
    let result = divide(72, 0);
    assert_eq!(result, Err(String::from("Division by zero is not allowed")));
}
