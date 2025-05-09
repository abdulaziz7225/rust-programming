fn main() {
    // Boolean
    let t = true;
    let f: bool = false; // with explicit type annotation

    // Character
    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // Compund Types
    // Tuple
    let tup1 = (500, 6.4, 1, "hello", true, 'A');
    let tup2: (i32, f64, u8, &str, bool, char) = (1000, 12.32, 255, "Rust", false, 'T');

    // Tuple Unpacking
    let tup3 = (500, 6.4, 255);
    let (a, b, c) = tup3;
    println!("a = {a}, b = {b}, c = {c}");
    println!("a = {}, b = {}, c = {}", tup3.0, tup3.1, tup3.2);

    // Array
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("12 months are : {:?}", months);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5]; // [3, 3, 3, 3, 3]
    println!("a = {:?}", a);
    println!("b = {:?}", b);
}
