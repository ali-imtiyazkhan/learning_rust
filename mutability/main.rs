fn main() {
    println!("1) Basic mutability:");
    let mut a: i32 = 1;
    println!("  before: {}", a);
    a = 2;
    println!("  after:  {}", a);

    println!("\n2) Shadowing (immutable then shadow):");
    let x = 5;
    let x = x + 1; // shadowing creates a new variable with same name
    println!("  x shadowed: {}", x);

    println!("\n3) Mutable references:");
    let mut v = vec![1, 2, 3];
    add_element(&mut v);
    println!("  v after add_element: {:?}", v);

    println!("\n4) Borrowing rules (immutable borrows must end before mutable borrows):");
    let mut s = String::from("hello");
    {
        let r1 = &s; // immutable borrow
        println!("  immutable borrow r1: {}", r1);
    } // r1 goes out of scope here
    {
        let r2 = &mut s; // now mutable borrow allowed
        r2.push_str(" world");
        println!("  s after mutable borrow r2: {}", r2);
    }

    println!("\n5) Interior mutability with RefCell:");
    use std::cell::RefCell;
    let data = RefCell::new(5);
    *data.borrow_mut() += 10;
    println!("  data via RefCell: {:?}", data.borrow());
}

fn add_element(v: &mut Vec<i32>) {
    v.push(4);
}