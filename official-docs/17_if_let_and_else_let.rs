enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // snip
}

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // Same as above
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    // If let and else let
    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    match coin {
        Coin::Quarter(ref state) => println!("State quarter from {state:?}"),
        _ => count += 1,
    }

    // Same as above
    let mut count = 0;
    if let Coin::Quarter(ref state) = coin {
        println!("State quarter from {state:?}");
    } else {
        count += 1;
    }
}
