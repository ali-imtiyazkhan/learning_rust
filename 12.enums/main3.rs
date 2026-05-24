// Enum holding different types of data
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Enum for user
enum UserInfo {
    Name(String),
    Age(u32),
    Email(String),
    Address { city: String, pincode: u32 },
}

fn extract_message(msg: Message) {
    match msg {
        // Unit variant - no data
        Message::Quit => {
            println!("Quit - no data!");
        }

        // Struct variant - extract named fields
        Message::Move { x, y } => {
            println!("Move to x={}, y={}", x, y);
        }

        // Tuple variant - extract single value
        Message::Write(text) => {
            println!("Write message: {}", text);
        }

        // Tuple variant - extract multiple values
        Message::ChangeColor(r, g, b) => {
            println!("Change color to R={}, G={}, B={}", r, g, b);
        }
    }
}

fn extract_user_info(info: UserInfo) {
    match info {
        UserInfo::Name(n) => {
            println!("Name    : {}", n);
        }
        UserInfo::Age(a) => {
            println!("Age     : {}", a);
        }
        UserInfo::Email(e) => {
            println!("Email   : {}", e);
        }
        UserInfo::Address { city, pincode } => {
            println!("City    : {}", city);
            println!("Pincode : {}", pincode);
        }
    }
}

// Using if let - extract single variant easily
fn if_let_example(msg: Message) {
    // Only extract if it matches Move variant
    if let Message::Move { x, y } = msg {
        println!("if let → Moving to x={}, y={}", x, y);
    } else {
        println!("Not a Move variant!");
    }
}

// Using while let - extract in a loop
fn while_let_example() {
    let mut stack = vec![
        Some(1),
        Some(2),
        Some(3),
        None,
    ];

    while let Some(top) = stack.pop() {
        println!("Got value: {}", top);
    }
}

// Real world - Option enum data extraction
fn option_example(value: Option<i32>) {
    match value {
        Some(n) => println!("Got number: {}", n),
        None    => println!("Got nothing!"),
    }
}

// Real world - Result enum data extraction
fn result_example(value: Result<i32, String>) {
    match value {
        Ok(n)    => println!("Success: {}", n),
        Err(e)   => println!("Error  : {}", e),
    }
}

fn main() {
    println!("===== Message Enum =====");
    extract_message(Message::Quit);
    extract_message(Message::Move { x: 10, y: 20 });
    extract_message(Message::Write(String::from("Hello Nasruddin!")));
    extract_message(Message::ChangeColor(255, 128, 0));

    println!("\n===== UserInfo Enum =====");
    extract_user_info(UserInfo::Name(String::from("Nasruddin Khan")));
    extract_user_info(UserInfo::Age(30));
    extract_user_info(UserInfo::Email(String::from("nasru@example.com")));
    extract_user_info(UserInfo::Address {
        city: String::from("Jaipur"),
        pincode: 302001,
    });

    println!("\n===== if let Example =====");
    if_let_example(Message::Move { x: 5, y: 15 });

    println!("\n===== while let Example =====");
    while_let_example();

    println!("\n===== Option Example =====");
    option_example(Some(42));
    option_example(None);

    println!("\n===== Result Example =====");
    result_example(Ok(100));
    result_example(Err(String::from("Something went wrong!")));
}