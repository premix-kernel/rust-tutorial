# แบบฝึกหัด: บทที่ 17 - Unsafe Rust

## แบบฝึกหัดที่ 1: Raw Pointers

สร้างและ dereference raw pointers

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
fn main() {
    let x = 10;

    // สร้าง raw pointers
    let ptr_const: *const i32 = &x;
    let ptr_mut: *mut i32 = &x as *const i32 as *mut i32;

    unsafe {
        println!("Value via const ptr: {}", *ptr_const);
        println!("Value via mut ptr: {}", *ptr_mut);
    }
}
```text

</details>

---

## แบบฝึกหัดที่ 2: Unsafe Function

สร้าง unsafe function ที่ swap ค่า 2 ตัว

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
unsafe fn swap_raw(a: *mut i32, b: *mut i32) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

fn main() {
    let mut x = 10;
    let mut y = 20;

    println!("Before: x={}, y={}", x, y);

    unsafe {
        swap_raw(&mut x, &mut y);
    }

    println!("After: x={}, y={}", x, y);
}
```text

</details>

---

## แบบฝึกหัดที่ 3: Safe Abstraction

ห่อ unsafe code ด้วย safe function

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    let (left, right) = split_at_mut(&mut arr, 2);

    println!("Left: {:?}", left);   // [1, 2]
    println!("Right: {:?}", right); // [3, 4, 5]
}
```text

</details>

---

## แบบฝึกหัดที่ 4: Static Mutable

ใช้ static mut (ระวัง!)

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
static mut COUNTER: i32 = 0;

fn increment() {
    unsafe {
        COUNTER += 1;
    }
}

fn main() {
    increment();
    increment();
    increment();

    unsafe {
        println!("Counter: {}", COUNTER);
    }
}
```text

**หมายเหตุ:** ไม่แนะนำใน production code! ใช้ `Mutex` หรือ `AtomicI32` แทน

</details>

---

## แบบฝึกหัดที่ 5: FFI

เรียก C function จาก Rust (concept)

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
// ประกาศ external function
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -5: {}", abs(-5));
    }
}

// สร้าง function ให้ C เรียก
#[no_mangle]
pub extern "C" fn rust_function(x: i32) -> i32 {
    x * 2
}
```

</details>

---

👉 [บทที่ 18](./ch18-exercises.md)

```