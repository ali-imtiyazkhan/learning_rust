use std::fmt;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Implement Display trait manually
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    
    // Uses the Display implementation
    println!("Display output: {}", p);
    
    // Uses the #[derive(Debug)] implementation
    println!("Debug output: {:?}", p);
    
    // Uses pretty debug output
    println!("Pretty Debug output: {:#?}", p);
}
