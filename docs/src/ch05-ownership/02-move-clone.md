# Move & Clone

## Move (การย้าย)

เมื่อ assign ตัวแปร heap type ให้ตัวแปรอื่น ค่าจะถูก **move**:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // MOVE!

    // s1 invalidated แล้ว
    // println!("{}", s1); // ❌ Error: borrow of moved value

    println!("{}", s2); // ✅ OK
}
```

### แผนภาพ Move

```
ก่อน Move:
s1 ──────► "hello" (Heap)

หลัง Move:
s1 ──╳──► (invalidated)
s2 ──────► "hello" (Heap)
```

---

## Clone (การคัดลอก Deep Copy)

ถ้าต้องการ copy ข้อมูลจริงๆ ใช้ `.clone()`:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // CLONE - deep copy

    println!("s1 = {}", s1); // ✅ OK
    println!("s2 = {}", s2); // ✅ OK
}
```

### แผนภาพ Clone

```
หลัง Clone:
s1 ──────► "hello" (Heap memory 1)
s2 ──────► "hello" (Heap memory 2)
```

> **คำเตือน:** `.clone()` อาจแพงในแง่ performance เพราะต้อง copy ทุก byte

---

## Copy vs Clone

|              | Copy                    | Clone              |
| ------------ | ----------------------- | ------------------ |
| ทำงานอย่างไร | Automatic, bitwise copy | ต้องเรียก .clone() |
| ราคา         | ถูก (stack only)        | อาจแพง (heap copy) |
| ใช้กับ       | Stack types             | Heap types         |
| ตัวอย่าง     | `i32`, `bool`           | `String`, `Vec`    |

### Copy (อัตโนมัติสำหรับ Stack types)

```rust
fn main() {
    let x = 5;
    let y = x; // COPY (อัตโนมัติ)

    println!("x = {}, y = {}", x, y); // ✅ ทั้งสองใช้ได้
}
```

---

## เมื่อไหร่ใช้ Clone?

### 1. ต้องการให้ทั้งสองตัวแปรใช้ข้อมูลเดียวกัน

```rust
fn main() {
    let original = String::from("hello");
    let backup = original.clone();

    // ใช้ทั้งสองได้
    println!("original: {}", original);
    println!("backup: {}", backup);
}
```

### 2. ส่งค่าเข้า function แต่ยังต้องการใช้

```rust
fn main() {
    let s = String::from("hello");

    print_string(s.clone()); // ส่ง clone ไป

    println!("Still have: {}", s); // ✅ s ยังใช้ได้
}

fn print_string(s: String) {
    println!("{}", s);
}
```

> **ดีกว่า:** ใช้ References แทน clone (บทถัดไป)

---

## Clone กับ Collections

```rust
fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = v1.clone();

    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);

    // แก้ไข v2 ไม่กระทบ v1
    // (ถ้าทำได้ ต้องเป็น mut)
}
```

---

## Copy Trait

Types ที่มี **Copy trait** จะถูก copy อัตโนมัติ:

```rust
fn main() {
    // ทั้งหมดนี้มี Copy
    let a: i32 = 5;
    let b = a; // copy

    let c: f64 = 3.14;
    let d = c; // copy

    let e: bool = true;
    let f = e; // copy

    let g: char = 'A';
    let h = g; // copy

    // Tuple ที่มี Copy types
    let tuple1 = (1, 2, 3);
    let tuple2 = tuple1; // copy

    // ใช้ได้ทั้งหมด
    println!("{} {} {} {} {} {}", a, c, e, g, tuple1.0, tuple2.0);
}
```

---

## สรุป: Move vs Clone vs Copy

```rust
fn main() {
    // Copy - Stack types (อัตโนมัติ)
    let x = 5;
    let y = x;
    println!("x={}, y={}", x, y); // ✅

    // Move - Heap types (default)
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // ❌ moved
    println!("{}", s2); // ✅

    // Clone - Explicit deep copy
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3={}, s4={}", s3, s4); // ✅
}
```

---

## ลองทำดู! 🎯

1. สร้าง 2 Strings และลอง move ระหว่างกัน
2. ใช้ `.clone()` เพื่อให้ทั้งสองใช้ได้
3. ลองกับ Vec และ HashMaps

---

## สรุป

| การทำงาน | เมื่อไหร่            | ผลลัพธ์                      |
| -------- | -------------------- | ---------------------------- |
| Copy     | Stack types          | ทั้งสองใช้ได้                |
| Move     | Heap types (default) | ตัวแรกใช้ไม่ได้              |
| Clone    | เรียก .clone()       | ทั้งสอง​ใช้ได้ (คนละ memory) |

👉 ต่อไป: [References & Borrowing](./03-references.md)
