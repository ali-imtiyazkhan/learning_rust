// Fill in the blank and fix the errors
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Added missing function
fn show_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("Quit!");
        }
        Message::Move { x, y } => {
            println!("Move to x={}, y={}", x, y);
        }
        Message::Write(text) => {
            println!("Write: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Color: R={}, G={}, B={}", r, g, b);
        }
    }
}

fn main() {
    // Fixed: added type "Message" in array brackets
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg)
    }
}