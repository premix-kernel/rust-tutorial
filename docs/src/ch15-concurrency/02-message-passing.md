# Message Passing

ส่งข้อมูลระหว่าง threads ด้วย channels

## Basic Channel

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

---

## Multiple Messages

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec!["hi", "from", "thread"];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

---

## Multiple Producers

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        tx.send("from thread 1").unwrap();
    });

    thread::spawn(move || {
        tx1.send("from thread 2").unwrap();
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```text

👉 ต่อไป: [Shared State](./03-shared-state.md)

```