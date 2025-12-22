# Tokio Runtime

**Tokio** เป็น async runtime ที่นิยมที่สุดใน Rust ecosystem

## Setup

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

Features ที่มี:

| Feature           | Description                    |
| ----------------- | ------------------------------ |
| `rt`              | Runtime core                   |
| `rt-multi-thread` | Multi-threaded runtime         |
| `time`            | tokio::time                    |
| `net`             | TCP/UDP networking             |
| `io-util`         | I/O utilities                  |
| `sync`            | Synchronization primitives     |
| `macros`          | #[tokio::main], #[tokio::test] |
| `full`            | ทุกอย่าง                       |

---

## Basic Usage

```rust
#[tokio::main]
async fn main() {
    println!("Hello from Tokio!");

    let result = do_something().await;
    println!("Result: {}", result);
}

async fn do_something() -> String {
    // simulate async work
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    "Done!".to_string()
}
```

### #[tokio::main] ทำอะไร?

```rust
// มันแปลง:
#[tokio::main]
async fn main() {
    // code
}

// เป็น:
fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            // code
        })
}
```

---

## Manual Runtime

```rust
fn main() {
    // สร้าง runtime เอง
    let rt = tokio::runtime::Runtime::new().unwrap();

    // รัน future
    rt.block_on(async {
        println!("Running on Tokio!");
    });

    // หรือ spawn และ block
    let handle = rt.spawn(async {
        42
    });

    let result = rt.block_on(handle).unwrap();
    println!("Result: {}", result);
}
```

### Runtime Types

```rust
// Multi-threaded (default)
let rt = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(4)
    .enable_all()
    .build()
    .unwrap();

// Single-threaded (current thread)
let rt = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .unwrap();
```

---

## Spawning Tasks

### tokio::spawn

```rust
use tokio::task;

#[tokio::main]
async fn main() {
    // Spawn task - รันแบบ concurrent
    let handle = task::spawn(async {
        println!("Hello from spawned task!");
        42
    });

    // รอผลลัพธ์
    let result = handle.await.unwrap();
    println!("Task returned: {}", result);
}
```

### Multiple Tasks

```rust
use tokio::task;

#[tokio::main]
async fn main() {
    let mut handles = vec![];

    for i in 0..5 {
        let handle = task::spawn(async move {
            println!("Task {} running", i);
            i * 2
        });
        handles.push(handle);
    }

    // รอทุก tasks
    for handle in handles {
        let result = handle.await.unwrap();
        println!("Result: {}", result);
    }
}
```

---

## Concurrent Execution

### join! - รอทุก futures

```rust
use tokio::join;
use tokio::time::{sleep, Duration};

async fn task_a() -> i32 {
    sleep(Duration::from_millis(100)).await;
    1
}

async fn task_b() -> i32 {
    sleep(Duration::from_millis(200)).await;
    2
}

#[tokio::main]
async fn main() {
    // รันพร้อมกัน, รอทุกอันเสร็จ
    let (a, b) = join!(task_a(), task_b());

    println!("Results: {} + {} = {}", a, b, a + b);
    // ใช้เวลา ~200ms (ไม่ใช่ 300ms)
}
```

### try_join! - หยุดเมื่อ error

```rust
use tokio::try_join;

async fn may_fail(succeed: bool) -> Result<i32, &'static str> {
    if succeed {
        Ok(42)
    } else {
        Err("failed!")
    }
}

#[tokio::main]
async fn main() {
    let result = try_join!(
        may_fail(true),
        may_fail(true)
    );

    match result {
        Ok((a, b)) => println!("Both succeeded: {}, {}", a, b),
        Err(e) => println!("One failed: {}", e),
    }
}
```

---

## Tokio Time

```rust
use tokio::time::{sleep, interval, Duration, Instant};

#[tokio::main]
async fn main() {
    // Sleep
    println!("Sleeping...");
    sleep(Duration::from_secs(1)).await;
    println!("Woke up!");

    // Interval
    let mut interval = interval(Duration::from_millis(500));
    for _ in 0..3 {
        interval.tick().await;
        println!("Tick!");
    }

    // Measure time
    let start = Instant::now();
    sleep(Duration::from_millis(100)).await;
    println!("Elapsed: {:?}", start.elapsed());
}
```

---

## Tokio Channels

### mpsc - Multiple Producer Single Consumer

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);  // buffer size 32

    // Spawn sender
    let tx1 = tx.clone();
    tokio::spawn(async move {
        tx1.send("Hello from task 1").await.unwrap();
    });

    tokio::spawn(async move {
        tx.send("Hello from task 2").await.unwrap();
    });

    // Receive
    while let Some(msg) = rx.recv().await {
        println!("Received: {}", msg);
    }
}
```

### oneshot - Single value

```rust
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        tx.send("Result!").unwrap();
    });

    let result = rx.await.unwrap();
    println!("Got: {}", result);
}
```

---

## ลองทำดู! 🎯

1. สร้าง 3 tasks ด้วย tokio::spawn และรอทุก task
2. ใช้ interval พิมพ์ทุก 1 วินาที
3. ใช้ mpsc channel ส่งข้อมูลระหว่าง tasks

---

## สรุป

| Function           | Description            |
| ------------------ | ---------------------- |
| `#[tokio::main]`   | Async main entry       |
| `tokio::spawn`     | Spawn task             |
| `join!`            | Wait for all           |
| `try_join!`        | Wait, fail fast        |
| `sleep`            | Async delay            |
| `interval`         | Periodic timer         |
| `mpsc::channel`    | Multi-producer channel |
| `oneshot::channel` | Single value channel   |

👉 ต่อไป: [Async Patterns](./04-async-patterns.md)
