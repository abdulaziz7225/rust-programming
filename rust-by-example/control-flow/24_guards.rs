#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn main() {
    let temperature = Temperature::Fahrenheit(35);
    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{t}C is above 30 Celsius"),
        Temperature::Celsius(t) => println!("{t}C is equal to or below 30 Celsius"),
        Temperature::Fahrenheit(t) if t > 86 => println!("{t}F is above 86 Fahrenheit"),
        Temperature::Fahrenheit(t) => println!("{t}F is equal to and below 86 Fahrenheit"),
    }

    // Note that the compiler won't take guard conditions into account when checking if all patterns are covered by the match expression
    let number: u8 = 4;
    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => println!("Should never happen"),
    }
}
