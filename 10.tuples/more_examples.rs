fn main() {
    let person = ("Alice", 28, true);
    
    // Destructuring a tuple
    let (name, age, is_active) = person;
    
    println!("Name: {}, Age: {}, Active: {}", name, age, is_active);
    
    // Accessing via index
    println!("First element: {}", person.0);
}
