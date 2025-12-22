# Unsafe คืออะไร

**unsafe** ปลดล็อคความสามารถพิเศษที่ compiler ตรวจสอบไม่ได้

## Unsafe Superpowers

ใน `unsafe` block ทำได้ 5 อย่าง:

| Superpower               | คำอธิบาย               |
| ------------------------ | ---------------------- |
| Dereference raw pointers | ใช้ *const T, *mut T   |
| Call unsafe functions    | ฟังก์ชันที่มี `unsafe` |
| Access mutable statics   | ตัวแปร static mut      |
| Implement unsafe traits  | เช่น Send, Sync        |
| Access union fields      | union แบบ C            |

---

## 1. Dereference Raw Pointers

```rust
fn main() {
    let mut num = 5;

    // สร้าง raw pointers (safe - ยังไม่ได้ dereference)
    let r1 = &num as *const i32;  // immutable raw
    let r2 = &mut num as *mut i32; // mutable raw

    // dereference ต้อง unsafe
    unsafe {
        println!("r1 is: {}", *r1);
        *r2 = 10;
        println!("r2 is: {}", *r2);
    }
}
```

### Raw Pointers vs References

| Aspect       | References       | Raw Pointers |
| ------------ | ---------------- | ------------ |
| Null         | ไม่ได้           | ได้          |
| Dangling     | Compiler ป้องกัน | ไม่ป้องกัน   |
| Aliasing     | มี rules         | ไม่มี rules  |
| Auto-cleanup | ผ่าน Drop        | ไม่มี        |

---

## 2. Call Unsafe Functions

```rust
unsafe fn dangerous() {
    println!("Doing dangerous stuff!");
}

fn main() {
    // เรียก unsafe function ต้องอยู่ใน unsafe block
    unsafe {
        dangerous();
    }
}
```

---

## 3. Access Mutable Statics

```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```

> **คำเตือน:** `static mut` อันตรายมากใน multi-threaded ใช้ `Mutex` หรือ `Atomic` แทน

---

## 4. Implement Unsafe Traits

```rust
unsafe trait MyUnsafeTrait {
    fn do_something(&self);
}

unsafe impl MyUnsafeTrait for i32 {
    fn do_something(&self) {
        println!("{}", self);
    }
}
```

ตัวอย่างจริง: `Send` และ `Sync` traits

---

## 5. Access Union Fields

```rust
#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}

fn main() {
    let u = MyUnion { f1: 1 };

    // อ่าน field ต้อง unsafe เพราะไม่รู้ว่า field ไหนถูก set
    unsafe {
        println!("f1: {}", u.f1);
    }
}
```

---

## เมื่อไหร่ควรใช้ unsafe?

### ✅ ใช้เมื่อ

1. **FFI** - เรียก C/C++ code
2. **Performance** - hot paths ที่ต้องเร็วมาก
3. **Hardware** - เข้าถึง hardware โดยตรง
4. **Implement abstractions** - สร้าง safe wrapper

### ❌ ไม่ควรใช้

1. ข้าม borrow checker เพราะไม่เข้าใจ
2. แก้ compile error แบบขี้เกียจ
3. ทุกที่ที่มีทางเลือก safe

---

## Safe Abstractions

วิธีที่ถูกต้อง: ห่อ unsafe ด้วย safe interface

```rust
pub fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    // unsafe ภายใน แต่ interface เป็น safe
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

---

## ลองทำดู! 🎯

1. สร้าง raw pointer และ dereference
2. สร้าง unsafe function
3. ลองเข้าถึง static mut

---

## สรุป

| Keyword        | ใช้เมื่อ                       |
| -------------- | ------------------------------ |
| `unsafe { }`   | Block ที่ทำ unsafe operations  |
| `unsafe fn`    | Function ที่ต้องเรียกใน unsafe |
| `unsafe trait` | Trait ที่ต้อง impl ใน unsafe   |
| `unsafe impl`  | Implement unsafe trait         |

### Best Practices

1. **ลด unsafe ให้น้อยที่สุด**
2. **Document invariants** ที่ต้องรักษา
3. **Test extensively**
4. **Wrap ด้วย safe API**

👉 ต่อไป: [Raw Pointers](./02-raw-pointers.md)
