fn main() {
    // --- Lifetimes ---
    // Lifetimes ensure that references are valid for as long as we need them to be.

    let string1 = String::from("long string is long");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // --- Lifetime in Structs ---
    let text = String::from("Example text.");
    let first_sentence = text.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Important excerpt: {:?}", i);

    // --- The Static Lifetime ---
    // 'static means the reference can live for the entire duration of the program.
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);
}

// --- Generic Lifetime Annotations in Functions ---
// This function tells Rust that the returned reference will be valid as long as 
// BOTH x and y are valid.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// --- Lifetime Annotations in Struct Definitions ---
// This struct cannot outlive the reference it holds in its 'part' field.
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}
