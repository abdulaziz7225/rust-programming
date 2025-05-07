fn main() {
    let day = 6;
    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day"),
    }

    let another_day = 3;
    match another_day {
        1 | 2 | 3 | 4 | 5 => println!("Weekday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day"),
    }

    let other_day = 4;
    let result = match other_day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid day",
    };
    println!("{}", result);

    // Expression-based
    let number = 10;
    match number {
        1..=5 => println!("Small number"),
        6..=10 => println!("Medium number"),
        _ => println!("Other"),    
    }
}
