fn main() {
    // --- The Option Enum ---
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    // It's included in the prelude, so you don't need to import it.

    let some_number = Some(5);
    let some_char = Some('e');

    // To have a None value, you must tell Rust what type the Option would be
    let absent_number: Option<i32> = None;

    println!("Some number: {:?}", some_number);
    println!("Absent number: {:?}", absent_number);

    // --- Why Option is better than Null ---
    // You cannot perform operations on T directly if it's wrapped in Option<T>.
    // You MUST handle the None case.

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // ERROR: cannot add `Option<i8>` to `i8`

    // --- Using Option with Functions ---
    let result = plus_one(Some(5));
    let none_result = plus_one(None);

    println!("Result: {:?}", result);
    println!("None Result: {:?}", none_result);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
