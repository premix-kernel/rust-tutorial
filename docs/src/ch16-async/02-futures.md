# Futures

**Future** คือ trait หลักของ async programming ใน Rust

## Future Trait

```rust
pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),   // Future เสร็จแล้ว มีค่า T
    Pending,    // ยังไม่เสร็จ ให้ poll อีกทีหลัง
}
```

---

## ทำงานอย่างไร?

```text
                    ┌─────────────┐
                    │   Future    │
                    └──────┬──────┘
                           │ poll()
                           ▼
              ┌────────────┴────────────┐
              │                         │
        Poll::Ready(T)            Poll::Pending
        (เสร็จแล้ว)              (รอก่อน)
              │                         │
              ▼                         ▼
         ใช้ค่า T              Executor จะ poll
                               อีกครั้งเมื่อพร้อม
```

---

## async Block

สร้าง anonymous Future:

```rust,ignore
async fn example() {
    let future = async {
        println!("Inside async block");
        42
    };

    // future ยังไม่รัน!
    let result = future.await;  // รันตอนนี้
    println!("Result: {}", result);
}
```

---

## Futures are Lazy

**สำคัญมาก:** Future ไม่ทำอะไรจนกว่าจะถูก poll

```rust,ignore
async fn do_something() {
    println!("This runs when polled!");
}

fn main() {
    let future = do_something(); // ไม่ print!

    // ต้อง await หรือใช้ executor
    // ไม่งั้น future จะไม่ทำอะไรเลย

    drop(future); // ทิ้งทั้งที่ยังไม่ได้รัน
}
```

---

## Combine Futures

### Sequential

```rust,ignore
async fn sequential() {
    let a = async_fn_1().await;  // รัน 1 ก่อน
    let b = async_fn_2().await;  // แล้วรัน 2
    println!("{} {}", a, b);
}
```

### Concurrent กับ join!

```rust,ignore
use tokio::join;

async fn concurrent() {
    // รันพร้อมกัน!
    let (a, b) = join!(
        async_fn_1(),
        async_fn_2()
    );
    println!("{} {}", a, b);
}
```

### Racing กับ select!

```rust,ignore
use tokio::select;

async fn race() {
    select! {
        result = async_fn_1() => println!("1 finished: {}", result),
        result = async_fn_2() => println!("2 finished: {}", result),
    }
    // แค่อันแรกที่เสร็จ
}
```

---

## Pin คืออะไร?

`Pin` ป้องกัน Future ไม่ให้ถูก move ในหน่วยความจำ:

```rust
use std::pin::Pin;
use std::future::Future;

fn takes_future(future: Pin<&mut dyn Future<Output = i32>>) {
    // future ถูก pin ไว้จะ move ไม่ได้
}
```

**ทำไมต้อง Pin?**

- บาง Future มี self-references
- ถ้า move แล้ว references จะ invalid
- Pin รับประกันว่าจะไม่ move

---

## สร้าง Future เอง (Advanced)

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct MyFuture {
    count: u32,
}

impl Future for MyFuture {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.count += 1;
        if self.count >= 3 {
            Poll::Ready(self.count)
        } else {
            cx.waker().wake_by_ref(); // บอกให้ poll อีกครั้ง
            Poll::Pending
        }
    }
}
```

---

## async fn vs Future

```rust,ignore
// ทั้งสองเหมือนกัน:

async fn foo() -> i32 {
    42
}

fn bar() -> impl Future<Output = i32> {
    async { 42 }
}
```text

---

## ลองทำดู! 🎯

1. สร้าง async function ที่ return String
2. ใช้ join! รวม 2 futures
3. ลองสร้าง future แล้วไม่ await ดูว่าเกิดอะไร

---

## สรุป

| Concept       | Description               |
| ------------- | ------------------------- |
| Future        | Trait สำหรับ async values |
| Poll::Ready   | Future เสร็จแล้ว          |
| Poll::Pending | ยังไม่เสร็จ               |
| Lazy          | ไม่รันจนกว่าจะ poll       |
| Pin           | ป้องกันการ move           |

👉 ต่อไป: [Tokio Runtime](./03-tokio.md)

```