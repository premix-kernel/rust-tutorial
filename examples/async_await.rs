// ===========================================
// Async/Await - บทที่ 16
// รัน: cargo run --example async_await
// ===========================================

use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("🦀 Async/Await Demo\n");

    // 1. Basic async function
    println!("1️⃣ Basic Async:");
    let result = fetch_data().await;
    println!("  Result: {}", result);

    // 2. Sequential await
    println!("\n2️⃣ Sequential Await:");
    let start = std::time::Instant::now();
    let a = slow_operation("A", 100).await;
    let b = slow_operation("B", 100).await;
    println!("  Results: {} + {} = {}", a, b, a + b);
    println!("  Time: {:?}", start.elapsed());

    // 3. Concurrent with join!
    println!("\n3️⃣ Concurrent with join!:");
    let start = std::time::Instant::now();
    let (a, b, c) = tokio::join!(
        slow_operation("A", 100),
        slow_operation("B", 100),
        slow_operation("C", 100),
    );
    println!("  Results: {} + {} + {} = {}", a, b, c, a + b + c);
    println!("  Time: {:?}", start.elapsed());

    // 4. Spawn background task
    println!("\n4️⃣ Spawn Task:");
    let handle = tokio::spawn(async {
        println!("  Background task starting...");
        sleep(Duration::from_millis(200)).await;
        println!("  Background task done!");
        42
    });

    println!("  Main task continues...");
    let result = handle.await.unwrap();
    println!("  Background result: {}", result);

    // 5. Multiple spawned tasks
    println!("\n5️⃣ Multiple Tasks:");
    let mut handles = vec![];

    for i in 1..=3 {
        let handle = tokio::spawn(async move {
            sleep(Duration::from_millis(100)).await;
            i * 10
        });
        handles.push(handle);
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await.unwrap());
    }
    println!("  Results: {:?}", results);

    // 6. select! - race
    println!("\n6️⃣ Select (Race):");
    tokio::select! {
        val = slow_operation("Fast", 50) => {
            println!("  Fast won: {}", val);
        }
        val = slow_operation("Slow", 200) => {
            println!("  Slow won: {}", val);
        }
    }

    // 7. Error handling in async
    println!("\n7️⃣ Async Error Handling:");
    match fetch_user(1).await {
        Ok(user) => println!("  Found: {}", user),
        Err(e) => println!("  Error: {}", e),
    }

    match fetch_user(-1).await {
        Ok(user) => println!("  Found: {}", user),
        Err(e) => println!("  Error: {}", e),
    }
}

async fn fetch_data() -> String {
    sleep(Duration::from_millis(100)).await;
    String::from("Data fetched!")
}

async fn slow_operation(name: &str, delay_ms: u64) -> i32 {
    println!("  {} starting...", name);
    sleep(Duration::from_millis(delay_ms)).await;
    println!("  {} done!", name);
    delay_ms as i32
}

async fn fetch_user(id: i32) -> Result<String, String> {
    sleep(Duration::from_millis(50)).await;

    if id > 0 {
        Ok(format!("User {}", id))
    } else {
        Err(String::from("Invalid user ID"))
    }
}
