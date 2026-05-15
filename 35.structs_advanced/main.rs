// --- Advanced Struct Examples ---

// 1. Nested Structs
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

// 2. Structs with Methods and Associated Functions (impl)
impl Line {
    // Associated function (like a static method/constructor)
    fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self {
            start: Point { x: x1, y: y1 },
            end: Point { x: x2, y: y2 },
        }
    }

    // Method
    fn length(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx * dx + dy * dy).sqrt()
    }
}

// 3. Implementing Default for a Struct
#[derive(Debug)]
struct Config {
    port: u16,
    timeout: u32,
    retries: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 8080,
            timeout: 30,
            retries: 3,
        }
    }
}

// 4. A simple Builder Pattern
struct DatabaseConfig {
    host: String,
    port: u16,
}

impl DatabaseConfig {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            port: 5432,
        }
    }

    fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    fn build(self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

// 5. Recursive Struct (using Box)
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // Using Nested Structs
    let line = Line::new(0.0, 0.0, 3.0, 4.0);
    println!("Line: {:#?}", line);
    println!("Length: {}", line.length());

    // Using Default Implementation
    let default_config = Config::default();
    let custom_config = Config {
        port: 9000,
        ..Config::default()
    };
    println!("Default Config: {:?}", default_config);
    println!("Custom Config: {:?}", custom_config);

    // Using Builder Pattern
    let db_conn = DatabaseConfig::new("localhost")
        .port(3306)
        .build();
    println!("Database Connection: {}", db_conn);

    // Recursive Struct Usage
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("Recursive List: {:?}", list);
}
