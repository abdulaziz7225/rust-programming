fn main() {
    let _x: f64 = 1.23;
    let _y = 5i32;

    // Tuples
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("Tuple of tuples : {tuple_of_tuples:?}");
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("Too long tuple : {too_long_tuple:?}");

    // Arrays
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];
    println!("Value of xs : {xs:?}");
    // println!("Value of ys : {ys:?}");
    println!("Number of elements in ys : {}", ys.len());

    // Example of empty slice `&[]`
    let empty_array: [u32; 1] = [56];
    assert_eq!(&empty_array, &[]);
    // assert_eq!(&empty_array, &[][..]); // Same but more verbose
}
