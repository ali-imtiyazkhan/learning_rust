fn main() {
    // --- Partial Move and Destructuring Example ---

    struct Person {
        name: String,
        age: Box<u8>,
    }
    
    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };
    
    // By using 'ref', we borrow the fields instead of moving them.
    // If we didn't use 'ref' for 'name', it would be moved out of 'person',
    // making 'person' unusable.
    let Person { ref name, ref age } = person;
    
    println!("The person's age is {}", age);
    println!("The person's name is {}", name);
    
    // Because we used 'ref', the original 'person' struct is still valid!
    println!("The person age from the struct is {}", person.age);
}
