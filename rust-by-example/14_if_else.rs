fn main() {
    let n = 11;
    if n < 0 {
        println!("{n} is negative");
    } else if n > 0 {
        println!("{n} is positive");
    } else {
        println!("{n} is zero");
    }

    let big_n = if n > -10 && n < 10 {
        println!("{n} is a small number. Increase it ten-fold");
        10 * n
    } else {
        println!("{n} is a big number. Halve the number");
        n / 2
    };
    println!("{n} -> {big_n}");
}
