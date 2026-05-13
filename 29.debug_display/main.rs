use std::fmt;

// --- The Debug Trait ---
// We can use the derive attribute to automatically implement Debug
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

// --- The Display Trait ---
// Display must be implemented manually for custom type
struct Point {
    x: f64,
    y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize how it should be displayed
        write!(f, "({}, {})", self.x, self.y)
    }
}

// let (s1, s2) = (&t.0, &t.1);  borrowing
// 's1' and 's2' are just LOOKING at the strings in 't'. 't' is still valid.

// let (s1, s2) = t.clone(); 
// 's1' and 's2' are NEW strings. 't' is still valid.


fn main() {
    // 1. Printing with Debug
    // {:?} is for normal debug, {:#?} is for "pretty" debug (multi-line)
    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));

    // 2. Printing with Display
    let point = Point { x: 3.3, y: 7.2 };
    println!("Display: {}", point);
    println!("Debug: {:?}", point); // This would FAIL if we didn't add #[derive(Debug)]

    // Let's add Debug to Point as well
}

// Re-defining Point with Debug for the example
#[derive(Debug)]
struct DebugPoint {
    x: f64,
    y: f64,
}

impl fmt::Display for DebugPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}
