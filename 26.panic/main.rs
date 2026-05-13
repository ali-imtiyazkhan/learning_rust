fn main() {
    // --- Unrecoverable Errors with panic! ---
    // panic!("crash and burn");

    // --- Using panic! Backtraces ---
    // If you run this code, it will panic because the index is out of bounds.
    let v = vec![1, 2, 3];
    // v[99]; // This will cause a panic!

    // To see a backtrace, run:
    // $env:RUST_BACKTRACE=1; cargo run

    // --- When to panic! ---
    // 1. Examples, prototype code, and tests.
    // 2. Cases where you have more information than the compiler (e.g., unwrap()).
    // 3. When your code ends up in a bad state (logic error).

    // let x: Option<i32> = None;
    // x.unwrap(); // This will panic!
}
