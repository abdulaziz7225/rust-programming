fn main() {
    let mut count = 0;
    loop {
        if count == 3 {
            break;
        }
        println!("Hello world!");
        count += 1;
    }

    let mut count2 = 0;
    let result = loop {
        if count2 == 5 {
            break count2;
        }
        println!("Hello");
        count2 += 1;
    };
    println!("The loop stopped at: {}", result);
}
