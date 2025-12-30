# Safe Abstractions

ห่อ unsafe code ด้วย safe API - วิธีที่ถูกต้องในการใช้ unsafe

## หลักการ

**Unsafe ควรอยู่ข้างใน, Safe ควรอยู่ข้างนอก**

```text
┌─────────────────────────────────┐
│         Safe API                │  ◄── ผู้ใช้เรียกที่นี่
│  ┌─────────────────────────┐    │
│  │    Unsafe Implementation│    │  ◄── unsafe อยู่ข้างใน
│  │    (hidden)             │    │
│  └─────────────────────────┘    │
└─────────────────────────────────┘
```

---

## ตัวอย่าง 1: split_at_mut

Standard library ใช้ pattern นี้:

```rust
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    // Safe precondition check
    assert!(mid <= len, "mid must be <= len");

    // Unsafe implementation ที่ถูกต้อง
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut v, 3);

    left[0] = 100;
    right[0] = 200;

    println!("{:?}", left);   // [100, 2, 3]
    println!("{:?}", right);  // [200, 5, 6]
}
```

### ทำไมถึง Safe?

1. **Precondition check**: `assert!(mid <= len)` ป้องกัน out-of-bounds
2. **Non-overlapping**: สอง slices ไม่ทับกัน
3. **Valid lifetimes**: ทั้งคู่ผูกกับ input lifetime

---

## ตัวอย่าง 2: Custom Vec

```rust,ignore
pub struct MyVec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        MyVec {
            ptr: std::ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.cap {
            self.grow();
        }

        unsafe {
            std::ptr::write(self.ptr.add(self.len), value);
        }
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe { Some(&*self.ptr.add(index)) }
        } else {
            None
        }
    }

    fn grow(&mut self) {
        let new_cap = if self.cap == 0 { 1 } else { self.cap * 2 };
        let new_layout = std::alloc::Layout::array::<T>(new_cap).unwrap();

        let new_ptr = if self.cap == 0 {
            unsafe { std::alloc::alloc(new_layout) as *mut T }
        } else {
            let old_layout = std::alloc::Layout::array::<T>(self.cap).unwrap();
            unsafe {
                std::alloc::realloc(
                    self.ptr as *mut u8,
                    old_layout,
                    new_layout.size(),
                ) as *mut T
            }
        };

        self.ptr = new_ptr;
        self.cap = new_cap;
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if self.cap > 0 {
            // Drop all elements
            for i in 0..self.len {
                unsafe {
                    std::ptr::drop_in_place(self.ptr.add(i));
                }
            }
            // Free memory
            let layout = std::alloc::Layout::array::<T>(self.cap).unwrap();
            unsafe {
                std::alloc::dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}
```

---

## ตัวอย่าง 3: Thread-safe Counter

```rust,ignore
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Counter {
    value: AtomicUsize,
}

impl Counter {
    pub fn new() -> Self {
        Counter {
            value: AtomicUsize::new(0),
        }
    }

    pub fn increment(&self) -> usize {
        self.value.fetch_add(1, Ordering::SeqCst)
    }

    pub fn get(&self) -> usize {
        self.value.load(Ordering::SeqCst)
    }
}

// Safe to share across threads
unsafe impl Sync for Counter {}
```

---

## Best Practices

### 1. Minimize Unsafe Scope

```rust
// ❌ Bad: unsafe block ใหญ่เกินไป
unsafe {
    let ptr = some_pointer();
    let len = calculate_length();
    validate_input(len);
    let slice = std::slice::from_raw_parts(ptr, len);
    process(slice);
}

// ✅ Good: unsafe แค่ที่จำเป็น
let ptr = some_pointer();
let len = calculate_length();
validate_input(len);
let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
process(slice);
```

### 2. Document Invariants

```rust
/// A non-empty string that is guaranteed to be valid UTF-8.
///
/// # Invariants
///
/// - `ptr` always points to valid memory
/// - The memory contains valid UTF-8 bytes
/// - `len` is always > 0
pub struct NonEmptyString {
    ptr: *const u8,
    len: usize,
}
```

### 3. Validate Early

```rust
pub fn from_raw_parts(ptr: *const u8, len: usize) -> Result<Self, Error> {
    // ตรวจสอบก่อน unsafe
    if ptr.is_null() {
        return Err(Error::NullPointer);
    }
    if len == 0 {
        return Err(Error::EmptyString);
    }

    // unsafe เฉพาะส่วนที่ต้องใช้จริง
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };

    // ตรวจสอบหลัง
    if std::str::from_utf8(slice).is_err() {
        return Err(Error::InvalidUtf8);
    }

    Ok(Self { ptr, len })
}
```

### 4. Test Extensively

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_at_mut_basic() {
        let mut v = vec![1, 2, 3, 4];
        let (left, right) = split_at_mut(&mut v, 2);
        assert_eq!(left, &[1, 2]);
        assert_eq!(right, &[3, 4]);
    }

    #[test]
    fn test_split_at_mut_empty_left() {
        let mut v = vec![1, 2, 3];
        let (left, right) = split_at_mut(&mut v, 0);
        assert!(left.is_empty());
        assert_eq!(right, &[1, 2, 3]);
    }

    #[test]
    #[should_panic]
    fn test_split_at_mut_out_of_bounds() {
        let mut v = vec![1, 2, 3];
        split_at_mut(&mut v, 10);  // should panic
    }
}
```

---

## ลองทำดู! 🎯

1. สร้าง safe wrapper สำหรับ raw pointer operation
2. implement `get_unchecked` ใน custom collection
3. เขียน tests สำหรับ edge cases

---

## สรุปบทที่ 17

| แนวคิด           | คำอธิบาย                 |
| ---------------- | ------------------------ |
| unsafe block     | เปิดใช้ unsafe features  |
| raw pointers     | *const T, *mut T         |
| unsafe fn        | ฟังก์ชันที่ต้อง unsafe   |
| extern           | FFI                      |
| safe abstraction | ห่อ unsafe ด้วย safe API |

### Checklist ก่อน Ship unsafe Code

- [ ] Preconditions ถูก document
- [ ] ทุก invariant ถูกรักษา
- [ ] unsafe scope น้อยที่สุด
- [ ] มี tests ครอบคลุม
- [ ] Code review แล้ว

👉 ต่อไป: [บทที่ 18: Macros](../ch18-macros/README.md)
