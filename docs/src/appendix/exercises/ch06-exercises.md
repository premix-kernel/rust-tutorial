# แบบฝึกหัด: บทที่ 6 - Structs

## แบบฝึกหัดที่ 1: สร้าง Struct

สร้าง struct `Person` ที่มี:

- `name: String`
- `age: u32`
- `email: String`

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
struct Person {
    name: String,
    age: u32,
    email: String,
}

fn main() {
    let person = Person {
        name: String::from("สมชาย"),
        age: 30,
        email: String::from("somchai@example.com"),
    };

    println!("{} อายุ {} ปี", person.name, person.age);
}
```

</details>

---

## แบบฝึกหัดที่ 2: Methods

เพิ่ม method `introduce` ให้ struct `Person` ที่ print "สวัสดี ผมชื่อ {name}"

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn introduce(&self) {
        println!("สวัสดี ผมชื่อ {}", self.name);
    }
}

fn main() {
    let person = Person {
        name: String::from("สมชาย"),
        age: 30,
    };

    person.introduce(); // สวัสดี ผมชื่อ สมชาย
}
```

</details>

---

## แบบฝึกหัดที่ 3: Associated Function

เพิ่ม associated function `new` ที่สร้าง Person ใหม่

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
}

fn main() {
    let person = Person::new(String::from("สมชาย"), 30);
    println!("{}, {} ปี", person.name, person.age);
}
```

</details>

---

## แบบฝึกหัดที่ 4: Tuple Struct

สร้าง tuple struct สำหรับ RGB color

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
struct Color(u8, u8, u8);

fn main() {
    let red = Color(255, 0, 0);
    let green = Color(0, 255, 0);

    println!("Red: RGB({}, {}, {})", red.0, red.1, red.2);
}
```

</details>

---

## แบบฝึกหัดที่ 5: Rectangle Area

สร้าง struct `Rectangle` ที่มี method:

- `area()` คำนวณพื้นที่
- `can_hold(&other)` ตรวจสอบว่าใส่ rectangle อื่นได้ไหม

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };

    println!("Area: {}", rect1.area());
    println!("Can hold rect2: {}", rect1.can_hold(&rect2));
}
```

</details>

---

👉 [บทที่ 7](./ch07-exercises.md)
