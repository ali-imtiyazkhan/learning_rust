fn main() {
    // --- Tuples ---
    // A tuple is a general way of grouping together a number of values with 
    // a variety of types into one compound type.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // --- Destructuring ---
    // We can use pattern matching to destructure a tuple value.
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    // --- Accessing by Index ---
    // We can also access a tuple element directly by using a period (.) 
    // followed by the index of the value we want to access.
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("Tuple elements: {}, {}, {}", five_hundred, six_point_four, one);

    // --- Empty Tuple (Unit Type) ---
    let _empty: () = ();

    // --- Returning multiple values from a function ---
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // Return a tuple
}
