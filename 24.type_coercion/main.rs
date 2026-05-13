// --- Deref Coercion ---
// Deref coercion converts a reference to a type that implements the Deref trait 
// into a reference to another type. For example, deref coercion can convert 
// &String to &str because String implements the Deref trait such that it 
// returns &str.

fn main() {
    let m = MyBox::new(String::from("Rust"));
    
    // Deref coercion makes it possible to call hello with a reference to MyBox<String>
    // 1. MyBox<String> is dereferenced to String
    // 2. String is dereferenced to &str
    hello(&m);

    // Without deref coercion, we would have to write:
    // hello(&(*m)[..]);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// --- Types of Coercion ---
// 1. From &T to &U when T: Deref<Target=U>
// 2. From &mut T to &mut U when T: DerefMut<Target=U>
// 3. From &mut T to &U when T: Deref<Target=U>
