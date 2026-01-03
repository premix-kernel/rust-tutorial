# RefCell\<T\>

**RefCell\<T\>** ให้ **interior mutability** - mutate ข้อมูลแม้มี immutable references

## ปัญหา: Borrowing Rules ตอน Compile

```rust,ignore
fn main() {
    let x = 5;

    // ❌ Error at compile time
    // let y = &mut x; // cannot borrow as mutable
}
```

บางครั้งต้องการ mutate แม้ interface เป็น immutable

---

## RefCell Basics

```rust,ignore
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);

    // borrow() returns Ref<T> (like &T)
    println!("Value: {}", data.borrow());

    // borrow_mut() returns RefMut<T> (like &mut T)
    *data.borrow_mut() += 1;

    println!("Value: {}", data.borrow()); // 6
}
```

---

## Compile-time vs Runtime

| Aspect      | &T / &mut T   | RefCell        |
| ----------- | ------------- | -------------- |
| Check       | Compile-time  | Runtime        |
| Error       | Compile error | Panic          |
| Flexibility | Strict        | Flexible       |
| Performance | No overhead   | Small overhead |

```rust,ignore
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);

    let borrow1 = data.borrow();
    let borrow2 = data.borrow(); // ✅ OK - multiple immutable borrows

    println!("{} {}", borrow1, borrow2);

    drop(borrow1);
    drop(borrow2);

    let mut_borrow = data.borrow_mut();
    // let another = data.borrow_mut(); // 💥 Panic at runtime!
}
```

---

## Borrowing Rules (Runtime)

เหมือนกับ compile-time rules:

1. **หลาย immutable borrows** ได้
2. **หนึ่ง mutable borrow** เท่านั้น
3. **ไม่มี mutable + immutable พร้อมกัน**

```rust,ignore
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(vec![1, 2, 3]);

    // ✅ Multiple borrows (must be dropped before mut borrow)
    {
        let a = data.borrow();
        let b = data.borrow();
        println!("{:?} {:?}", a, b);
    }

    // ✅ Single mut borrow
    {
        let mut c = data.borrow_mut();
        c.push(4);
    }

    println!("{:?}", data.borrow());
}
```

---

## try_borrow และ try_borrow_mut

ไม่ panic แต่ return Result:

```rust,ignore
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);

    let borrow = data.borrow();

    match data.try_borrow_mut() {
        Ok(mut value) => *value += 1,
        Err(_) => println!("Already borrowed!"),
    }
}
```

---

## Use Case: Mock Objects

```rust,ignore
use std::cell::RefCell;

trait Messenger {
    fn send(&self, msg: &str);
}

struct MockMessenger {
    messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            messages: RefCell::new(vec![]),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
        // &self is immutable, but we can still mutate!
        self.messages.borrow_mut().push(String::from(msg));
    }
}

fn main() {
    let messenger = MockMessenger::new();
    messenger.send("Hello");
    messenger.send("World");

    assert_eq!(messenger.messages.borrow().len(), 2);
}
```

---

## Rc + RefCell = Multiple Owners + Mutability

```rust,ignore
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // Add another child to branch
    branch.children.borrow_mut().push(Rc::new(Node {
        value: 7,
        children: RefCell::new(vec![]),
    }));

    println!("leaf: {:#?}", leaf);
    println!("branch: {:#?}", branch);
}
```

---

## Common Pattern: Shared Mutable State

```rust,ignore
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let counter = Rc::new(RefCell::new(0));

    let counter1 = Rc::clone(&counter);
    let counter2 = Rc::clone(&counter);

    *counter1.borrow_mut() += 1;
    *counter2.borrow_mut() += 1;
    *counter.borrow_mut() += 1;

    println!("Final count: {}", counter.borrow()); // 3
}
```

---

## Cell\<T\> (simpler alternative)

สำหรับ Copy types:

```rust,ignore
use std::cell::Cell;

fn main() {
    let x = Cell::new(5);

    x.set(10);           // ไม่ต้อง borrow
    let value = x.get(); // Copy ออกมา

    println!("{}", value); // 10
}
```

| Type         | Best for                           |
| ------------ | ---------------------------------- |
| Cell\<T\>    | Copy types (i32, bool, etc.)       |
| RefCell\<T\> | Non-Copy types (String, Vec, etc.) |

---

## ลองทำดู! 🎯

1. สร้าง struct ที่มี `RefCell\<Vec\<String\>\>`
2. Implement method ที่ modify Vec ผ่าน &self
3. ลอง borrow ผิด rule และดู panic

---

## สรุป

| Method             | Return                              | Panics               |
| ------------------ | ----------------------------------- | -------------------- |
| `borrow()`         | `Ref<T>`                            | If mutably borrowed  |
| `borrow_mut()`     | `RefMut<T>`                         | If any borrow exists |
| `try_borrow()`     | `Result<Ref<T>, BorrowError>`       | Never                |
| `try_borrow_mut()` | `Result<RefMut<T>, BorrowMutError>` | Never                |

### เลือกใช้

| Situation                     | Use              |
| ----------------------------- | ---------------- |
| Need mutability through &self | RefCell          |
| Copy types                    | Cell             |
| Thread-safe                   | Mutex            |
| Multiple owners + mutability  | `Rc<RefCell<T>>` |

👉 ต่อไป: [Weak\<T\>](./04-weak.md)
