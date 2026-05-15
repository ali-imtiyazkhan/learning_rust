fn main() {
    // 1. Capacity vs Length
    let mut v = Vec::with_capacity(10);
    v.push(1);
    v.push(2);
    println!("Length: {}, Capacity: {}", v.len(), v.capacity()); // len: 2, cap: 10
    
    // 2. Removing Elements
    let mut numbers = vec![10, 20, 30, 40, 50];
    let last = numbers.pop(); // Returns Option<T>
    println!("Popped: {:?}, Remaining: {:?}", last, numbers);

    let second = numbers.remove(1); // Removes element at index 1 (shifts others)
    println!("Removed at index 1: {}, Remaining: {:?}", second, numbers);

    // 3. Sorting
    let mut unsorted = vec![5, 2, 8, 1, 9];
    unsorted.sort();
    println!("Sorted: {:?}", unsorted);

    // Sort by custom logic (e.g., descending)
    unsorted.sort_by(|a, b| b.cmp(a));
    println!("Sorted descending: {:?}", unsorted);

    // 4. Retain (Keep elements that match a condition)
    let mut v = vec![1, 2, 3, 4, 5, 6];
    v.retain(|&x| x % 2 == 0); // Keep only even numbers
    println!("After retain (even only): {:?}", v);

    // 5. Functional Methods
    let v = vec![1, 2, 3, 4, 5];
    
    // Summing values
    let sum: i32 = v.iter().sum();
    println!("Sum: {}", sum);

    // Checking conditions
    let all_positive = v.iter().all(|&x| x > 0);
    let any_greater_than_10 = v.iter().any(|&x| x > 10);
    println!("All positive: {}, Any > 10: {}", all_positive, any_greater_than_10);

    // 6. Splitting and Appending
    let mut v1 = vec![1, 2, 3];
    let mut v2 = vec![4, 5, 6];
    v1.append(&mut v2); // v2 becomes empty
    println!("Appended v1: {:?}, v2: {:?}", v1, v2);

    let v3 = v1.split_off(3); // Splits at index 3
    println!("v1 after split: {:?}, v3 (newly created): {:?}", v1, v3);
}
