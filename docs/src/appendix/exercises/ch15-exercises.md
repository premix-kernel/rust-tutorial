# แบบฝึกหัด: บทที่ 15 - Concurrency

## แบบฝึกหัดที่ 1: Spawn Thread

สร้าง thread ที่ print เลข 1-5

<details>
<summary>ดูเฉลย</summary>

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Main thread
    for i in 1..=3 {
        println!("Main: {}", i);
        thread::sleep(Duration::from_millis(100));
    }

    handle.join().unwrap();
}
```

</details>

---

## แบบฝึกหัดที่ 2: move Closure

ใช้ move closure เพื่อส่ง data ไป thread

<details>
<summary>ดูเฉลย</summary>

```rust
use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5];

    let handle = thread::spawn(move || {
        println!("Data in thread: {:?}", data);
        let sum: i32 = data.iter().sum();
        sum
    });

    let result = handle.join().unwrap();
    println!("Sum: {}", result);
}
```

</details>

---

## แบบฝึกหัดที่ 3: Channel

ใช้ channel ส่งข้อมูลระหว่าง threads

<details>
<summary>ดูเฉลย</summary>

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec!["Hello", "from", "thread"];
        for msg in messages {
            tx.send(msg).unwrap();
        }
    });

    for received in rx {
        println!("Received: {}", received);
    }
}
```

</details>

---

## แบบฝึกหัดที่ 4: Arc + Mutex

ใช้ Arc และ Mutex สำหรับ shared mutable state

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
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

    println!("Count: {}", *counter.lock().unwrap());
}
```

</details>

---

## แบบฝึกหัดที่ 5: Multiple Producers

สร้าง multiple producers ที่ส่งข้อมูลไป receiver เดียว

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    // Producer 1
    let tx1 = tx.clone();
    thread::spawn(move || {
        for i in 1..=3 {
            tx1.send(format!("P1: {}", i)).unwrap();
        }
    });

    // Producer 2
    thread::spawn(move || {
        for i in 1..=3 {
            tx.send(format!("P2: {}", i)).unwrap();
        }
    });

    // Receiver
    for msg in rx {
        println!("{}", msg);
    }
}
```

</details>

---

👉 [บทที่ 16](./ch16-exercises.md)
