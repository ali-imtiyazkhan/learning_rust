fn main() {
    // 1. Searching and Checking
    let s = "The quick brown fox jumps over the lazy dog";
    println!("Contains 'fox': {}", s.contains("fox"));
    println!("Starts with 'The': {}", s.contains("The"));
    println!("Ends with 'dog': {}", s.ends_with("dog"));

    // 2. Replacing
    let new_s = s.replace("lazy", "energetic");
    println!("Replaced: {}", new_s);

    // 3. Trimming
    let spaced = "   hello world   ";
    println!("Trimmed: '{}'", spaced.trim());

    // 4. Splitting
    println!("Words in sentence:");
    for word in s.split_whitespace() {
        println!(" - {}", word);
    }

    let csv = "apple,banana,cherry";
    let fruits: Vec<&str> = csv.split(',').collect();
    println!("Fruits from CSV: {:?}", fruits);

    // 5. Case Conversion
    let name = "Rustacean";
    println!("Upper: {}, Lower: {}", name.to_uppercase(), name.to_lowercase());

    // 6. Mutating (Inserting and Removing)
    let mut buf = String::from("hello");
    buf.insert(5, '!');         // Insert char at index
    buf.insert_str(0, "Say: "); // Insert string at index
    println!("After insertions: {}", buf);

    buf.remove(0); // Removes 'S'
    println!("After remove: {}", buf);

    buf.truncate(5); // Keep only first 5 chars
    println!("After truncate: {}", buf);

    // 7. Parsing (String to Other Types)
    let number_str = "42";
    let number: i32 = number_str.parse().expect("Not a number");
    println!("Parsed number: {}", number);

    let bool_str = "true";
    let boolean: bool = bool_str.parse().unwrap();
    println!("Parsed boolean: {}", boolean);
}
