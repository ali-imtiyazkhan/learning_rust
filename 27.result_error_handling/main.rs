use std::fs::File;
use std::io::{self, Read};

fn main() {
    // --- Recoverable Errors with Result ---
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => {
            println!("Problem opening the file: {:?}", error);
            // Handle error (e.g., return or panic)
            return;
        }
    };

    // --- Shortcuts for Panic on Error: unwrap and expect ---
    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("hello.txt should be included in this project");

    // --- Propagating Errors ---
    let result = read_username_from_file();
    println!("Username result: {:?}", result);
}

// --- The ? Operator: A Shortcut for Propagating Errors ---
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// Chaining method calls with the ? operator
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
