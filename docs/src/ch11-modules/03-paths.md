# Paths

เข้าถึง items ใน module tree ด้วย **paths**

## 2 รูปแบบของ Path

| Type     | เริ่มจาก       | Syntax                |
| -------- | -------------- | --------------------- |
| Absolute | crate root     | `crate::module::item` |
| Relative | current module | `module::item`        |

---

## Absolute Path

เริ่มจาก crate root ด้วย `crate`:

```rust,ignore
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Adding to waitlist");
        }
    }
}

fn main() {
    // Absolute path - ชัดเจน ไม่กำกวม
    crate::front_of_house::hosting::add_to_waitlist();
}
```

---

## Relative Path

เริ่มจาก module ปัจจุบัน:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn main() {
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

---

## super Keyword

`super` = parent module (เหมือน `..` ใน filesystem)

```rust,ignore
mod parent {
    fn parent_function() {
        println!("In parent");
    }

    pub mod child {
        pub fn call_parent() {
            // super ชี้ไปที่ parent module
            super::parent_function();
        }

        pub fn call_sibling() {
            // เรียก sibling module
            super::sibling::sibling_function();
        }
    }

    pub mod sibling {
        pub fn sibling_function() {
            println!("In sibling");
        }
    }
}

fn main() {
    parent::child::call_parent();
    parent::child::call_sibling();
}
```

---

## self Keyword

`self` = current module

```rust,ignore
mod my_module {
    pub fn function_a() {
        println!("Function A");
    }

    pub fn function_b() {
        // ใช้ self เรียก function ใน module เดียวกัน
        self::function_a();

        // หรือเรียกตรงๆ ก็ได้
        function_a();
    }
}
```

### ใช้ self กับ use

```rust
mod outer {
    pub mod inner {
        pub fn func() {}
    }

    pub fn use_inner() {
        // use self เพื่อนำเข้าจาก current module
        use self::inner::func;
        func();
    }
}
```

---

## Path ใน use Statement

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }
}

// นำเข้า module
use crate::front_of_house::hosting;

// หรือนำเข้า function โดยตรง
use crate::front_of_house::hosting::add_to_waitlist;

fn main() {
    // เรียกผ่าน module
    hosting::add_to_waitlist();

    // หรือเรียกตรง
    add_to_waitlist();
}
```

---

## เลือกใช้ Absolute vs Relative?

| Situation    | ใช้         |
| ------------ | ----------- |
| Code อาจย้าย | Absolute    |
| อยู่ใกล้กัน  | Relative    |
| Re-export    | Absolute    |
| Library      | แบบไหนก็ได้ |

```rust,ignore
// ✅ Good: ถ้าย้าย main ไปที่อื่น ไม่ต้องแก้
crate::utils::helper();

// ⚠️ Might break: ถ้าย้าย function นี้ไปที่อื่น
utils::helper();
```

---

## ตัวอย่างจริง

```rust,ignore
mod database {
    pub mod connection {
        pub fn connect() {
            println!("Connecting...");
        }
    }

    pub mod queries {
        use super::connection;  // parent's sibling

        pub fn execute() {
            connection::connect();
            println!("Executing query...");
        }
    }
}

mod handlers {
    use crate::database;  // absolute path

    pub fn handle_request() {
        database::queries::execute();
    }
}

fn main() {
    handlers::handle_request();
}
```

---

## ลองทำดู! 🎯

1. สร้าง nested modules และใช้ `super`
2. ใช้ `self` ใน use statement
3. เปรียบเทียบ absolute และ relative paths

---

## สรุป

| Keyword | ความหมาย                |
| ------- | ----------------------- |
| `crate` | Root ของ crate ปัจจุบัน |
| `super` | Parent module           |
| `self`  | Current module          |
| `::`    | Path separator          |

### Path Examples

```rust,ignore
crate::module::item        // absolute
module::item               // relative
self::item                 // current module
super::item                // parent module
super::super::item         // grandparent
```

👉 ต่อไป: [แยกไฟล์](./04-separating-files.md)
