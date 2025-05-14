fn main() {
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3, 4];

    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    // Reading elements of vectors
    let third2: &i32 = &v2[2];
    println!("The third element of v2 : {third2}");

    let third3: Option<&i32> = v3.get(2);
    match third3 {
        Some(third3) => println!("The third element of v3 : {third3}"),
        None => println!("There is no third element"),
    }

    // let does_not_exist = &v1[100];
    let does_not_exist = v1.get(100);
    println!("{does_not_exist:?}");

    // let mut v4 = vec![1, 2, 3, 4, 5];
    // let first = &v4[0]; // immutable borrow occurs here
    // v4.push(6); // mutable borrow occurs here
    // println!("The first element of v4 : {first}"); // immutable borrow later used here

    // Iterating over values in a vector
    let mut v5 = vec![100, 32, 57];
    for i in &mut v5 {
        *i += 50;
    }
    for i in &v5 {
        println!("{i}");
    }

    // Using an enum to store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
