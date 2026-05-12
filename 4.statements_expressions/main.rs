fn main() {
    // --- Statements ---
    // Statements are instructions that perform some action and do not return a value.
    // let x = 6; is a statement.
    let x = 6;

    // --- Expressions ---
    // Expressions evaluate to a value.
    // 5 + 6 is an expression that evaluates to 11.
    let y = {
        let x = 3;
        x + 1 // Note: No semicolon here. This makes it an expression.
    };

    println!("The value of y is: {}", y);

    // Expressions can be part of statements.
    // The calling of a function is an expression. 
    // The calling of a macro is an expression. 
    // A new scope block created with curly brackets is an expression.

    let z = add_one(5);
    println!("The value of z is: {}", z);
}

fn add_one(x: i32) -> i32 {
    x + 1 // This is an expression being returned
}
