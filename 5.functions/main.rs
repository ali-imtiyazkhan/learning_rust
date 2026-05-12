fn main() {
    println!("Hello, world!");

    // Calling a function with an argument
    another_function(5);

    // Calling a function with multiple arguments
    print_labeled_measurement(5, 'h');

    // Functions with return values
    let x = five();
    println!("The value of x is: {}", x);

    let sum = add(10, 20);
    println!("The sum is: {}", sum);
}

// Function with one parameter
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

// Function with multiple parameters
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

// Function that returns a value (implicit return)
fn five() -> i32 {
    5 // Note: No semicolon here
}

// Function that returns a value (explicit parameters)
fn add(a: i32, b: i32) -> i32 {
    a + b // This is an expression
}

// Function using explicit return keyword
fn add_with_return(a: i32, b: i32) -> i32 {
    return a + b; // You can use return, but it's not idiomatic for the last expression
}
