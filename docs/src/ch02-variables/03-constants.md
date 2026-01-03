# Constants & Shadowing

## Constants (ค่าคงที่)

**Constants** คือค่าที่ไม่มีวันเปลี่ยนแปลง ใช้ `const` แทน `let`

```rust,ignore
const MAX_POINTS: u32 = 100_000;
const PI: f64 = 3.14159265359;
const APP_NAME: &str = "My Rust App";

fn main() {
    println!("Max points: {}", MAX_POINTS);
    println!("Pi: {}", PI);
    println!("App: {}", APP_NAME);
}
```

### กฎของ Constants

1. **ต้องระบุชนิด** - ไม่สามารถให้ Rust เดาได้
2. **ต้องเป็นค่าคงที่** - ไม่ได้มาจากการคำนวณตอน runtime
3. **ใช้ได้ทุกที่** - รวมถึง global scope
4. **SCREAMING_SNAKE_CASE** - ใช้ตัวพิมพ์ใหญ่และ underscore

```rust
// ✅ ถูกต้อง
const MAX_SIZE: usize = 100;

// ❌ ไม่ได้ - ต้องระบุชนิด
// const MIN_SIZE = 10;

// ❌ ไม่ได้ - ค่าต้องคงที่
// const RANDOM: u32 = rand::random();
```

---

## Static Variables

**Static** คล้าย constant แต่มี memory address คงที่

```rust,ignore
static LANGUAGE: &str = "Rust";
static mut COUNTER: u32 = 0; // ⚠️ ต้องใช้ unsafe

fn main() {
    println!("Language: {}", LANGUAGE);
}
```

> **คำเตือน:** `static mut` ต้องใช้ใน `unsafe` block
> และควรหลีกเลี่ยงถ้าเป็นไปได้

---

## const vs static vs let

| คุณสมบัติ         | const | static          | let          |
| ----------------- | ----- | --------------- | ------------ |
| เปลี่ยนค่าได้     | ❌    | ❌ (ยกเว้น mut) | ✅ (ถ้า mut) |
| ต้องระบุชนิด      | ✅    | ✅              | ❌           |
| Global scope      | ✅    | ✅              | ❌           |
| มี memory address | ❌    | ✅              | ✅           |
| Inline ได้        | ✅    | ❌              | ❌           |

---

## Shadowing (ทบทวน)

Shadowing คือการประกาศตัวแปรชื่อเดิมซ้ำ:

```rust,ignore
fn main() {
    let x = 5;

    // Shadow ด้วยค่าใหม่
    let x = x + 1;

    {
        // Shadow ใน inner scope
        let x = x * 2;
        println!("Inner x: {}", x); // 12
    }

    println!("Outer x: {}", x); // 6
}
```

### ประโยชน์ของ Shadowing

#### 1. เปลี่ยนชนิดข้อมูลได้

```rust,ignore
fn main() {
    let spaces = "   ";        // &str
    let spaces = spaces.len(); // usize

    println!("spaces: {}", spaces); // 3
}
```

#### 2. แปลงค่าโดยไม่ต้องสร้างชื่อใหม่

```rust,ignore
fn main() {
    let input = "42";
    let input: i32 = input.parse().unwrap();

    println!("input: {}", input); // 42
}
```

#### 3. ใช้ใน Scope ที่ต้องการ

```rust,ignore
fn main() {
    let x = 1;

    {
        let x = 2;
        println!("x in block: {}", x); // 2
    }

    println!("x outside: {}", x); // 1
}
```

---

## ตัวอย่างการใช้งานจริง

### Configuration Constants

```rust,ignore
const MAX_CONNECTIONS: u32 = 100;
const TIMEOUT_SECONDS: u64 = 30;
const API_VERSION: &str = "v1.0";

fn main() {
    println!("Server config:");
    println!("  Max connections: {}", MAX_CONNECTIONS);
    println!("  Timeout: {} seconds", TIMEOUT_SECONDS);
    println!("  API version: {}", API_VERSION);
}
```

### Mathematical Constants

```rust,ignore
const PI: f64 = 3.14159265358979323846;
const E: f64 = 2.71828182845904523536;

fn main() {
    let radius = 5.0;
    let area = PI * radius * radius;

    println!("Circle area: {:.2}", area); // 78.54
}
```

---

## ลองทำดู! 🎯

1. สร้าง constants สำหรับแอพของคุณ (ชื่อแอพ, เวอร์ชัน)
2. ลอง shadow ตัวแปรเพื่อแปลง string เป็น number
3. สร้าง constants สำหรับค่า RGB ของสี

---

## สรุปบทที่ 2

ในบทนี้คุณได้เรียนรู้:

- ✅ Immutable และ Mutable variables
- ✅ Scalar types: integers, floats, booleans, characters
- ✅ Compound types: tuples และ arrays
- ✅ Constants และ Shadowing

👉 ต่อไป: [บทที่ 3: Functions](../ch03-functions/README.md)
