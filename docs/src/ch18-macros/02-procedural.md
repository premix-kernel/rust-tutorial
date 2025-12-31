# Procedural Macros

**Procedural Macros** แปลง Rust code เป็น code อื่น ณ compile time

## 3 ประเภท

| Type          | Syntax               | Use Case             |
| ------------- | -------------------- | -------------------- |
| Derive        | `#[derive(MyMacro)]` | Generate impl blocks |
| Attribute     | `#[my_attr]`         | Modify items         |
| Function-like | `my_macro!(...)`     | Custom syntax        |

---

## Setup

Procedural macros ต้องอยู่ใน separate crate:

```toml
# my-macro/Cargo.toml
[package]
name = "my-macro"
version = "0.1.0"

[lib]
proc-macro = true

[dependencies]
quote = "1"
syn = { version = "2", features = ["full"] }
proc-macro2 = "1"
```

---

## Derive Macro Example

สร้าง derive macro สำหรับ `HelloMacro` trait:

### 1. Define Trait (main crate)

```rust
// src/lib.rs
pub trait HelloMacro {
    fn hello_macro();
}
```

### 2. Create Derive Macro (macro crate)

```rust,ignore
// my-macro/src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Parse input
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Generate code
    let expanded = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello from {}!", stringify!(#name));
            }
        }
    };

    TokenStream::from(expanded)
}
```

### 3. Use It

```rust
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();  // "Hello from Pancakes!"
}
```

---

## Derive with Fields

อ่าน struct fields:

```rust,ignore
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(Describe)]
pub fn describe_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Get field names
    let field_names: Vec<_> = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .map(|f| f.ident.as_ref().unwrap().to_string())
                .collect(),
            _ => vec![],
        },
        _ => vec![],
    };

    let fields_str = field_names.join(", ");

    let expanded = quote! {
        impl #name {
            pub fn describe() {
                println!("{} has fields: {}", stringify!(#name), #fields_str);
            }
        }
    };

    TokenStream::from(expanded)
}
```

Usage:

```rust
#[derive(Describe)]
struct User {
    name: String,
    age: u32,
}

fn main() {
    User::describe();  // "User has fields: name, age"
}
```

---

## Attribute Macro Example

```rust,ignore
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn log_call(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let block = &input.block;
    let sig = &input.sig;

    let expanded = quote! {
        #sig {
            println!("Calling function: {}", stringify!(#name));
            let result = (|| #block)();
            println!("Function {} returned", stringify!(#name));
            result
        }
    };

    TokenStream::from(expanded)
}
```

Usage:

```rust
#[log_call]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = add(2, 3);
    // Output:
    // Calling function: add
    // Function add returned
    println!("Sum: {}", sum);
}
```

---

## Function-like Macro

```rust,ignore
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn make_greeting(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as LitStr);
    let greeting = format!("Hello, {}!", name.value());

    let expanded = quote! {
        #greeting
    };

    TokenStream::from(expanded)
}
```

Usage:

```rust
let msg = make_greeting!("World");
println!("{}", msg);  // "Hello, World!"
```

---

## Real-world Examples

หลาย crates ใช้ procedural macros:

| Crate     | Macro                               | Purpose                 |
| --------- | ----------------------------------- | ----------------------- |
| serde     | `#[derive(Serialize, Deserialize)]` | JSON/YAML serialization |
| tokio     | `#[tokio::main]`                    | Async runtime setup     |
| axum      | `#[debug_handler]`                  | Better error messages   |
| sqlx      | `#[derive(FromRow)]`                | Database mapping        |
| thiserror | `#[derive(Error)]`                  | Error types             |

---

## ลองทำดู! 🎯

1. สร้าง derive macro สำหรับ `Debug` clone
2. สร้าง attribute macro สำหรับ timing functions
3. ศึกษา syn และ quote documentation

---

## สรุป

| Concept       | Description             |
| ------------- | ----------------------- |
| `proc_macro`  | Rust's proc macro crate |
| `syn`         | Parse Rust code         |
| `quote`       | Generate Rust code      |
| `TokenStream` | Input/Output of macros  |

### Crate Structure

```text
my-project/
├── Cargo.toml
├── src/lib.rs         # Main library
└── my-macro/
    ├── Cargo.toml     # proc-macro = true
    └── src/lib.rs     # Macro implementations
```

👉 ต่อไป: [Macros ที่ใช้บ่อย](./03-common-macros.md)
