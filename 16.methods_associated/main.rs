#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// --- Implementation Block ---
impl Rectangle {
    // --- Method ---
    // Methods are defined within the context of a struct (or enum/trait).
    // Their first parameter is always self, which represents the instance.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // --- Associated Function ---
    // Associated functions don't have self as their first parameter.
    // They are often used for constructors that create a new instance.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// Multiple impl blocks are allowed
impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Calling an associated function
    let sq = Rectangle::square(3);
    println!("Square: {:?}", sq);
}
