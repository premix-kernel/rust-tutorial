# แบบฝึกหัด: บทที่ 14 - Smart Pointers

## แบบฝึกหัดที่ 1: Box

ใช้ Box สร้าง recursive data structure (linked list)

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // Print list
    fn print_list(list: &List) {
        match list {
            Cons(value, next) => {
                print!("{} -> ", value);
                print_list(next);
            }
            Nil => println!("Nil"),
        }
    }

    print_list(&list); // 1 -> 2 -> 3 -> Nil
}
```text

</details>

---

## แบบฝึกหัดที่ 2: Rc

ใช้ Rc สร้าง shared ownership

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
use std::rc::Rc;

fn main() {
    let data = Rc::new(String::from("Hello"));

    println!("Count after creation: {}", Rc::strong_count(&data));

    {
        let clone1 = Rc::clone(&data);
        println!("Count after clone1: {}", Rc::strong_count(&data));

        let clone2 = Rc::clone(&data);
        println!("Count after clone2: {}", Rc::strong_count(&data));
    }

    println!("Count after block: {}", Rc::strong_count(&data));
}
```

**Output:**

```
Count after creation: 1
Count after clone1: 2
Count after clone2: 3
Count after block: 1
```text

</details>

---

## แบบฝึกหัดที่ 3: RefCell

ใช้ RefCell สำหรับ interior mutability

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);

    println!("Before: {:?}", data.borrow());

    *data.borrow_mut() += 10;

    println!("After: {:?}", data.borrow());
}
```text

</details>

---

## แบบฝึกหัดที่ 4: Rc + RefCell

รวม Rc และ RefCell เพื่อ shared mutable data

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let data = Rc::new(RefCell::new(vec![1, 2, 3]));

    let clone1 = Rc::clone(&data);
    let clone2 = Rc::clone(&data);

    // แก้ไขผ่าน clone1
    clone1.borrow_mut().push(4);

    // แก้ไขผ่าน clone2
    clone2.borrow_mut().push(5);

    // ดูผลลัพธ์
    println!("{:?}", data.borrow()); // [1, 2, 3, 4, 5]
}
```text

</details>

---

## แบบฝึกหัดที่ 5: Weak

ใช้ Weak เพื่อป้องกัน reference cycle

<details>
<summary>ดูเฉลย</summary>

```rust,ignore
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // Set parent (weak reference)
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("Leaf value: {}", leaf.value);

    // Access parent through weak reference
    if let Some(parent) = leaf.parent.borrow().upgrade() {
        println!("Parent value: {}", parent.value);
    }
}
```

</details>

---

👉 [บทที่ 15](./ch15-exercises.md)

```