fn main() {
    // --- Tuple Partial Move Example ---

    let t: (String, String) = (String::from("hello"), String::from("world"));
     
    // Ownership of the first element is moved from 't' to '_s'
    let _s: String = t.0;
    
    // We can still access the second element because it hasn't been moved
    println!("Accessed t.1: {:?}", t.1);

    // However, we CANNOT print the whole tuple 't' because it's partially moved
    // println!("{:?}", t); // This would cause a compile error

    // --- Solution: Borrow instead of Move ---
    let t2: (String, String) = (String::from("hello"), String::from("world"));
    
    // By using a reference (&), we borrow the value instead of taking ownership
    let _s2: &String = &t2.0;
    
    // Now t2 remains fully valid!
    println!("Whole tuple t2 is still valid: {:?}", t2);
}
