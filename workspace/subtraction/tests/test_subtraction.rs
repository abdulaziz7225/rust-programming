use subtraction::subtract;

#[test]
fn subtract_two_positive_integers() {
    let result = subtract(10, 25);
    assert_eq!(result, -15);
}

#[test]
fn subtract_two_negative_integers() {
    let result = subtract(-32, -47);
    assert_eq!(result, 15);
}
