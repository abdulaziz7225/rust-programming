fn main() {
    for i in 1..6 {
        println!("Value of i : {}", i);
    }

    println!();

    // Inclusive range
    for j in 1..=6 {
        println!("Value of j : {}", j);
    }

    println!();

    for k in 1..=10 {
        if k == 3 {
            continue; // skip 3
        }
        if k == 5 {
            break; // stop before printing 5
        }
        println!("k = {}", k);
    }
}
