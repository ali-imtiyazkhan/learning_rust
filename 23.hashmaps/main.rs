use std::collections::HashMap;

fn main() {
    // --- Creating a New HashMap ---
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // --- Accessing Values ---
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{}: {}", team_name, score);

    // --- Iterating Over Key-Value Pairs ---
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // --- Updating a HashMap ---

    // 1. Overwriting a Value
    scores.insert(String::from("Blue"), 25);

    // 2. Only Inserting a Value If the Key Has No Value
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);

    // 3. Updating a Value Based on the Old Value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
