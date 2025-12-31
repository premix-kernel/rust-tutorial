# Parameters (พารามิเตอร์)

**Parameters** คือค่าที่ส่งเข้าไปในฟังก์ชัน เพื่อให้ฟังก์ชันทำงานกับข้อมูลที่แตกต่างกันได้

## Syntax

```rust,ignore
fn function_name(param1: Type1, param2: Type2) {
    // use param1 and param2
}
```

> **สำคัญ:** ต้องระบุ **ชนิดข้อมูล** ของทุก parameter!

---

## ตัวอย่างพื้นฐาน

```rust
fn main() {
    print_number(5);
    print_number(10);
}

fn print_number(x: i32) {
    println!("The number is: {}", x);
}
```

ผลลัพธ์:

```text
The number is: 5
The number is: 10
```
```