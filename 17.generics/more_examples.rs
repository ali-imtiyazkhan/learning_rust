use std::fmt::{Display, Debug};

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn compare_and_print<T: Display + PartialOrd>(a: T, b: T) {
    if a > b {
        println!("{} is greater than {}", a, b);
    } else {
        println!("{} is not greater than {}", a, b);
    }
}

fn some_function<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("T is {}", t);
    println!("U is {:?}", u);
    0
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }


    fn mixup<V>(self, other: Pair<V>) -> Pair<(T, V)> {
        Pair {
            x: (self.x, other.x),
            y: (self.y, other.y),
        }
    }
}

struct ArrayWrapper<T, const N: usize> {
    data: [T; N],
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest number: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest char: {}", largest(&chars));

    compare_and_print(10, 5);
    compare_and_print("apple", "banana");

    some_function(String::from("Hello"), vec![1, 2, 3]);

    let p1 = Pair::new(5, 10);
    let p2 = Pair::new("A", "B");
    let p3 = p1.mixup(p2);
    println!("Mixed pair x: {:?}", p3.x);

    let _wrapper = ArrayWrapper {
        data: [1, 2, 3], 
    };
    let _wrapper: ArrayWrapper<i32, 3> = ArrayWrapper { data: [1, 2, 3] };
}
