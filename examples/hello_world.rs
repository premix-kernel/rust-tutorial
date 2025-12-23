// ===========================================
// Hello World - บทที่ 1
// รัน: cargo run --example hello_world
// ===========================================

fn main() {
    // พิมพ์ข้อความ
    println!("🦀 สวัสดี Rust!");
    println!("Hello, World!");

    // ตัวแปร (บทที่ 2)
    let name = "Rustacean";
    let age: u32 = 25;
    let is_learning = true;

    println!("\n📝 ตัวอย่างตัวแปร:");
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Is Learning: {}", is_learning);

    // ฟังก์ชัน (บทที่ 3)
    println!("\n🔧 ตัวอย่างฟังก์ชัน:");
    greet("World");
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);

    // Control Flow (บทที่ 4)
    println!("\n🔀 ตัวอย่าง Control Flow:");
    check_number(10);

    for i in 1..=5 {
        println!("Loop: {}", i);
    }
}

// ฟังก์ชันทักทาย
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// ฟังก์ชันบวกเลข
fn add(a: i32, b: i32) -> i32 {
    a + b // ไม่มี semicolon = return
}

// ตรวจสอบเลข
fn check_number(n: i32) {
    if n > 0 {
        println!("{} is positive", n);
    } else if n < 0 {
        println!("{} is negative", n);
    } else {
        println!("{} is zero", n);
    }
}
