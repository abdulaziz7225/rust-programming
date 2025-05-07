fn main() {
    let score = 85;

    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B");
    } else if score >= 70 {
        println!("Grade: C");
    } else {
        println!("Grade; F");
    }

    // Using if as and Expression
    let time = 20;
    // let greeting = if time < 18 {
    //     "Good day"
    // } else {
    //     "Good evening"
    // };
    let greeting = if time < 18 {"Good day"} else {"Good evening"};
    println!("{}", greeting);
}
