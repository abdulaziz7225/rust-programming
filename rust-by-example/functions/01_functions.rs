fn main() {
    fizzbuzz_n(100);
}

fn is_divisible(dividend: u32, divisor: u32) -> bool {
    if divisor == 0 {
        return false;
    }
    dividend % divisor == 0
}

fn fizzbuzz(n: u32) {
    if is_divisible(n, 15) {
        println!("FizzBuzz");
    } else if is_divisible(n, 3) {
        println!("Fizz");
    } else if is_divisible(n, 5) {
        println!("Buzz");
    } else {
        println!("{}", n);
    }
}

fn fizzbuzz_n(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}
