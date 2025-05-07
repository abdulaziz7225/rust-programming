fn main() {
    /*
    &str - is called "string slices" and is used for fixed text like "Hello"
    String - is used when you need a string that can change
    */
    let greeting: &str = "Hello";
    println!("{}", greeting);

    // To create a 'String' from a string literal, use the 'to_string()' method of the 'String::from()' function
    let text1 = "Hello world!".to_string();
    println!("{}", text1);

    let text2 = String::from("Welcome to Rust Programming Lanugage!");
    println!("{}", text2);

    println!();

    // Change a String
    let mut greeting2 = String::from("Hello");

    // Use push_str() to add text to a string
    greeting2.push_str(" World");
    println!("{}", greeting2); // Hello World

    // Use push() to add one character
    greeting2.push('!');
    println!("{}", greeting2); // Hello World!

    println!();

    // Concatenate Strings
    let s1 = String::from("Hello");
    let s2 = String::from("World!");
    let s3 = String::from("What a beautiful day!");

    let result = format!("{} {} {}", s1, s2, s3);
    println!("{}", result);

    /*
    You can only add a &str to a String with '+' symbol. That's why all the remaining strings
    except for the first one should be converted to a string slice with '&' symbol. That's why
    &s2 and &s3 (a string slice) is used here
    */
    let result2 = s1 + " " + &s2 + " " + &s3;
    println!("{}", result2);

    println!();

    // String Length
    let name = String::from("John");
    println!("Length: {}", name.len()) // 4
}
