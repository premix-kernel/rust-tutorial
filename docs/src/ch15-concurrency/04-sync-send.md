# Sync & Send

Marker traits สำหรับ concurrency

## Send

Type ที่ส่งระหว่าง threads ได้:

```rust
// Most types are Send
// Rc<T> is NOT Send (use Arc<T> instead)
```

## Sync

Type ที่หลาย threads เข้าถึงพร้อมกันได้:

```rust
// T is Sync if &T is Send
// RefCell<T> is NOT Sync
// Mutex<T> IS Sync
```text

---

## สรุปบทที่ 15

| แนวคิด        | ใช้เมื่อ                       |
| ------------- | ------------------------------ |
| thread::spawn | สร้าง thread                   |
| channel       | ส่งข้อมูลระหว่าง threads       |
| Mutex         | ป้องกัน data race              |
| Arc           | แชร์ ownership ระหว่าง threads |

👉 ต่อไป: [บทที่ 16: Async/Await](../ch16-async/README.md)

```