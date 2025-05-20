fn main() {
    let shadowed_binding: i32 = 1;
    {
        println!("Before being shadowed : {shadowed_binding}");

        let shadowed_binding: &str = "abc";
        println!("Shadowed in inner block : {shadowed_binding}");
    }
    println!("Outside inner block : {shadowed_binding}");

    let shadowed_binding: f64 = 4342.43;
    println!("Shadowed in outer block : {shadowed_binding}");

    // Freezing
    let mut _mutable_integer: i32 = 100;
    println!("Mutable integer : {_mutable_integer}");
    {
        let _mutable_integer: i32 = 50;
        println!("Mutable integer : {_mutable_integer}");

        // _mutable_integer = 10;
    }
    _mutable_integer += 75;
    println!("Mutable integer : {_mutable_integer}");
}
