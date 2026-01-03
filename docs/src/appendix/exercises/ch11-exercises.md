# แบบฝึกหัด: บทที่ 11 - Modules

## แบบฝึกหัดที่ 1: Module พื้นฐาน

สร้าง module `math` ที่มี function `add` และ `multiply`

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
}

fn main() {
    println!("5 + 3 = {}", math::add(5, 3));
    println!("5 * 3 = {}", math::multiply(5, 3));
}
```

</details>

---

## แบบฝึกหัดที่ 2: use Statement

ใช้ `use` เพื่อ import function จาก module

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

use math::add;

fn main() {
    println!("5 + 3 = {}", add(5, 3));
}
```

</details>

---

## แบบฝึกหัดที่ 3: Nested Modules

สร้าง module `shapes` ที่มี submodule `circle` และ `rectangle`

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
mod shapes {
    pub mod circle {
        pub fn area(radius: f64) -> f64 {
            std::f64::consts::PI * radius * radius
        }
    }

    pub mod rectangle {
        pub fn area(width: f64, height: f64) -> f64 {
            width * height
        }
    }
}

use shapes::circle;
use shapes::rectangle;

fn main() {
    println!("Circle area: {}", circle::area(5.0));
    println!("Rectangle area: {}", rectangle::area(4.0, 3.0));
}
```

</details>

---

## แบบฝึกหัดที่ 4: pub struct

สร้าง public struct ใน module แต่บาง field เป็น private

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
mod user {
    pub struct User {
        pub username: String,
        email: String,  // private
    }

    impl User {
        pub fn new(username: String, email: String) -> Self {
            Self { username, email }
        }

        pub fn email(&self) -> &str {
            &self.email
        }
    }
}

fn main() {
    let user = user::User::new(
        String::from("john"),
        String::from("john@example.com"),
    );

    println!("Username: {}", user.username);
    println!("Email: {}", user.email());
}
```

</details>

---

## แบบฝึกหัดที่ 5: re-export

ใช้ `pub use` เพื่อ re-export items

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
mod internal {
    pub mod helpers {
        pub fn format_name(name: &str) -> String {
            format!("Hello, {}!", name)
        }
    }
}

// Re-export ให้ใช้ง่ายขึ้น
pub use internal::helpers::format_name;

fn main() {
    // ใช้ได้โดยตรง
    println!("{}", format_name("World"));
}
```

</details>

---

👉 [บทที่ 12](./ch12-exercises.md)
