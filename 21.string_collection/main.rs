fn main() {
    // --- Creating a New String ---
    let mut s = String::new();
    
    let data = "initial contents";
    let s = data.to_string();
    let s = String::from("initial contents");

    // --- Updating a String ---
    let mut s = String::from("foo");
    s.push_str("bar"); // appends a string slice
    s.push('l');       // appends a single character
    println!("{}", s);

    // --- Concatenation ---
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 is moved here and can no longer be used
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3); // format! doesn't take ownership
    println!("{}", s);

    // --- Indexing into Strings ---
    // Rust strings don't support indexing like s[0] because of UTF-8 encoding.
    let hello = "Здравствуйте";
    // let answer = &hello[0]; // ERROR

    // --- Slicing Strings ---
    let s = &hello[0..4]; // Be careful with boundaries (must be on char boundaries)
    println!("{}", s);

    // --- Iterating Over Strings ---
    // 1. By characters
    for c in "Зд".chars() {
        println!("{}", c);
    }

    // 2. By bytes
    for b in "Зд".bytes() {
        println!("{}", b);
    }
}
