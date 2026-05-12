fn main() {
    // --- Booleans ---
    let t = true;
    let f: bool = false; // with explicit type annotation

    if t {
        println!("This is true!");
    }

    // --- Characters ---
    // char type is four bytes in size and represents a Unicode Scalar Value
    let c = 'z';
    let z: char = 'ℤ'; // unicode
    let heart_eyed_cat = '😻';

    println!("Char 1: {}, Char 2: {}, Emoji: {}", c, z, heart_eyed_cat);

    // --- Unit Type ---
    // The unit type is a tuple with no elements: ()
    // It is returned by expressions that don't otherwise return a value
    let unit: () = ();
    
    println!("The unit value is: {:?}", unit);

    // Functions that don't return anything explicitly actually return ()
    let result = explicit_unit();
    println!("Function returned: {:?}", result);
}

fn explicit_unit() -> () {
    println!("This function returns the unit type.");
}
