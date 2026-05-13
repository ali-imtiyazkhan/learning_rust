fn main() {
    // --- Creating a New Vector ---
    let v: Vec<i32> = Vec::new();
    
    // Using the vec! macro
    let v = vec![1, 2, 3];

    // --- Updating a Vector ---
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // --- Reading Elements ---
    let third: &i32 = &v[2]; // crashes if index out of bounds
    println!("The third element is {}", third);

    match v.get(2) { // returns Option<&T>
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // --- Iterating Over Values ---
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // dereference to change the value
    }

    // --- Using Enums to Store Multiple Types ---
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
