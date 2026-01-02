# Unsafe Functions

ฟังก์ชันที่ต้องเรียกภายใน `unsafe` block

## Defining Unsafe Functions

```rust
// ประกาศด้วย unsafe fn
unsafe fn dangerous() {
    println!("Doing something dangerous!");
}

fn main() {
    // ❌ Error: เรียกนอก unsafe ไม่ได้
    // dangerous();

    // ✅ ต้องเรียกใน unsafe block
    unsafe {
        dangerous();
    }
}
```

---

## เมื่อไหร่ใช้ unsafe fn?

ใช้เมื่อฟังก์ชันมี **preconditions** ที่ compiler ตรวจไม่ได้:

```rust
/// Divides two numbers without checking for zero.
///
/// # Safety
///
/// Caller must ensure that `divisor` is not zero.
unsafe fn divide_unchecked(dividend: i32, divisor: i32) -> i32 {
    dividend / divisor
}

fn main() {
    let a = 10;
    let b = 2;

    // Caller รับผิดชอบเรื่อง preconditions
    unsafe {
        let result = divide_unchecked(a, b);
        println!("{} / {} = {}", a, b, result);
    }
}
```

---

## Document Safety Requirements

**สำคัญ:** ทุก unsafe fn ควรมี `# Safety` section:

```rust
/// Reads a value from the given pointer.
///
/// # Safety
///
/// - `ptr` must be valid and properly aligned.
/// - `ptr` must point to a properly initialized value of type `T`.
/// - The memory at `ptr` must not be mutated while this function runs.
unsafe fn read_value<T>(ptr: *const T) -> T {
    ptr.read()
}
```

---

## FFI - Foreign Function Interface

เรียกฟังก์ชันจากภาษาอื่น (เช่น C):

### เรียก C Functions

```rust,ignore
// ประกาศ external functions
extern "C" {
    fn abs(input: i32) -> i32;
    fn sqrt(x: f64) -> f64;
}

fn main() {
    unsafe {
        println!("abs(-5) = {}", abs(-5));
        println!("sqrt(16.0) = {}", sqrt(16.0));
    }
}
```

### Calling Convention

| ABI          | ใช้กับ                |
| ------------ | --------------------- |
| `"C"`        | C functions (default) |
| `"system"`   | Windows API           |
| `"stdcall"`  | Win32 API             |
| `"fastcall"` | Optimized             |

```rust,ignore
extern "system" {
    fn GetCurrentProcessId() -> u32;
}
```

---

## Export Rust to C

```rust,ignore
#[no_mangle]  // ไม่เปลี่ยนชื่อ function
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn greet(name: *const std::ffi::c_char) {
    unsafe {
        let c_str = std::ffi::CStr::from_ptr(name);
        if let Ok(s) = c_str.to_str() {
            println!("Hello, {}!", s);
        }
    }
}
```

### Build as Library

```toml,ignore
# Cargo.toml
[lib]
crate-type = ["cdylib"]  # Dynamic library
# หรือ
crate-type = ["staticlib"]  # Static library
```

---

## Working with C Strings

```rust,ignore
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

extern "C" {
    fn strlen(s: *const c_char) -> usize;
}

fn main() {
    // Rust String -> C string
    let rust_string = "Hello, C!";
    let c_string = CString::new(rust_string).unwrap();

    unsafe {
        let len = strlen(c_string.as_ptr());
        println!("Length: {}", len);
    }

    // C string -> Rust String
    let c_ptr: *const c_char = c_string.as_ptr();
    unsafe {
        let c_str = CStr::from_ptr(c_ptr);
        let rust_str = c_str.to_str().unwrap();
        println!("Back to Rust: {}", rust_str);
    }
}
```

---

## ตัวอย่างจริง: Binding to C Library

```rust,ignore
// Binding to libc
#[link(name = "c")]
extern "C" {
    fn puts(s: *const std::ffi::c_char) -> i32;
    fn getenv(name: *const std::ffi::c_char) -> *const std::ffi::c_char;
}

fn main() {
    use std::ffi::{CString, CStr};

    let msg = CString::new("Hello from Rust via C!").unwrap();
    let var = CString::new("PATH").unwrap();

    unsafe {
        puts(msg.as_ptr());

        let path = getenv(var.as_ptr());
        if !path.is_null() {
            let path_str = CStr::from_ptr(path).to_str().unwrap();
            println!("PATH = {}", &path_str[..50.min(path_str.len())]);
        }
    }
}
```

---

## Static Mut Variables

```rust,ignore
static mut COUNTER: u32 = 0;

fn increment() {
    unsafe {
        COUNTER += 1;
    }
}

fn get_count() -> u32 {
    unsafe { COUNTER }
}

fn main() {
    increment();
    increment();
    increment();

    println!("Count: {}", get_count());  // 3
}
```

> **คำเตือน:** `static mut` อันตรายใน multi-threaded! ใช้ `Mutex` หรือ `AtomicU32` แทน

---

## ลองทำดู! 🎯

1. สร้าง unsafe fn พร้อม Safety documentation
2. เรียก C function `abs` และ `sqrt`
3. สร้าง Rust function ที่ export ไป C

---

## สรุป

| Concept     | Syntax                               |
| ----------- | ------------------------------------ |
| Declare     | `unsafe fn name() {}`                |
| Call        | `unsafe { name(); }`                 |
| FFI declare | `extern "C" { fn name(); }`          |
| FFI export  | `#[no_mangle] pub extern "C" fn`     |
| C string    | `CString::new()`, `CStr::from_ptr()` |
| Static mut  | `static mut NAME: T = value;`        |

👉 ต่อไป: [Safe Abstractions](./04-safe-abstractions.md)
