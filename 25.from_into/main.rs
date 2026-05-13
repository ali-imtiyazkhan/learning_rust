// --- From Trait ---
// The From trait allows a type to define how to create itself from another type.
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// --- Into Trait ---
// The Into trait is the reciprocal of the From trait. 
// If you have implemented the From trait for your type, 
// you get Into for free.

fn main() {
    // Using From
    let num = Number::from(30);
    println!("My number is {:?}", num);

    // Using Into
    let int = 5;
    let num: Number = int.into(); // We must specify the type
    println!("My number is {:?}", num);

    // Example with String
    let my_str = "hello";
    let my_string = String::from(my_str);
    let my_string_into: String = my_str.into();
}

// --- TryFrom and TryInto ---
// Used for conversions that can fail (returns a Result)
// impl TryFrom<i32> for EvenNumber { ... }
