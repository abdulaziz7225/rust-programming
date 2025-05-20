fn main() {
    let mut count = 0u32;
    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("Three");
            continue;
        }

        println!("{count}");

        if count == 5 {
            println!("Ok, that's enough");
            break;
        }
    }

    // Nesting and Labels
    let mut i: i32 = 0;
    let mut j: i32 = 0;

    'outer: loop {
        'inner: loop {
            if j == 5 {
                break 'inner;
                // break;
            }

            if i == 4 {
                break 'outer;
            }

            println!("i = {i}, j = {j}");
            j += 1;
        }
        i += 1;
        j = 0;
    }

    // Returning from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);
}
