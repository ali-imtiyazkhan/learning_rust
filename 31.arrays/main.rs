fn main() {
    // 1. Fixed-size arrays: [T; N]
    // Arrays in Rust have a fixed length known at compile time.
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array a: {:?}", a);

    // 2. Initialize an array with the same value for all elements: [value; N]
    let b = [0; 10]; // Creates an array of 10 zeros
    println!("Array b (all zeros): {:?}", b);

    // 3. Getting the length of an array
    println!("Length of a: {}", a.len());

    // 4. Arrays live on the stack
    // Unlike Vectors, arrays are not growable and are allocated on the stack.
    println!("Size of array a: {} bytes", std::mem::size_of_val(&a));
}
