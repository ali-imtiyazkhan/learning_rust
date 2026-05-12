fn main() {
    // --- Ownership Rules ---
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    { // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward
        // do stuff with s
    } // this scope is now over, and s is no longer valid

    // --- String Type ---
    // String is stored on the heap and can be mutated
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); 

    // --- Variables and Data Interacting with Move ---
    let s1 = String::from("hello");
    let s2 = s1; // s1 is MOVED to s2. s1 is no longer valid!
    
    // println!("{}, world!", s1); // This would cause a compile-time error!
    println!("{}, world!", s2);

    // --- Variables and Data Interacting with Clone ---
    let s3 = String::from("hello");
    let s4 = s3.clone(); // Deep copy of the heap data

    println!("s3 = {}, s4 = {}", s3, s4); // Both are valid

    // --- Ownership and Functions ---
    let s5 = String::from("hello"); // s5 comes into scope
    takes_ownership(s5); // s5's value moves into the function...
                         // ... and so is no longer valid here

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it’s okay to still
                   // use x afterward

} // Here, x goes out of scope, then s5. But because s5's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
