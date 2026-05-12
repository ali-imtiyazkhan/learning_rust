fn main() {
    // --- String (Owned) ---
    // A String is stored on the heap and can be modified.
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("Owned String: {}", s);

    // --- &str (String Slice / Reference) ---
    // A string literal is a string slice (&str) that is hardcoded into the binary.
    let literal: &str = "hello literal";
    println!("String Literal: {}", literal);

    // --- Slicing a String ---
    let s_slice: &str = &s[0..5]; // Borrowing a part of the String
    println!("Slice of String: {}", s_slice);

    // --- Conversions ---
    
    // 1. &str to String
    let from_str = "convert me".to_string();
    let from_str_2 = String::from("convert me too");

    // 2. String to &str
    let string_obj = String::from("hello");
    let back_to_str: &str = &string_obj; // Deref coercion

    println!("Conversions: {}, {}, {}", from_str, from_str_2, back_to_str);

    // --- Functions and Strings ---
    let my_string = String::from("hello world");

    // It's better to pass &str to functions if you don't need ownership
    print_str(&my_string); // Works with &String (coerced to &str)
    print_str("a literal"); // Works with literal
}

fn print_str(s: &str) {
    println!("Printing from function: {}", s);
}
