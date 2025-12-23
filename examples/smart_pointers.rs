// ===========================================
// Smart Pointers - บทที่ 14
// รัน: cargo run --example smart_pointers
// ===========================================

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

fn main() {
    println!("🦀 Smart Pointers Demo\n");

    // 1. Box - Heap allocation
    println!("1️⃣ Box<T>:");
    let b = Box::new(5);
    println!("b = {}", b);

    // Recursive type with Box
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("List: ");
    print_list(&list);

    // 2. Rc - Reference Counting
    println!("\n2️⃣ Rc<T>:");
    let data = Rc::new(String::from("Hello"));
    println!("Count after creation: {}", Rc::strong_count(&data));

    {
        let clone1 = Rc::clone(&data);
        println!("Count after clone1: {}", Rc::strong_count(&data));

        let clone2 = Rc::clone(&data);
        println!("Count after clone2: {}", Rc::strong_count(&data));

        println!("Data from clone1: {}, clone2: {}", clone1, clone2);
    }

    println!("Count after block: {}", Rc::strong_count(&data));

    // 3. RefCell - Interior Mutability
    println!("\n3️⃣ RefCell<T>:");
    let value = RefCell::new(5);
    println!("Before: {:?}", value.borrow());

    *value.borrow_mut() += 10;
    println!("After: {:?}", value.borrow());

    // 4. Rc + RefCell
    println!("\n4️⃣ Rc<RefCell<T>>:");
    let shared = Rc::new(RefCell::new(vec![1, 2, 3]));

    let a = Rc::clone(&shared);
    let b = Rc::clone(&shared);

    a.borrow_mut().push(4);
    b.borrow_mut().push(5);

    println!("Shared vector: {:?}", shared.borrow());

    // 5. Weak - Prevent cycles
    println!("\n5️⃣ Weak<T>:");
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("Leaf strong count: {}", Rc::strong_count(&leaf));
    println!("Leaf weak count: {}", Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("After branch:");
        println!("  Leaf strong: {}", Rc::strong_count(&leaf));
        println!("  Branch strong: {}", Rc::strong_count(&branch));
        println!("  Branch weak: {}", Rc::weak_count(&branch));
        println!("  Branch children: {}", branch.children.borrow().len());

        // Access parent
        if let Some(parent) = leaf.parent.borrow().upgrade() {
            println!("  Leaf's parent value: {}", parent.value);
        }
    }

    println!("After branch dropped:");
    println!("  Leaf strong: {}", Rc::strong_count(&leaf));

    // Parent is gone
    if leaf.parent.borrow().upgrade().is_none() {
        println!("  Leaf's parent is gone");
    }
}

// Recursive type with Box
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn print_list(list: &List) {
    match list {
        Cons(value, next) => {
            print!("{} -> ", value);
            print_list(next);
        }
        Nil => println!("Nil"),
    }
}

// Tree structure with Weak for parent
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
