// Basic Enum
enum Direction {
    North,
    South,
    East,
    West,
}

// Enum with Data
enum Shape {
    Circle(f64),               // radius
    Rectangle(f64, f64),       // width, height
    Triangle(f64, f64, f64),   // three sides
}

// Enum with Struct-like variants
enum UserStatus {
    Active,
    Inactive,
    Banned { reason: String },
}

// Function using match with enum
fn describe_direction(dir: Direction) {
    match dir {
        Direction::North => println!("Going North! ⬆️"),
        Direction::South => println!("Going South! ⬇️"),
        Direction::East  => println!("Going East!  ➡️"),
        Direction::West  => println!("Going West!  ⬅️"),
    }
}

// Function to calculate area using Shape enum
fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(r)         => 3.14 * r * r,
        Shape::Rectangle(w, h)   => w * h,
        Shape::Triangle(a, b, c) => {
            // Heron's formula
            let s = (a + b + c) / 2.0;
            (s * (s-a) * (s-b) * (s-c)).sqrt()
        }
    }
}

// Function for UserStatus
fn check_status(status: UserStatus) {
    match status {
        UserStatus::Active              => println!("User is Active ✅"),
        UserStatus::Inactive            => println!("User is Inactive ❌"),
        UserStatus::Banned { reason }   => println!("User Banned: {}", reason),
    }
}

fn main() {
    // Using Direction enum
    let dir = Direction::North;
    describe_direction(dir);

    // Using Shape enum
    let circle    = Shape::Circle(5.0);
    let rectangle = Shape::Rectangle(4.0, 6.0);
    let triangle  = Shape::Triangle(3.0, 4.0, 5.0);

    println!("Circle Area    : {:.2}", calculate_area(circle));
    println!("Rectangle Area : {:.2}", calculate_area(rectangle));
    println!("Triangle Area  : {:.2}", calculate_area(triangle));

    // Using UserStatus enum
    check_status(UserStatus::Active);
    check_status(UserStatus::Inactive);
    check_status(UserStatus::Banned { 
        reason: String::from("Violated rules") 
    });
}