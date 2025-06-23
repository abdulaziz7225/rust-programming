fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Received: {val}");
    }

    // The values we get from the calls to next are immutable references to the values
    // in the vector. The iter method produces an iterator over immutable references.
    // If we want to create an iterator that takes ownership of v1 and returns owned
    // values, we can call into_iter instead of iter. Similarly, if we want to iterate
    // over mutable references, we can call iter_mut instead of iter.
    let v1 = vec![1, 2, 3];
    let mut v1_iter2 = v1.iter();

    assert_eq!(v1_iter2.next(), Some(&1));
    assert_eq!(v1_iter2.next(), Some(&2));
    assert_eq!(v1_iter2.next(), Some(&3));
    assert_eq!(v1_iter2.next(), None);

    // Methods That Consume the Iterator
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);

    // Methods that Produce Other Iterators
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}
