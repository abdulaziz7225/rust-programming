use multiplication::multiply;

#[test]
fn multiply_two_positive_integers() {
    let result = multiply(3, 7);
    assert_eq!(result, 21);
}

#[test]
fn multiply_negative_and_positive_integers() {
    let result = multiply(5, -6);
    assert_eq!(result, -30);
}

#[test]
fn multiply_two_negative_integers() {
    let result = multiply(-2, -4);
    assert_eq!(result, 8);
}
