fn main() {
    let mut count = 1;

    while count <= 10 {
        if count == 6 {
            break;
        }
        println!("Count: {}", count);
        count += 1;
    }

    println!();

    let mut num = 1;
    while num <= 10 {
        if num == 6 {
            num += 1;
            continue;
        }
        println!("Number : {}", num);
        num += 1;
    }
}
