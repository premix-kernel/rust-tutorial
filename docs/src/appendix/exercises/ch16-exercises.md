# แบบฝึกหัด: บทที่ 16 - Async/Await

## แบบฝึกหัดที่ 1: Async Function

เขียน async function `fetch_data` ที่ return String

<details>
<summary>ดูเฉลย</summary>

```rust
async fn fetch_data() -> String {
    // Simulate network delay
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    String::from("Data from server")
}

#[tokio::main]
async fn main() {
    let data = fetch_data().await;
    println!("{}", data);
}
```

</details>

---

## แบบฝึกหัดที่ 2: Multiple Awaits

รัน 2 async functions ตามลำดับ

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
async fn step1() -> i32 {
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    println!("Step 1 done");
    10
}

async fn step2(value: i32) -> i32 {
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    println!("Step 2 done");
    value * 2
}

#[tokio::main]
async fn main() {
    let v1 = step1().await;
    let v2 = step2(v1).await;
    println!("Result: {}", v2);
}
```

</details>

---

## แบบฝึกหัดที่ 3: tokio::join!

รัน multiple futures พร้อมกัน

<details>
<summary>ดูเฉลย</summary>

```rust
async fn task_a() -> i32 {
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    println!("Task A done");
    1
}

async fn task_b() -> i32 {
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    println!("Task B done");
    2
}

#[tokio::main]
async fn main() {
    let (a, b) = tokio::join!(task_a(), task_b());
    println!("Results: {} + {} = {}", a, b, a + b);
}
```

</details>

---

## แบบฝึกหัดที่ 4: tokio::spawn

Spawn background task

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        42
    });

    println!("Doing other work...");

    let result = handle.await.unwrap();
    println!("Background task result: {}", result);
}
```

</details>

---

## แบบฝึกหัดที่ 5: Async กับ Result

Handle errors ใน async function

<details>
<summary>ดูเฉลย</summary>

```rust
async fn fetch_user(id: i32) -> Result<String, String> {
    if id > 0 {
        Ok(format!("User {}", id))
    } else {
        Err(String::from("Invalid user ID"))
    }
}

#[tokio::main]
async fn main() {
    match fetch_user(1).await {
        Ok(user) => println!("Found: {}", user),
        Err(e) => println!("Error: {}", e),
    }

    // หรือใช้ ?
    async fn get_users() -> Result<(), String> {
        let user = fetch_user(1).await?;
        println!("{}", user);
        Ok(())
    }
}
```

</details>

---

👉 [บทที่ 17](./ch17-exercises.md)
