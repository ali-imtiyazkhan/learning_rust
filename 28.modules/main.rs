// --- Modules, Packages, and Crates ---

// 1. A crate is the smallest amount of code that the Rust compiler considers at a time.
// 2. A package is one or more crates that provide a set of functionality.
// 3. A package contains a Cargo.toml file that describes how to build those crates.

// --- Defining Modules ---
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {} // private by default
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// --- Bringing Paths into Scope with 'use' ---
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // With 'use'
    hosting::add_to_waitlist();
}

fn main() {
    eat_at_restaurant();
    println!("Modules example created!");
}
