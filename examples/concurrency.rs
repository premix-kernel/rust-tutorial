// ===========================================
// Concurrency - บทที่ 15
// รัน: cargo run --example concurrency
// ===========================================

use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

fn main() {
    println!("🦀 Concurrency Demo\n");

    // 1. Basic thread
    println!("1️⃣ Basic Thread:");
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("  Spawned thread: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..=2 {
        println!("  Main thread: {}", i);
        thread::sleep(Duration::from_millis(100));
    }

    handle.join().unwrap();

    // 2. Move closure
    println!("\n2️⃣ Move Closure:");
    let data = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("  Data in thread: {:?}", data);
    });

    handle.join().unwrap();

    // 3. Return value from thread
    println!("\n3️⃣ Return from Thread:");
    let handle = thread::spawn(|| {
        let sum: i32 = (1..=100).sum();
        sum
    });

    let result = handle.join().unwrap();
    println!("  Sum 1-100 = {}", result);

    // 4. Message Passing (Channel)
    println!("\n4️⃣ Message Passing:");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec!["Hello", "from", "thread"];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for received in rx {
        println!("  Received: {}", received);
    }

    // 5. Multiple producers
    println!("\n5️⃣ Multiple Producers:");
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        tx.send("P1: Hello").unwrap();
    });

    thread::spawn(move || {
        tx2.send("P2: World").unwrap();
    });

    for _ in 0..2 {
        println!("  {}", rx.recv().unwrap());
    }

    // 6. Shared State with Mutex
    println!("\n6️⃣ Mutex:");
    let counter = Mutex::new(0);

    {
        let mut num = counter.lock().unwrap();
        *num += 1;
    }

    println!("  Counter: {:?}", counter);

    // 7. Arc + Mutex (multi-threaded)
    println!("\n7️⃣ Arc + Mutex:");
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

    println!("  Final counter: {}", *counter.lock().unwrap());

    // 8. Multiple threads with work
    println!("\n8️⃣ Parallel Work:");
    let data = Arc::new(vec![1, 2, 3, 4, 5, 6, 7, 8]);
    let mut handles = vec![];

    for i in 0..4 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let start = i * 2;
            let end = start + 2;
            let slice = &data[start..end];
            let sum: i32 = slice.iter().sum();
            println!("  Thread {}: {:?} = {}", i, slice, sum);
            sum
        });
        handles.push(handle);
    }

    let total: i32 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    println!("  Total: {}", total);
}
