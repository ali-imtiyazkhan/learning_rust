// --- Defining Structs ---
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// --- Tuple Structs ---
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// --- Unit-Like Structs ---
struct AlwaysEqual;

fn main() {
    // --- Instantiating Structs ---
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someuser123"),
        active: true,
        sign_in_count: 1,
    };

    // Accessing fields
    user1.email = String::from("anotheremail@example.com");
    println!("User email: {}", user1.email);

    // Using Field Init Shorthand
    let user2 = build_user(String::from("user2@example.com"), String::from("user2"));
    println!("User2: {}", user2.username);

    // --- Struct Update Syntax ---
    // Create a new struct instance from an existing one
    let user3 = User {
        email: String::from("user3@example.com"),
        ..user1 // Use remaining fields from user1
    };

    println!("User3 sign in count: {}", user3.sign_in_count);

    // --- Tuple Structs Usage ---
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Black color: {}, origin point: {}", black.0, origin.0);

    // --- Unit-Like Structs Usage ---
    let _subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // shorthand
        username, // shorthand
        active: true,
        sign_in_count: 1,
    }
}
