# Loops - การวนซ้ำ

Rust มี 3 ประเภทของ loop:

| Loop    | ใช้เมื่อ                     |
| ------- | ---------------------------- |
| `loop`  | วนไม่รู้จบ จนกว่าจะ break    |
| `while` | วนตราบใดที่เงื่อนไขเป็น true |
| `for`   | วนตามจำนวนที่กำหนด           |

---

## loop - วนไม่รู้จบ

```rust
fn main() {
    let mut count = 0;

    loop {
        count += 1;
        println!("Count: {}", count);

        if count >= 3 {
            break; // ออกจาก loop
        }
    }
}
```

ผลลัพธ์:

```
Count: 1
Count: 2
Count: 3
```

### Return ค่าจาก loop

`loop` สามารถ return ค่าผ่าน `break`:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // return 20
        }
    };

    println!("Result: {}", result); // 20
}
```

---

## while - วนตามเงื่อนไข

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!");
}
```

ผลลัพธ์:

```
3!
2!
1!
LIFTOFF!
```

---

## for - วนตาม collection

`for` เป็น loop ที่ใช้บ่อยที่สุด:

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("Value: {}", element);
    }
}
```

### Range

```rust
fn main() {
    // 1 ถึง 4 (ไม่รวม 5)
    for number in 1..5 {
        println!("{}", number);
    }
    // 1, 2, 3, 4

    println!("---");

    // 1 ถึง 5 (รวม 5)
    for number in 1..=5 {
        println!("{}", number);
    }
    // 1, 2, 3, 4, 5
}
```

### Reverse

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");
}
```

ผลลัพธ์:

```
3!
2!
1!
LIFTOFF!
```

---

## continue - ข้าม iteration

```rust
fn main() {
    for number in 1..=10 {
        if number % 2 == 0 {
            continue; // ข้ามเลขคู่
        }
        println!("{}", number);
    }
}
```

ผลลัพธ์:

```
1
3
5
7
9
```

---

## Loop Labels

ใช้ label เพื่อ break/continue loop นอก:

```rust
fn main() {
    let mut count = 0;

    'outer: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("  remaining = {}", remaining);
            if remaining == 9 {
                break; // break inner loop
            }
            if count == 2 {
                break 'outer; // break outer loop
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {}", count);
}
```

---

## เปรียบเทียบ Loops

### เมื่อไหร่ใช้อะไร?

```rust
fn main() {
    // ใช้ for เมื่อรู้จำนวนรอบ หรือวน collection
    for i in 0..5 {
        println!("for: {}", i);
    }

    // ใช้ while เมื่อขึ้นอยู่กับเงื่อนไข
    let mut x = 5;
    while x > 0 {
        println!("while: {}", x);
        x -= 1;
    }

    // ใช้ loop เมื่อต้องการ return ค่า หรือ retry logic
    let result = loop {
        // some logic
        break 42;
    };
    println!("loop result: {}", result);
}
```

---

## ตัวอย่างจริง: FizzBuzz

```rust
fn main() {
    for n in 1..=15 {
        if n % 15 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", n);
        }
    }
}
```

---

## ตัวอย่างจริง: หาผลรวม

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let mut sum = 0;

    for n in numbers {
        sum += n;
    }

    println!("Sum: {}", sum); // 15
}
```

---

## ลองทำดู! 🎯

1. เขียน loop พิมพ์ตาราง 9
2. เขียนโปรแกรมหาตัวเลขเฉพาะตั้งแต่ 1-50
3. เขียน nested loop พิมพ์รูปสามเหลี่ยม \*

---

## สรุป

| Loop  | Syntax                  | ใช้เมื่อ         |
| ----- | ----------------------- | ---------------- |
| loop  | `loop { ... }`          | วนไม่รู้จบ       |
| while | `while cond { ... }`    | เงื่อนไข         |
| for   | `for x in iter { ... }` | collection/range |

👉 ต่อไป: [Match เบื้องต้น](./03-match-basics.md)
