# Threads

การสร้างและจัดการ threads ใน Rust

### 🧵 Thread Visualization

```text
+-------------------------------------------------------------------+
|                    Multi-Threading in Rust                        |
+-------------------------------------------------------------------+
|                                                                   |
|   Main Thread                 Spawned Thread                      |
|   -----------                 --------------                      |
|       |                                                           |
|       | spawn -----------------------> |                          |
|       |                                |                          |
|       v                                v                          |
|   +-------+                       +-------+                       |
|   | Task1 |                       | TaskA |                       |
|   +-------+                       +-------+                       |
|       |                                |                          |
|   +-------+                       +-------+                       |
|   | Task2 |                       | TaskB |                       |
|   +-------+                       +-------+                       |
|       |                                |                          |
|       | <-------- join() -------------+                           |
|       |                                                           |
|       v                                                           |
|    Continue                                                       |
|                                                                   |
+-------------------------------------------------------------------+
```

---

## สร้าง Thread

```rust,ignore
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("hi from spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..3 {
        println!("hi from main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // รอ thread จบ
    handle.join().unwrap();
}
```

Output (อาจต่างกันแต่ละครั้ง):

```text
hi from main thread: 1
hi from spawned thread: 1
hi from main thread: 2
hi from spawned thread: 2
hi from spawned thread: 3
hi from spawned thread: 4
```

---

## JoinHandle

`thread::spawn` returns `JoinHandle<T>`:

```rust,ignore
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        // return value from thread
        42
    });

    // join() returns Result<T, E>
    let result = handle.join().unwrap();
    println!("Thread returned: {}", result);
}
```

### รอหลาย Threads

```rust,ignore
use std::thread;

fn main() {
    let mut handles = vec![];

    for i in 0..5 {
        let handle = thread::spawn(move || {
            println!("Thread {} starting", i);
            i * 2
        });
        handles.push(handle);
    }

    // รอทุก threads จบ
    let results: Vec<i32> = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect();

    println!("Results: {:?}", results);
}
```

---

## Move Closure

ใช้ `move` เพื่อย้าย ownership เข้า thread:

```rust,ignore
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // ❌ Error: closure may outlive the current function
    // let handle = thread::spawn(|| {
    //     println!("vector: {:?}", v);
    // });

    // ✅ ใช้ move
    let handle = thread::spawn(move || {
        println!("vector: {:?}", v);
    });

    // println!("{:?}", v); // ❌ v was moved

    handle.join().unwrap();
}
```

---

## Thread Builder

กำหนด thread name และ stack size:

```rust,ignore
use std::thread;

fn main() {
    let builder = thread::Builder::new()
        .name("worker".to_string())
        .stack_size(4 * 1024 * 1024); // 4MB stack

    let handle = builder.spawn(|| {
        println!("Thread name: {:?}", thread::current().name());
    }).unwrap();

    handle.join().unwrap();
}
```

---

## Thread Information

```rust,ignore
use std::thread;

fn main() {
    // Current thread info
    let current = thread::current();
    println!("Main thread name: {:?}", current.name());
    println!("Main thread id: {:?}", current.id());

    let handle = thread::spawn(|| {
        let current = thread::current();
        println!("Spawned thread name: {:?}", current.name());
        println!("Spawned thread id: {:?}", current.id());
    });

    handle.join().unwrap();

    // จำนวน CPU cores
    println!("Available parallelism: {:?}", thread::available_parallelism());
}
```

---

## Panic Handling in Threads

Panic ใน thread หนึ่งไม่กระทบ threads อื่น:

```rust,ignore
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        panic!("oops!");
    });

    // join() returns Err if thread panicked
    match handle.join() {
        Ok(_) => println!("Thread completed successfully"),
        Err(e) => println!("Thread panicked: {:?}", e),
    }

    println!("Main thread continues!");
}
```

---

## Thread Parking

หยุด thread ชั่วคราว:

```rust,ignore
use std::thread;
use std::time::Duration;

fn main() {
    let parked_thread = thread::spawn(|| {
        println!("Thread starting, will park...");
        thread::park();  // รอจนกว่าจะถูก unpark
        println!("Thread unparked!");
    });

    thread::sleep(Duration::from_secs(1));
    println!("Main thread unparking the other thread");
    parked_thread.thread().unpark();

    parked_thread.join().unwrap();
}
```

