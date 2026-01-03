# Raw Pointers

**Raw Pointers** (`*const T` และ `*mut T`) คือ pointers แบบ C ไม่มี safety guarantees

## สร้าง Raw Pointers

```rust,ignore
fn main() {
    let mut num = 5;

    // สร้าง raw pointers จาก references (safe)
    let r1: *const i32 = &num;      // immutable raw pointer
    let r2: *mut i32 = &mut num;    // mutable raw pointer

    println!("r1 address: {:p}", r1);
    println!("r2 address: {:p}", r2);

    // การสร้าง pointer เป็น safe
    // แต่การใช้งาน (dereference) ต้อง unsafe
}
```

---

## Dereference Raw Pointers

```rust,ignore
fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // ❌ Error: ทำนอก unsafe ไม่ได้
    // println!("{}", *r1);

    // ✅ ต้องใช้ unsafe block
    unsafe {
        println!("r1: {}", *r1);  // อ่านค่า
        *r2 = 10;                  // เขียนค่า
        println!("r2: {}", *r2);
    }
}
```

---

## Raw Pointers vs References

| Aspect     | References (&T)     | Raw Pointers (\*const T) |
| ---------- | ------------------- | ------------------------ |
| Null       | ❌ ไม่ได้           | ✅ ได้                   |
| Dangling   | ❌ Compiler ป้องกัน | ✅ อาจเกิดได้            |
| Aliasing   | มี rules            | ไม่มี rules              |
| Auto-deref | ✅ ได้              | ❌ ต้อง unsafe           |
| Lifetime   | ✅ tracked          | ❌ ไม่ tracked           |

---

## สร้าง Null Pointer

```rust,ignore
fn main() {
    // null pointer
    let null_ptr: *const i32 = std::ptr::null();
    let null_mut_ptr: *mut i32 = std::ptr::null_mut();

    // ตรวจสอบ null
    if null_ptr.is_null() {
        println!("Pointer is null!");
    }

    // ❌ อันตราย: dereference null pointer จะ crash
    // unsafe { println!("{}", *null_ptr); }
}
```

---

## Pointer Arithmetic

```rust,ignore
fn main() {
    let arr = [1, 2, 3, 4, 5];
    let ptr = arr.as_ptr();  // *const i32

    unsafe {
        // เข้าถึง elements ด้วย pointer arithmetic
        println!("arr[0] = {}", *ptr);
        println!("arr[1] = {}", *ptr.add(1));
        println!("arr[2] = {}", *ptr.add(2));

        // offset ใช้ signed integer
        println!("arr[4] = {}", *ptr.offset(4));
    }
}
```

### Pointer Methods

| Method          | Description                 |
| --------------- | --------------------------- |
| `ptr.add(n)`    | เลื่อนไปข้างหน้า n elements |
| `ptr.sub(n)`    | เลื่อนไปข้างหลัง n elements |
| `ptr.offset(n)` | เลื่อนด้วย signed offset    |
| `ptr.is_null()` | ตรวจ null                   |
| `ptr.read()`    | อ่านค่า (unsafe)            |
| `ptr.write(v)`  | เขียนค่า (unsafe)           |

---

## Casting Between Pointer Types

```rust,ignore
fn main() {
    let mut num = 42i32;

    // reference -> raw pointer
    let ptr: *mut i32 = &mut num;

    // const <-> mut (อันตราย!)
    let const_ptr: *const i32 = ptr;
    let mut_ptr: *mut i32 = const_ptr as *mut i32;

    // pointer -> integer
    let addr: usize = ptr as usize;
    println!("Address: 0x{:x}", addr);

    // integer -> pointer (อันตรายมาก!)
    let ptr_from_int: *const i32 = addr as *const i32;

    unsafe {
        println!("Value: {}", *ptr_from_int);
    }
}
```

---

## ตัวอย่างจริง: Swap Values

```rust,ignore
unsafe fn swap<T>(a: *mut T, b: *mut T) {
    let temp = std::ptr::read(a);
    std::ptr::copy_nonoverlapping(b, a, 1);
    std::ptr::write(b, temp);
}

fn main() {
    let mut x = 1;
    let mut y = 2;

    unsafe {
        swap(&mut x, &mut y);
    }

    println!("x = {}, y = {}", x, y);  // x = 2, y = 1
}
```

---

## Safe Pointer Functions

```rust,ignore
use std::ptr;

fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    let src = [10, 20, 30];

    unsafe {
        // copy_nonoverlapping: memcpy
        ptr::copy_nonoverlapping(
            src.as_ptr(),
            arr.as_mut_ptr(),
            3
        );
    }

    println!("{:?}", arr);  // [10, 20, 30, 4, 5]
}
```

---

## ลองทำดู! 🎯

1. สร้าง raw pointer จาก reference
2. ใช้ pointer arithmetic เข้าถึง array
3. ลอง swap values ด้วย raw pointers

---

## สรุป

| Concept     | Syntax             |
| ----------- | ------------------ |
| Immutable   | `*const T`         |
| Mutable     | `*mut T`           |
| Create      | `&x as *const T`   |
| Null        | `std::ptr::null()` |
| Dereference | `unsafe { *ptr }`  |
| Add         | `ptr.add(n)`       |

### ข้อควรระวัง

1. ❌ อย่า dereference null pointer
2. ❌ อย่า dereference dangling pointer
3. ❌ อย่าละเมิด aliasing rules
4. ✅ ใช้เฉพาะเมื่อจำเป็น

👉 ต่อไป: [Unsafe Functions](./03-unsafe-functions.md)
