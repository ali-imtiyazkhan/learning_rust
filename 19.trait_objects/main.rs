pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // Trait Object: dyn Draw
    // Box<dyn Draw> is a pointer to any type that implements the Draw trait.
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button: {}", self.label);
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing a select box with {} options", self.options.len());
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

// --- Static Dispatch (Generics) ---
// This forces all components to be of the SAME type T.
// pub struct ScreenGeneric<T: Draw> {
//     pub components: Vec<T>,
// }

// --- Dynamic Dispatch (Trait Objects) ---
// This allows a Vec to contain DIFFERENT types, as long as they implement Draw.
