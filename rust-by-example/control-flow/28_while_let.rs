fn main() {
    // Without while let
    let mut optional = Some(0);
    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{i}`. Try again");
                    optional = Some(i + 1);
                }
            },
            _ => break,
        }
    }

    // With while let
    /*
    `if let` have additional optional `else` and `else if` clauses
    `while let` does have any of these
    */
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{i}`. Try again");
            optional = Some(i + 1);
        }
    }
}