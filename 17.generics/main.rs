// --- Generics in Structs ---
struct Point<T> {
    x: T,
    y: T,
}

struct PointMulti<T, U> {
    x: T,
    y: U,
}

// --- Generics in Enums ---
// enum Option<T> {
//     Some(T),
//     None,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// --- Generics in Methods ---
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Only for f32
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    // --- Using Generics in Functions ---
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    // --- Using Generic Structs ---
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // let wont_work = Point { x: 5, y: 4.0 }; // ERROR: mixed types

    let both_integer = PointMulti { x: 5, y: 10 };
    let both_float = PointMulti { x: 1.0, y: 4.0 };
    let integer_and_float = PointMulti { x: 5, y: 4.0 }; // Works!

    println!("integer.x = {}", integer.x());
}

// Without generics
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// With generics (requires Trait bounds, which is the next topic)
// fn largest<T: PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
