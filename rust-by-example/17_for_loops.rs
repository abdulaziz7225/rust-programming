fn main() {
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{n}");
        }
    }

    // for loop and iterators
    /*
    into_iter, iter and iter_mut all handle the conversion of a collection into an iterator in
    different ways, by providing different views on the data within. By default the for loop
    will apply the into_iter function to the collection
    */
    // into_iter()
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello, {name}"),
        }
    }
    println!("Names : {names:?}");

    // iter()
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello, {name}"),
        }
    }
    println!("Names : {names:?}");

    // iter_mut()
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("Names : {names:?}");
}
