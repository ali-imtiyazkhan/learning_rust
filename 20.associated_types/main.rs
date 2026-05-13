// --- Associated Types ---
// Associated types connect a type placeholder with a trait such that the 
// trait method definitions can use these placeholder types in their signatures.

pub trait Iterator {
    type Item; // Associated type

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32; // Specifying the concrete type for the associated type

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// --- Comparison with Generics ---
// If we used generics:
// pub trait IteratorGeneric<T> {
//     fn next(&mut self) -> Option<T>;
// }
// This would allow multiple implementations for the same type (e.g., IteratorGeneric<String> 
// and IteratorGeneric<u32> for Counter), which is usually not what we want for an iterator.

fn main() {
    let mut counter = Counter::new();

    println!("Next: {:?}", counter.next());
    println!("Next: {:?}", counter.next());
    println!("Next: {:?}", counter.next());
    println!("Next: {:?}", counter.next());
    println!("Next: {:?}", counter.next());
    println!("Next: {:?}", counter.next());
}
