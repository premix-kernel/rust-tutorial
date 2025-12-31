# panic! - Unrecoverable Errors

`panic!` หยุดโปรแกรมทันทีเมื่อเกิด error ที่จัดการไม่ได้

## เมื่อไหร่เกิด Panic?

### 1. เรียก panic! เอง

```rust
fn main() {
    panic!("crash and burn");
}
```

Output:

```text
thread 'main' panicked at 'crash and burn', src/main.rs:2:5
```
```