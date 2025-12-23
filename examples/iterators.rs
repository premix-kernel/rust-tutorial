// ===========================================
// Iterators & Closures - บทที่ 13
// รัน: cargo run --example iterators
// ===========================================

fn main() {
    println!("🦀 Iterators & Closures Demo\n");

    // 1. Closures
    println!("1️⃣ Closures:");
    let add = |a, b| a + b;
    println!("5 + 3 = {}", add(5, 3));

    // Closure capturing environment
    let multiplier = 10;
    let multiply = |x| x * multiplier;
    println!("5 * {} = {}", multiplier, multiply(5));

    // 2. Iterator basics
    println!("\n2️⃣ Iterator basics:");
    let numbers = vec![1, 2, 3, 4, 5];

    print!("Numbers: ");
    for n in &numbers {
        print!("{} ", n);
    }
    println!();

    // 3. map
    println!("\n3️⃣ map - transform each element:");
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    // 4. filter
    println!("\n4️⃣ filter - keep matching elements:");
    let evens: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("Evens: {:?}", evens);

    // 5. Chaining
    println!("\n5️⃣ Chaining - map + filter + collect:");
    let result: Vec<i32> = numbers
        .iter()
        .filter(|x| *x % 2 == 1) // odd numbers
        .map(|x| x * 10) // multiply by 10
        .collect();
    println!("Odd * 10: {:?}", result);

    // 6. fold
    println!("\n6️⃣ fold - reduce to single value:");
    let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);
    println!("Sum: {}", sum);

    let product: i32 = numbers.iter().fold(1, |acc, x| acc * x);
    println!("Product: {}", product);

    // 7. find
    println!("\n7️⃣ find - first matching element:");
    let first_even = numbers.iter().find(|x| *x % 2 == 0);
    println!("First even: {:?}", first_even);

    // 8. enumerate
    println!("\n8️⃣ enumerate - with indices:");
    let fruits = vec!["Apple", "Banana", "Orange"];
    for (index, fruit) in fruits.iter().enumerate() {
        println!("  {}: {}", index, fruit);
    }

    // 9. zip
    println!("\n9️⃣ zip - combine two iterators:");
    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![100, 85, 90];

    let combined: Vec<_> = names.iter().zip(scores.iter()).collect();
    println!("Combined: {:?}", combined);

    // 10. take and skip
    println!("\n🔟 take and skip:");
    let first_three: Vec<_> = numbers.iter().take(3).collect();
    println!("First 3: {:?}", first_three);

    let skip_two: Vec<_> = numbers.iter().skip(2).collect();
    println!("Skip 2: {:?}", skip_two);

    // 11. any and all
    println!("\n1️⃣1️⃣ any and all:");
    let has_even = numbers.iter().any(|x| x % 2 == 0);
    println!("Has even: {}", has_even);

    let all_positive = numbers.iter().all(|x| *x > 0);
    println!("All positive: {}", all_positive);

    // 12. Custom Iterator
    println!("\n1️⃣2️⃣ Custom Iterator:");
    let counter = Counter::new(5);
    let counter_vec: Vec<u32> = counter.collect();
    println!("Counter: {:?}", counter_vec);
}

// Custom Iterator
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
