// --- Defining Enums ---
enum IpAddrKind {
    V4,
    V6,
}

// --- Enums with Data ---
enum IpAddr {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // anonymous struct
    Write(String),
    ChangeColor(i32, i32, i32), // tuple
}

fn main() {
    // --- Enum Values ---
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // --- Using Enums with Data ---
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // --- Message Enum Usage ---
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 10, y: 20 };
    let m3 = Message::Write(String::from("hello"));
    let m4 = Message::ChangeColor(255, 0, 0);

    route(four);
    route(six);
}

fn route(ip_kind: IpAddrKind) {
    // handle routing
}
