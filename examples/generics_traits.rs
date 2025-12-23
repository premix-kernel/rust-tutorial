// ===========================================
// Generics & Traits - บทที่ 10
// รัน: cargo run --example generics_traits
// ===========================================

#![allow(dead_code)] // Allow unused fields for demo purposes

use std::fmt::Display;

fn main() {
    println!("🦀 Generics & Traits Demo\n");

    // 1. Generic Function
    println!("1️⃣ Generic Function:");
    let numbers = vec![34, 50, 25, 100, 65];
    println!("Numbers: {:?}", numbers);
    println!("Largest: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Chars: {:?}", chars);
    println!("Largest: {}", largest(&chars));

    // 2. Generic Struct
    println!("\n2️⃣ Generic Struct:");
    let int_point = Point::new(5, 10);
    let float_point = Point::new(1.5, 4.5);
    println!("Int Point: {:?}", int_point);
    println!("Float Point: {:?}", float_point);
    println!(
        "Distance from origin: {}",
        float_point.distance_from_origin()
    );

    // 3. Multiple Type Parameters
    println!("\n3️⃣ Multiple Type Parameters:");
    let mixed = MixedPoint { x: 5, y: 4.5 };
    println!("Mixed: ({}, {})", mixed.x, mixed.y);

    // 4. Traits
    println!("\n4️⃣ Traits:");
    let article = NewsArticle {
        headline: String::from("Rust 2024 Released!"),
        author: String::from("Jane Doe"),
        content: String::from("Rust 2024 brings exciting new features..."),
    };
    println!("{}", article.summarize());
    println!("Author: {}", article.summarize_author());

    let tweet = Tweet {
        username: String::from("rustacean"),
        content: String::from("Learning Rust is fun! 🦀"),
        reply: false,
        retweet: false,
    };
    println!("{}", tweet.summarize());

    // 5. Trait Bounds
    println!("\n5️⃣ Trait Bounds:");
    notify(&article);
    notify(&tweet);

    // 6. Return impl Trait
    println!("\n6️⃣ Return impl Trait:");
    let item = returns_summarizable();
    println!("{}", item.summarize());

    // 7. Notify verbose demo
    println!("\n7️⃣ Notify Verbose:");
    notify_verbose(&tweet);
}

// Generic function
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Generic struct
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

// Specific implementation for f64
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Multiple type parameters
#[derive(Debug)]
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

// Trait definition
trait Summary {
    fn summarize(&self) -> String;

    // Default implementation
    fn summarize_author(&self) -> String {
        String::from("(Unknown)")
    }
}

struct NewsArticle {
    headline: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} - {}",
            self.headline,
            self.author,
            &self.content[..30]
        )
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!(
            "{}: {} (reply: {}, retweet: {})",
            self.username, self.content, self.reply, self.retweet
        )
    }
}

// Trait bounds
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Alternative syntax with Display
fn notify_verbose<T: Summary + Display>(item: &T) {
    println!("Verbose: {} - {}", item, item.summarize());
}

// Return impl Trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("example"),
        content: String::from("This is a sample tweet"),
        reply: false,
        retweet: false,
    }
}

// Display for Tweet (for notify_verbose)
impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.username)
    }
}
