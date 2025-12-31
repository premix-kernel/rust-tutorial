# Async พื้นฐาน

Asynchronous programming ใน Rust ช่วยจัดการ I/O-bound tasks อย่างมีประสิทธิภาพ

## Sync vs Async

### Synchronous (Blocking)

```rust,ignore
fn main() {
    let data1 = fetch_data_1(); // รอ... 2 seconds
    let data2 = fetch_data_2(); // รอ... 2 seconds
    // Total: 4 seconds
}
```

### Asynchronous (Non-blocking)

```rust,ignore
async fn main() {
    let (data1, data2) = join!(
        fetch_data_1(), // เริ่มพร้อมกัน
        fetch_data_2()
    );
    // Total: ~2 seconds
}
```

---

## async fn

```rust,ignore
async fn hello_world() {
    println!("Hello, world!");
}

// async fn returns a Future
// must be run by an executor
```

**async fn returns `impl Future`** - ไม่รันทันที ต้อง await หรือใช้ executor

---

## .await

```rust,ignore
async fn fetch_number() -> i32 {
    // simulate async work
    42
}

async fn main() {
    let number = fetch_number().await; // รอ Future complete
    println!("Number: {}", number);
}
```

### Sequential vs Concurrent

```rust,ignore
async fn learn_song() -> String {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    "La la la".to_string()
}

async fn sing_song(song: &str) {
    println!("Singing: {}", song);
}

async fn dance() {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("Dancing!");
}

// Sequential
async fn sequential() {
    let song = learn_song().await; // 1 sec
    sing_song(&song).await;
    dance().await;                  // 1 sec
    // Total: 2+ seconds
}

// Concurrent - dance while learning
async fn concurrent() {
    let (song, _) = tokio::join!(
        learn_song(),  // start learning
        dance()        // dance at the same time
    );
    sing_song(&song).await;
    // Total: ~1 second (overlapped)
}
```text

---

## Runtime: Tokio

Rust ไม่มี async runtime ในตัว ต้องใช้ library:

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

### Basic Usage

```rust,ignore
#[tokio::main]
async fn main() {
    println!("Hello from async main!");

    let result = do_something().await;
    println!("Result: {}", result);
}

async fn do_something() -> i32 {
    // simulate async work
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    42
}
```

### Manual Runtime

```rust,ignore
fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        println!("Running on tokio runtime");
    });
}
```

---

## async Block

```rust,ignore
async fn example() {
    let future = async {
        // code here runs lazily
        println!("Inside async block");
        42
    };

    // future ยังไม่รัน!
    let result = future.await;
    println!("Result: {}", result);
}
```

### Capture Variables

```rust,compile_fail
async fn example() {
    let name = String::from("Alice");

    // move ownership into async block
    let greeting = async move {
        format!("Hello, {}!", name)
    };

    // name moved, can't use here
    // println!("{}", name); // ❌

    println!("{}", greeting.await);
}
```

---

## Futures are Lazy

```rust,ignore
async fn my_async_fn() {
    println!("This runs when awaited");
}

fn main() {
    let future = my_async_fn(); // ไม่รันอะไรเลย!

    // ต้อง await หรือใช้ executor
    // future.await; // or
    // tokio::runtime::Runtime::new().unwrap().block_on(future);
}
```

---

## Return Types

```rust,ignore
use std::future::Future;

// These are equivalent:
async fn foo() -> i32 {
    42
}

fn bar() -> impl Future<Output = i32> {
    async {
        42
    }
}
```

---

## Error Handling in Async

```rust,ignore
async fn fetch_data() -> Result<String, std::io::Error> {
    // simulate possible failure
    Ok("data".to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = fetch_data().await?;
    println!("Data: {}", data);
    Ok(())
}
```

---

## Async in Traits (Rust 1.75+)

```rust,ignore
trait AsyncFetcher {
    async fn fetch(&self) -> String;
}

struct MyFetcher;

impl AsyncFetcher for MyFetcher {
    async fn fetch(&self) -> String {
        "data".to_string()
    }
}
```text

---

## When to Use Async

| Situation                  | Use                |
| -------------------------- | ------------------ |
| I/O-bound (network, files) | ✅ Async           |
| CPU-bound (computation)    | ❌ Use threads     |
| Need simple code           | ❌ Stick with sync |
| High concurrency           | ✅ Async           |

---

## ลองทำดู! 🎯

1. สร้าง async function ที่ return Result
2. ใช้ tokio::time::sleep แล้ว await
3. ลอง tokio::join! กับ 2 futures

---

## สรุป

| Concept          | Description                     |
| ---------------- | ------------------------------- |
| `async fn`       | Returns Future                  |
| `.await`         | Wait for Future                 |
| `async { }`      | Async block                     |
| `#[tokio::main]` | Async main entry                |
| Lazy             | Futures don't run until awaited |

### Runtimes

| Runtime   | Use Case                      |
| --------- | ----------------------------- |
| Tokio     | General purpose, most popular |
| async-std | Simpler, std-like API         |
| smol      | Minimal, lightweight          |

👉 ต่อไป: [Futures](./02-futures.md)

```