---

## Scoped Threads (Rust 1.63+)

ยืม data โดยไม่ต้อง move:

```rust,ignore
use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];

    thread::scope(|s| {
        s.spawn(|| {
            println!("numbers: {:?}", numbers);  // borrow, not move!
        });

        s.spawn(|| {
            println!("length: {}", numbers.len());
        });
    });  // threads จบเมื่อ scope จบ

    // numbers ยังใช้ได้!
    println!("After scope: {:?}", numbers);
}
```

---

## ตัวอย่างจริง: Parallel Computation

```rust,ignore
use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let chunk_size = data.len() / 2;

    let (left, right) = data.split_at(chunk_size);

    thread::scope(|s| {
        let left_sum = s.spawn(|| -> i32 {
            left.iter().sum()
        });

        let right_sum = s.spawn(|| -> i32 {
            right.iter().sum()
        });

        let total = left_sum.join().unwrap() + right_sum.join().unwrap();
        println!("Total sum: {}", total);
    });
}
```

---

## ลองทำดู! 🎯

1. สร้าง 3 threads ที่ print message แล้วรอทุก thread จบ
2. ใช้ thread::scope เพื่อ borrow data
3. จัดการ panic ใน thread ด้วย join()

### 💪 Advanced Exercises

**Exercise 1: Parallel Sum**
```rust,ignore
// เขียน function ที่คำนวณ sum ของ vector โดยแบ่งงานให้ 4 threads
fn parallel_sum(numbers: Vec<i32>) -> i32 {
    // TODO: แบ่ง vector เป็น 4 ส่วน
    // TODO: สร้าง 4 threads คำนวณ sum แต่ละส่วน  
    // TODO: รวมผลลัพธ์
    todo!()
}
```

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
use std::thread;

fn parallel_sum(numbers: Vec<i32>) -> i32 {
    let chunk_size = numbers.len() / 4;
    let chunks: Vec<Vec<i32>> = numbers
        .chunks(chunk_size)
        .map(|c| c.to_vec())
        .collect();
    
    let handles: Vec<_> = chunks
        .into_iter()
        .map(|chunk| {
            thread::spawn(move || chunk.iter().sum::<i32>())
        })
        .collect();
    
    handles.into_iter().map(|h| h.join().unwrap()).sum()
}
```

</details>

**Exercise 2: Thread-Safe Counter**
```rust,ignore
// สร้าง counter ที่หลาย threads เพิ่มค่าพร้อมกันได้
// Hint: ใช้ Arc<Mutex<i32>>
```

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
        handles.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }));
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Result: {}", *counter.lock().unwrap()); // 10
}
```

</details>

---

### 📝 ทดสอบความเข้าใจ

<details>
<summary>Q1: ทำไมต้องใช้ move กับ closure ใน thread::spawn?</summary>

**A:** เพราะ thread อาจอยู่นานกว่า scope ที่สร้างมัน ดังนั้นต้อง move ownership ของ captured variables เข้าไปใน thread เพื่อป้องกัน dangling references

</details>

<details>
<summary>Q2: thread::scope ต่างจาก thread::spawn อย่างไร?</summary>

**A:** `thread::scope` รับประกันว่า threads ทั้งหมดจะจบก่อนออกจาก scope ทำให้สามารถ borrow data จาก parent scope ได้โดยไม่ต้อง move

</details>

<details>
<summary>Q3: จะจัดการ panic ใน thread อย่างไร?</summary>

**A:** ใช้ `handle.join()` ที่ return `Result<T, Box<dyn Any>>` หาก thread panic จะได้ `Err` ที่มีข้อมูล panic แทน

</details>

---

## สรุป

| Function            | คำอธิบาย                    |
| ------------------- | --------------------------- |
| `thread::spawn`     | สร้าง thread ใหม่           |
| `handle.join()`     | รอ thread จบ                |
| `move \|\| {}`      | ย้าย ownership เข้า thread  |
| `thread::scope`     | Scoped threads (borrow ได้) |
| `thread::current()` | ข้อมูล current thread       |
| `thread::park()`    | หยุด thread ชั่วคราว        |
| `thread::Builder`   | กำหนด name/stack size       |

👉 ต่อไป: [Message Passing](./02-message-passing.md)
