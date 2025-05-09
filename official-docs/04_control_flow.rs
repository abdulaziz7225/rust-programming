fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else {
        println!("Number is divisible nor by 4 or 3");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" }; // expected integer, found `&str`
    println!("The value of number is : {number}");

    // Returning Values from Loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}\n");

    // Loop Labels to Disambiguate between Multiple Loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}\n");

    // Conditional Loops with while
    let mut number = 3;
    while number != 0 {
        println!("number = {number}");
        number -= 1;
    }

    println!();

    // for loops
    let elements = [10, 20, 30, 40, 50];
    for element in elements {
        println!("value = {element}");
    }

    println!();

    // 1, 2, 3
    for number in 1..4 {
        println!("number = {number}");
    }
    // 3, 2, 1
    for number in (1..4).rev() {
        println!("number = {number}");
    }
    // 4, 3, 2, 1
    for number in (1..=4).rev() {
        println!("number = {number}");
    }
}
