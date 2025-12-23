// ===========================================
// Structs & Enums - บทที่ 6-7
// รัน: cargo run --example structs_enums
// ===========================================

fn main() {
    println!("🦀 Structs & Enums Demo\n");

    // 1. Struct
    println!("1️⃣ Struct:");
    let user = User {
        name: String::from("สมชาย"),
        age: 30,
        email: String::from("somchai@example.com"),
    };
    println!(
        "User: {}, Age: {}, Email: {}",
        user.name, user.age, user.email
    );

    // 2. Methods
    println!("\n2️⃣ Methods:");
    let rect = Rectangle::new(10, 20);
    println!("Rectangle: {:?}", rect);
    println!("Area: {}", rect.area());
    println!("Can hold 5x5: {}", rect.can_hold(&Rectangle::new(5, 5)));

    // 3. Tuple Struct
    println!("\n3️⃣ Tuple Struct:");
    let color = Color(255, 128, 0);
    println!("RGB: ({}, {}, {})", color.0, color.1, color.2);

    // 4. Enum - all directions
    println!("\n4️⃣ Enum:");
    let directions = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];
    for dir in directions {
        move_player(dir);
    }

    // 5. Enum with Data
    println!("\n5️⃣ Enum with Data:");
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello")),
        Message::ChangeColor(255, 0, 0),
    ];
    for msg in messages {
        process_message(msg);
    }

    // 6. Option
    println!("\n6️⃣ Option:");
    let some_number = Some(42);
    let no_number: Option<i32> = None;
    print_option(some_number);
    print_option(no_number);

    // 7. Match - all coins
    println!("\n7️⃣ Match:");
    for coin in [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter] {
        println!("{:?} = {} cents", coin, value_in_cents(coin));
    }
}

// Struct
struct User {
    name: String,
    age: u32,
    email: String,
}

// Struct with methods
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Tuple Struct
struct Color(u8, u8, u8);

// Enum
#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn move_player(dir: Direction) {
    match dir {
        Direction::North => println!("Moving North ⬆️"),
        Direction::South => println!("Moving South ⬇️"),
        Direction::East => println!("Moving East ➡️"),
        Direction::West => println!("Moving West ⬅️"),
    }
}

// Enum with Data
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: RGB({}, {}, {})", r, g, b),
    }
}

fn print_option(opt: Option<i32>) {
    match opt {
        Some(value) => println!("Value: {}", value),
        None => println!("No value"),
    }
}

#[derive(Debug, Clone, Copy)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
