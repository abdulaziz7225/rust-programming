fn main() {
    const BIRTH_YEAR: i32 = 1980; // OK
    // const BIRTH_YEAR = 1999; // Error: missing type
    const MINUTES_PER_HOUR: i32 = 60;
    const PI: f64 = 3.14;
    println!("There are {} minuted in one hour", MINUTES_PER_HOUR);
    println!("Value of pi is equal to : {}", PI);
}