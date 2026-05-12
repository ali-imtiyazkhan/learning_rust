fn main() {
    // --- String Slices ---
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("Slices: {} and {}", hello, world);

    // Alternative range syntaxes
    let slice1 = &s[..5];  // starting from index 0
    let slice2 = &s[6..];  // until the end
    let slice3 = &s[..];   // entire string

    println!("Full slice: {}", slice3);

    // --- Array Slices ---
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
    println!("Array slice: {:?}", slice);

    // --- Slices as Function Parameters ---
    let my_string = String::from("hello world");

    // first_word works on slices of Strings, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // first_word also works on references to Strings, which are equivalent
    // to whole slices of Strings
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    
    println!("First word is: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
