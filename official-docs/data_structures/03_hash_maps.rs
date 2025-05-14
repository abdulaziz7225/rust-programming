use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name1 = String::from("Blue");
    let score1 = scores.get(&team_name1).copied().unwrap_or(0);
    let team_name2 = String::from("Green");
    let score2 = scores.get(&team_name2).copied().unwrap_or(100);
    println!("team_name1 = {team_name1}, score1 = {score1}");
    println!("team_name2 = {team_name2}, score2 = {score2}");

    for (key, value) in &scores {
        println!("key = {key}, value = {value}");
    }

    // Hash Maps and Ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    // println!("field_name = {field_name}");
    // println!("field_value = {field_value}");
    println!("map = {map:?}");

    // Updating a Hash Map
    // Overwriting a value
    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);
    println!("scores2 = {scores2:?}");
    scores2.insert(String::from("Blue"), 25);
    println!("scores2 = {scores2:?}");

    // Adding a key and value only if a key doesn't present
    scores2.entry(String::from("Yellow")).or_insert(50);
    scores2.entry(String::from("Blue")).or_insert(100);
    println!("scores2 = {scores2:?}");

    // Updating a value based on the old value
    let text = "hello world wonderful world";
    let mut text_map = HashMap::new();
    for word in text.split_whitespace() {
        let count = text_map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("text_map = {text_map:?}");
}
