// ===========================================
// Collections - บทที่ 8
// รัน: cargo run --example collections
// ===========================================

use std::collections::HashMap;

fn main() {
    println!("🦀 Collections Demo\n");

    // 1. Vec - Dynamic Array
    println!("1️⃣ Vector:");
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    println!("Vector: {:?}", numbers);

    // vec! macro
    let fruits = vec!["Apple", "Banana", "Orange"];
    println!("Fruits: {:?}", fruits);

    // Access
    println!("First fruit: {}", fruits[0]);
    println!("Second fruit: {:?}", fruits.get(1));

    // Iterate
    print!("All fruits: ");
    for fruit in &fruits {
        print!("{} ", fruit);
    }
    println!();

    // 2. String
    println!("\n2️⃣ String:");
    let mut s = String::from("Hello");
    s.push_str(", ");
    s.push_str("World!");
    println!("{}", s);

    // String methods
    println!("Length: {}", s.len());
    println!("Is empty: {}", s.is_empty());
    println!("Contains 'World': {}", s.contains("World"));

    // String concatenation
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = format!("{} {}!", s1, s2);
    println!("Formatted: {}", s3);

    // 3. HashMap
    println!("\n3️⃣ HashMap:");
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    println!("Scores: {:?}", scores);

    // Access
    let team = String::from("Blue");
    if let Some(score) = scores.get(&team) {
        println!("{} team score: {}", team, score);
    }

    // Update
    scores.insert(String::from("Blue"), 25); // overwrite
    scores.entry(String::from("Yellow")).or_insert(30); // insert if not exists
    println!("Updated: {:?}", scores);

    // Iterate
    println!("All scores:");
    for (key, value) in &scores {
        println!("  {} = {}", key, value);
    }

    // 4. Word Count Example
    println!("\n4️⃣ Word Count:");
    let text = "hello world hello rust world rust rust";
    let mut word_count: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Text: \"{}\"", text);
    println!("Word counts:");
    for (word, count) in &word_count {
        println!("  {}: {}", word, count);
    }
}
