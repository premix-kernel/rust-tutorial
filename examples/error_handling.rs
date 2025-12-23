// ===========================================
// Error Handling - บทที่ 9
// รัน: cargo run --example error_handling
// ===========================================

use std::fs::File;
use std::io::{self, Read};

fn main() {
    println!("🦀 Error Handling Demo\n");

    // 1. Result basics
    println!("1️⃣ Result basics:");
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // 2. unwrap_or
    println!("\n2️⃣ unwrap_or:");
    let result = divide(10.0, 0.0).unwrap_or(0.0);
    println!("Result with default: {}", result);

    // 3. map and and_then
    println!("\n3️⃣ Chaining with map:");
    let doubled = divide(10.0, 2.0).map(|x| x * 2.0).map(|x| x + 1.0);
    println!("10/2 * 2 + 1 = {:?}", doubled);

    // 4. ? operator
    println!("\n4️⃣ ? operator:");
    match read_username() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Could not read username: {}", e),
    }

    // 5. Custom Result type
    println!("\n5️⃣ Parse and calculate:");
    match parse_and_double("42") {
        Ok(n) => println!("'42' doubled = {}", n),
        Err(e) => println!("Error: {}", e),
    }

    match parse_and_double("abc") {
        Ok(n) => println!("'abc' doubled = {}", n),
        Err(e) => println!("Error: {}", e),
    }

    // 6. Option to Result
    println!("\n6️⃣ Option to Result:");
    let numbers = vec![1, 2, 3];
    match get_element(&numbers, 1) {
        Ok(n) => println!("Element at 1: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    match get_element(&numbers, 10) {
        Ok(n) => println!("Element at 10: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

// Custom divide function
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

// Using ? operator
fn read_username() -> Result<String, io::Error> {
    let mut file = match File::open("username.txt") {
        Ok(f) => f,
        Err(_) => return Ok(String::from("default_user")), // fallback
    };

    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username.trim().to_string())
}

// Parse and calculate
fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = s.parse()?;
    Ok(n * 2)
}

// Convert Option to Result
fn get_element(vec: &[i32], index: usize) -> Result<i32, String> {
    vec.get(index)
        .copied()
        .ok_or_else(|| format!("Index {} out of bounds", index))
}
