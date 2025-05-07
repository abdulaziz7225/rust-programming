use std::collections::HashMap;

fn main() {
    let mut capital_cities = HashMap::new();
    capital_cities.insert("England", "London");
    capital_cities.insert("Germany", "Berlin");
    capital_cities.insert("Norway", "Oslo");
    println!("{:?}", capital_cities);

    // Access values
    if let Some(city) = capital_cities.get("England") {
        println!("The capital of England is {city}");
    } else {
        println!("England is not in the map");
    }

    // Update values
    capital_cities.insert("Kazakhstan", "Nur Sultan");
    println!("{:?}", capital_cities);
    capital_cities.insert("Kazakhstan", "Astana");
    println!("{:?}", capital_cities);

    // Remove values
    capital_cities.remove("Norway");
    println!("{:?}", capital_cities);

    // Loop through a HashMap
    for (country, city) in &capital_cities {
        println!("The capital of {country} is {city}");
    }
}
