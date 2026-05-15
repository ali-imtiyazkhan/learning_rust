fn main() {
    // 1. Mutating an array
    let mut numbers = [1, 2, 3, 4, 5];
    numbers[0] = 10;
    println!("Mutated array: {:?}", numbers);

    // 2. Passing arrays to functions
    // By value (must specify size)
    print_fixed_array(numbers);
    
    // By reference (as a slice, more flexible)
    print_slice(&numbers);

    // 3. Array Slicing
    let slice = &numbers[1..4]; // Elements at index 1, 2, 3
    println!("Slice [1..4]: {:?}", slice);

    // 4. Using Iterators with Arrays
    // Using .map() (requires collecting into a Vec or using in a loop)
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled via map: {:?}", doubled);

    // 5. Windows and Chunks
    println!("Windows of 2:");
    for window in numbers.windows(2) {
        println!("{:?}", window); // [10, 2], [2, 3], [3, 4], [4, 5]
    }

    println!("Chunks of 2:");
    for chunk in numbers.chunks(2) {
        println!("{:?}", chunk); // [10, 2], [3, 4], [5]
    }

    // 6. Zip example
    let a = [1, 2, 3];
    let b = [4, 5, 6];
    println!("Zipped arrays:");
    for (val_a, val_b) in a.iter().zip(b.iter()) {
        println!("a: {}, b: {}, sum: {}", val_a, val_b, val_a + val_b);
    }
}

fn print_fixed_array(arr: [i32; 5]) {
    println!("Inside print_fixed_array: {:?}", arr);
}

fn print_slice(slice: &[i32]) {
    println!("Inside print_slice: {:?}", slice);
}
