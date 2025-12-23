// ===========================================
// Ownership - บทที่ 5
// รัน: cargo run --example ownership
// ===========================================

fn main() {
    println!("🦀 Ownership Demo\n");

    // 1. Move
    println!("1️⃣ Move:");
    let s1 = String::from("hello");
    let s2 = s1; // s1 ถูก move ไป s2
    // println!("{}", s1); // ❌ Error! s1 ถูก move ไปแล้ว
    println!("s2 = {}", s2);

    // 2. Clone
    println!("\n2️⃣ Clone:");
    let s3 = String::from("world");
    let s4 = s3.clone(); // deep copy
    println!("s3 = {}, s4 = {}", s3, s4);

    // 3. Copy (stack types)
    println!("\n3️⃣ Copy:");
    let x = 5;
    let y = x; // copy (integers implement Copy)
    println!("x = {}, y = {}", x, y);

    // 4. References (Borrowing)
    println!("\n4️⃣ References (Borrowing):");
    let s5 = String::from("rust");
    let len = calculate_length(&s5); // borrow
    println!("'{}' has {} characters", s5, len);

    // 5. Mutable References
    println!("\n5️⃣ Mutable References:");
    let mut s6 = String::from("Hello");
    println!("Before: {}", s6);
    change(&mut s6);
    println!("After: {}", s6);

    // 6. Slices
    println!("\n6️⃣ Slices:");
    let s7 = String::from("Hello World");
    let hello = &s7[0..5];
    let world = &s7[6..11];
    println!("First word: {}", hello);
    println!("Second word: {}", world);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", World!");
}
