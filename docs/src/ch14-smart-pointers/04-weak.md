# Weak\<T\>

**Weak\<T\>** เป็น reference ที่ไม่นับ ใช้ป้องกัน **reference cycles** (memory leak)

## ปัญหา: Reference Cycle

เมื่อ Rc ชี้หากันเป็นวงกลม:

```text
   ┌──────────────────────────┐
   │                          │
   ▼                          │
┌──────┐    strong    ┌──────┐│
│ Node │◄────────────►│ Node ││
│  A   │              │  B   ││
└──────┘              └──────┘│
   │                          │
   └──────────────────────────┘

เมื่อ A และ B ออกจาก scope:
- A มี strong reference จาก B → ยังไม่ drop
- B มี strong reference จาก A → ยังไม่ drop
→ Memory leak! 💥
```text

---

## ทางออก: ใช้ Weak

```rust
use std::rc::{Rc, Weak};

// Weak ไม่นับ reference
// ถ้า strong count เป็น 0 → data ถูก drop
// แม้จะยังมี weak references อยู่
```

---

## Rc::downgrade และ Weak::upgrade

```rust
use std::rc::{Rc, Weak};

fn main() {
    let strong = Rc::new(5);

    // สร้าง Weak จาก Rc
    let weak: Weak<i32> = Rc::downgrade(&strong);

    println!("Strong count: {}", Rc::strong_count(&strong));  // 1
    println!("Weak count: {}", Rc::weak_count(&strong));      // 1

    // Weak ต้อง upgrade เป็น Rc ก่อนใช้
    if let Some(value) = weak.upgrade() {
        println!("Value: {}", value);  // 5
    }

    // Drop strong reference
    drop(strong);

    // Weak upgrade ไม่ได้แล้ว
    assert!(weak.upgrade().is_none());
    println!("Data is gone!");
}
```text

---

## ตัวอย่าง: Tree Structure

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,      // Weak ไป parent
    children: RefCell<Vec<Rc<Node>>>, // Strong ไป children
}

fn main() {
    // สร้าง leaf node
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf strong count: {}", Rc::strong_count(&leaf));  // 1

    // สร้าง branch node
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // ตั้ง parent ของ leaf (weak reference)
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("After linking:");
    println!("  leaf strong: {}", Rc::strong_count(&leaf));    // 2 (leaf + branch's child)
    println!("  branch strong: {}", Rc::strong_count(&branch)); // 1
    println!("  branch weak: {}", Rc::weak_count(&branch));     // 1 (leaf's parent)

    // Access parent
    if let Some(parent) = leaf.parent.borrow().upgrade() {
        println!("leaf's parent value: {}", parent.value);  // 5
    }
}
```

### ทำไมถึงไม่ leak?

```text
              branch (strong=1, weak=1)
                ▲ Weak
                │
              ┌─┘
              │
leaf ─────────┘ Strong (in children vec)

เมื่อ branch ออกจาก scope:
1. branch's strong=0 → branch ถูก drop
2. leaf's strong -1 (from branch's children)
3. leaf's strong=1 → ยังอยู่
4. leaf's parent.upgrade() = None
```text

---

## เมื่อไหร่ใช้ Weak?

| Situation         | Use                    |
| ----------------- | ---------------------- |
| Parent → Children | **Rc** (strong)        |
| Children → Parent | **Weak**               |
| Observer pattern  | **Weak** (observers)   |
| Cache             | **Weak** (cached data) |
| Breaking cycles   | **Weak**               |

---

## ตัวอย่าง: Observer Pattern

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

trait Observer {
    fn notify(&self, message: &str);
}

struct Subject {
    observers: RefCell<Vec<Weak<dyn Observer>>>,
}

impl Subject {
    fn new() -> Subject {
        Subject {
            observers: RefCell::new(vec![]),
        }
    }

    fn subscribe(&self, observer: &Rc<dyn Observer>) {
        self.observers.borrow_mut().push(Rc::downgrade(observer));
    }

    fn notify_all(&self, message: &str) {
        // Clean up dead observers and notify living ones
        self.observers.borrow_mut().retain(|weak| {
            if let Some(observer) = weak.upgrade() {
                observer.notify(message);
                true  // keep
            } else {
                false // remove dead reference
            }
        });
    }
}

struct Logger;
impl Observer for Logger {
    fn notify(&self, message: &str) {
        println!("[LOG] {}", message);
    }
}

fn main() {
    let subject = Subject::new();

    {
        let logger: Rc<dyn Observer> = Rc::new(Logger);
        subject.subscribe(&logger);

        subject.notify_all("Hello");  // [LOG] Hello
    }  // logger dropped here

    subject.notify_all("World");  // Nothing printed, observer is gone
}
```text

---

## Weak Methods

| Method                | Description                                  |
| --------------------- | -------------------------------------------- |
| `Rc::downgrade(&rc)`  | สร้าง Weak จาก Rc                            |
| `weak.upgrade()`      | `Option\<Rc\<T\>\>` - None ถ้า data ถูก drop |
| `Weak::new()`         | สร้าง Weak ว่าง (upgrade = None เสมอ)        |
| `weak.strong_count()` | จำนวน strong refs (0 ถ้า dropped)            |
| `weak.weak_count()`   | จำนวน weak refs                              |

---

## ลองทำดู! 🎯

1. สร้าง doubly-linked list ด้วย Rc + Weak
2. สร้าง observer pattern
3. ลอง drop Rc แล้วดู Weak::upgrade()

---

## สรุปบทที่ 14

| Type           | Ownership  | Count  | Use Case            |
| -------------- | ---------- | ------ | ------------------- |
| `Box\<T\>`     | Single     | -      | Heap allocation     |
| `Rc\<T\>`      | Shared     | Strong | Multiple owners     |
| `Weak\<T\>`    | Non-owning | Weak   | Break cycles        |
| `RefCell\<T\>` | Single     | -      | Interior mutability |

### Patterns

```rust
// Tree data structure
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,      // Weak up
    children: RefCell<Vec<Rc<Node>>>, // Strong down
}

// Shared mutable state
Rc<RefCell<T>>

// Break reference cycles
Weak<T>
```text

👉 ต่อไป: [บทที่ 15: Concurrency](../ch15-concurrency/README.md)

```