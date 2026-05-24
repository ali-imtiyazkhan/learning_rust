fn main() {

    // 1. Some and None
    let a: Option<i32> = Some(10);
    let b: Option<i32> = None;

    // 2. match
    match a {
        Some(val) => println!("Value: {}", val),
        None      => println!("Nothing!"),
    }

    // 3. if let
    if let Some(val) = a {
        println!("Got: {}", val);
    }

    // 4. unwrap_or — give default if None
    println!("{}", b.unwrap_or(0)); // prints 0

    // 5. is_some / is_none — check without extracting
    println!("{}", a.is_some()); // true
    println!("{}", b.is_none()); // true
}