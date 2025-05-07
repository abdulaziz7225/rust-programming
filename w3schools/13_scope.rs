fn main() {
    // Variable inside a function
    my_function();
    println!("{}", message); // Error - you cannot access the message variable outside of the function

    // Variable inside a block
    let score = 80;
    if score > 50 {
        let result = "Pass";
        println!("Result : {}", result);
    }
    println!("Result : {}", result); // Error: result is out of scope here

    // Variables in the same scope
    let x = 5;
    let x = 10; // Error: name already used in this scope
}

fn my_function() {
    let message = "Hello, world!";
    println!("{}", message);
}
