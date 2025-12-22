# Async Patterns

รูปแบบการเขียน async code ที่พบบ่อย

## select! - Racing Futures

รอหลาย futures พร้อมกัน ใช้อันแรกที่เสร็จ:

```rust
use tokio::select;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    select! {
        _ = sleep(Duration::from_secs(1)) => {
            println!("1 second elapsed");
        }
        _ = sleep(Duration::from_secs(2)) => {
            println!("2 seconds elapsed");
        }
    }
    // จะ print "1 second elapsed" เพราะเสร็จก่อน
}
```

### select! กับ Return Values

```rust
use tokio::select;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = mpsc::channel::<i32>(1);
    let (tx2, mut rx2) = mpsc::channel::<String>(1);

    // Spawn senders
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        tx1.send(42).await.unwrap();
    });

    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        tx2.send("hello".to_string()).await.unwrap();
    });

    select! {
        Some(val) = rx1.recv() => println!("Got number: {}", val),
        Some(val) = rx2.recv() => println!("Got string: {}", val),
    }
    // จะได้ string เพราะ 50ms < 100ms
}
```

---

## Timeout

ให้ future มีเวลาจำกัด:

```rust
use tokio::time::{timeout, Duration};

async fn slow_function() -> String {
    tokio::time::sleep(Duration::from_secs(10)).await;
    "Done!".to_string()
}

#[tokio::main]
async fn main() {
    let result = timeout(
        Duration::from_secs(2),  // timeout หลัง 2 วินาที
        slow_function()
    ).await;

    match result {
        Ok(value) => println!("Got: {}", value),
        Err(_) => println!("Timeout! Function took too long."),
    }
}
```

### Timeout กับ select!

```rust
use tokio::select;
use tokio::time::{sleep, Duration};

async fn fetch_data() -> String {
    sleep(Duration::from_secs(5)).await;
    "Data".to_string()
}

#[tokio::main]
async fn main() {
    select! {
        result = fetch_data() => {
            println!("Got: {}", result);
        }
        _ = sleep(Duration::from_secs(2)) => {
            println!("Timeout!");
        }
    }
}
```

---

## Retry Pattern

ลองใหม่เมื่อ fail:

```rust
use tokio::time::{sleep, Duration};

async fn fallible_operation() -> Result<String, &'static str> {
    // Simulate random failure
    static mut ATTEMPT: u32 = 0;
    unsafe {
        ATTEMPT += 1;
        if ATTEMPT < 3 {
            Err("failed")
        } else {
            Ok("success!".to_string())
        }
    }
}

async fn retry<F, Fut, T, E>(f: F, max_retries: u32) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    let mut attempt = 0;
    loop {
        match f().await {
            Ok(value) => return Ok(value),
            Err(e) => {
                attempt += 1;
                if attempt >= max_retries {
                    return Err(e);
                }
                println!("Retry attempt {}/{}", attempt, max_retries);
                sleep(Duration::from_millis(100 * attempt as u64)).await;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let result = retry(|| fallible_operation(), 5).await;
    println!("Result: {:?}", result);
}
```

---

## Graceful Shutdown

ปิดโปรแกรมอย่างสะอาด:

```rust
use tokio::signal;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    // สร้าง shutdown signal
    let (shutdown_tx, _) = broadcast::channel::<()>(1);

    // Spawn workers
    for i in 0..3 {
        let mut rx = shutdown_tx.subscribe();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = rx.recv() => {
                        println!("Worker {} shutting down", i);
                        break;
                    }
                    _ = tokio::time::sleep(std::time::Duration::from_secs(1)) => {
                        println!("Worker {} working...", i);
                    }
                }
            }
        });
    }

    // รอ Ctrl+C
    signal::ctrl_c().await.unwrap();
    println!("Shutdown signal received!");

    // ส่ง shutdown
    drop(shutdown_tx);

    // รอให้ workers เสร็จ
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    println!("Goodbye!");
}
```

---

## Rate Limiting

จำกัดความถี่:

```rust
use tokio::time::{interval, Duration};
use tokio::sync::Semaphore;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // อนุญาตแค่ 3 concurrent requests
    let semaphore = Arc::new(Semaphore::new(3));

    let mut handles = vec![];

    for i in 0..10 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();

        let handle = tokio::spawn(async move {
            println!("Request {} started", i);
            tokio::time::sleep(Duration::from_millis(500)).await;
            println!("Request {} done", i);
            drop(permit);  // release permit
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
```

---

## Buffering and Batching

รวม items ก่อน process:

```rust
use tokio::sync::mpsc;
use tokio::time::{timeout, Duration};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<i32>(100);

    // Producer
    tokio::spawn(async move {
        for i in 0..20 {
            tx.send(i).await.unwrap();
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    });

    // Consumer with batching
    let batch_size = 5;
    let batch_timeout = Duration::from_millis(200);

    loop {
        let mut batch = Vec::with_capacity(batch_size);

        // Collect batch
        loop {
            let result = timeout(batch_timeout, rx.recv()).await;

            match result {
                Ok(Some(item)) => {
                    batch.push(item);
                    if batch.len() >= batch_size {
                        break;
                    }
                }
                Ok(None) => break,  // channel closed
                Err(_) => break,     // timeout
            }
        }

        if batch.is_empty() {
            break;
        }

        println!("Processing batch: {:?}", batch);
    }
}
```

---

## ลองทำดู! 🎯

1. ใช้ select! เลือกระหว่าง 2 async operations
2. สร้าง function ที่มี timeout
3. implement retry logic สำหรับ API call

---

## สรุปบทที่ 16

| Pattern           | Use Case                  |
| ----------------- | ------------------------- |
| `select!`         | Race futures              |
| `timeout`         | Limit execution time      |
| Retry             | Handle transient failures |
| Semaphore         | Rate limiting             |
| Batching          | Efficient processing      |
| Graceful shutdown | Clean exit                |

### เปรียบเทียบ

| Macro       | Behavior                 |
| ----------- | ------------------------ |
| `join!`     | รอทุกอันเสร็จ            |
| `select!`   | ใช้อันแรกที่เสร็จ        |
| `try_join!` | รอทุกอัน หยุดเมื่อ error |

👉 ต่อไป: [บทที่ 17: Unsafe Rust](../ch17-unsafe/README.md)
