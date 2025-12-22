# Shared State

แชร์ข้อมูลระหว่าง threads ด้วย Mutex

## Mutex

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    } // lock released here

    println!("m = {:?}", m);
}
```

---

## Arc + Mutex

**Arc** = Atomic Reference Counting (thread-safe Rc)

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap()); // 10
}
```

👉 ต่อไป: [Sync & Send](./04-sync-send.md)
