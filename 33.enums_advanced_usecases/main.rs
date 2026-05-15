enum WebEvent {
    // An enum variant can have no data
    PageLoad,
    PageUnload,
    // An enum variant can include a tuple
    KeyPress(char),
    // An enum variant can include an anonymous struct
    Paste(String),
    // Or include multiple types in a tuple
    Click { x: i64, y: i64 },
}

// You can define methods on enums using impl
impl WebEvent {
    fn inspect(&self) {
        match self {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            // Destructure 'c' from the KeyPress
            WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
            WebEvent::Paste(s) => println!("pasted \"{}\".", s),
            // Destructure 'x' and 'y' from the Click
            WebEvent::Click { x, y } => {
                println!("clicked at x={}, y={}.", x, y);
            },
        }
    }
}

// A state machine example
enum State {
    Start,
    Running,
    Finished(u32), // result code
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    pressed.inspect();
    pasted.inspect();
    click.inspect();
    load.inspect();
    unload.inspect();

    // --- State Machine ---
    let mut current_state = State::Start;

    loop {
        match current_state {
            State::Start => {
                println!("Initializing...");
                current_state = State::Running;
            }
            State::Running => {
                println!("System is running...");
                current_state = State::Finished(0);
            }
            State::Finished(code) => {
                println!("Process finished with code: {}", code);
                break;
            }
        }
    }
}
