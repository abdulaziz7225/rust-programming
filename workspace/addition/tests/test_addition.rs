use addition::add;

#[test]
fn add_two_positive_integers() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}

#[test]
fn add_two_negative_integers() {
    let result = add(-8, 5);
    assert_eq!(result, -3);
}
