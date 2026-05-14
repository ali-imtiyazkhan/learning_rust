fn main() {
    let a = [10, 20, 30, 40, 50];

    // 1. Accessing elements by index (starts at 0)
    let first = a[0];
    let second = a[1];
    println!("First: {}, Second: {}", first, second);

    // 2. Indexing out of bounds
    // let element = a[10]; // This would cause a compile-time error if the index is a constant
    // println!("This will panic if accessed at runtime with a dynamic index out of bounds");

    // 3. Iterating over an array using a loop
    println!("Iterating with for loop:");
    for element in a.iter() {
        println!("Value: {}", element);
    }

    // 4. Iterating with indices
    println!("Iterating with indices:");
    for i in 0..a.len() {
        println!("Index {}: {}", i, a[i]);
    }

    // 5. Multi-dimensional arrays
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6],
    ];
    println!("Matrix[0][1]: {}", matrix[0][1]);
}
