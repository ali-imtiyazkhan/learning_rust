fn main() {
    let s1 = String::from("hello");

    // --- References and Borrowing ---
    // The &s1 syntax lets us create a reference that refers to the value of s1 
    // but does not own it.
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // --- Mutable References ---
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("Changed string: {}", s2);

    // --- Reference Rules ---
    // 1. You can have either one mutable reference OR any number of immutable references.
    // 2. References must always be valid.

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // Error Example (Uncomment to see):
    // let r1 = &s;
    // let r2 = &mut s; // ERROR: cannot borrow `s` as mutable because it is also borrowed as immutable
    // println!("{}, {}", r1, r2);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
