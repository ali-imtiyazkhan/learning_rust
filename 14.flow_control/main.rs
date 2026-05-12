fn main() {
    // --- IF Expressions ---
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    // --- Using IF in a LET statement ---
    let condition = true;
    let number = if condition { 5 } else { 6 }; // types must match
    println!("The value of number is: {}", number);

    // --- Loops ---

    // 1. loop (infinite until break)
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            println!("Loop count reached 3, breaking...");
            break;
        }
    }

    // 2. while (conditional loop)
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // 3. for (loop through a collection)
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is: {}", element);
    }

    // 4. for with range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF AGAIN!!!");

    // --- Loop Labels (for nested loops) ---
    'outer: loop {
        println!("Outer loop");
        loop {
            println!("Inner loop");
            break 'outer; // break the outer loop
        }
    }
}
