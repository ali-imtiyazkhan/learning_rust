// --- Lifetime Elision Rules ---
// Rust has three rules to figure out lifetimes when they aren't explicit.
// These are called "elision rules".

// 1. Each parameter that is a reference gets its own lifetime parameter.
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self, 
//    the lifetime of self is assigned to all output lifetime parameters.

// Rule 1 & 2 applied here:
// Explicit: fn first_word<'a>(s: &'a str) -> &'a str { ... }
// Elided:
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // Rule 3 applied here:
    // The return type is given the lifetime of &self automatically.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let my_string = String::from("hello world");
    let word = first_word(&my_string);
    println!("First word: {}", word);

    let excerpt = ImportantExcerpt {
        part: "Some text",
    };
    let part = excerpt.announce_and_return_part("Starting...");
    println!("Part: {}", part);
}
