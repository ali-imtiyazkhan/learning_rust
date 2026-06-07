fn main(){
    let greeting = String::from("Hello, World!");
    println!("{}", greeting); // output : Hello, World!

    let char1 = greeting.chars().nth(0).unwrap();
    println!("The first character is: {}", char1);   // output : H

    for i in 0..100{
        println!("The value of i is: {}", i); // output : 0 to 99
    }
}