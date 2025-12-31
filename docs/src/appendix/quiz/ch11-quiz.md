# Quiz: บทที่ 11 - Modules

## คำถาม 11.1

`pub` keyword ทำอะไร?

A. ทำให้ private  
B. ทำให้ public  
C. สร้าง package  
D. import module

<details>
<summary>ดูเฉลย</summary>

**B. ทำให้ public**

```rust,ignore
pub fn public_function() { }
fn private_function() { } // default private
```

</details>

---

## คำถาม 11.2

`use crate::module::function` หมายถึง?

A. Import จาก external crate  
B. Import จาก current crate  
C. Export function  
D. Delete function

<details>
<summary>ดูเฉลย</summary>

**B. Import จาก current crate**

- `crate::` = current crate root
- `super::` = parent module
- `self::` = current module

</details>

---

## คำถาม 11.3

`mod.rs` ใช้ทำอะไร?

A. Main file  
B. Entry point ของ module  
C. Test file  
D. Config file

<details>
<summary>ดูเฉลย</summary>

**B. Entry point ของ module**

```text
src/
├── main.rs
└── my_module/
    ├── mod.rs      # entry point
    └── submodule.rs
```

</details>

---

## คำถาม 11.4

`pub use` ทำอะไร?

A. Private re-export  
B. Public re-export  
C. Delete export  
D. Test export

<details>
<summary>ดูเฉลย</summary>

**B. Public re-export**

```rust,ignore
pub use internal::helper::function;
// ตอนนี้ใช้ crate::function ได้เลย
```

</details>

---

## คำถาม 11.5

Crate กับ Package ต่างกันอย่างไร?

A. ไม่ต่าง  
B. Package มีได้หลาย crates  
C. Crate ใหญ่กว่า  
D. Crate มี Cargo.toml

<details>
<summary>ดูเฉลย</summary>

**B. Package มีได้หลาย crates**

- Package = มี Cargo.toml
- Crate = compilation unit (lib หรือ bin)

</details>

---

👉 [Quiz บทที่ 12](./ch12-quiz.md)

```