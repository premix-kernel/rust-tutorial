# สารบัญ

[บทนำ](./introduction.md)

---

# Part 1: พื้นฐาน

- [บทที่ 1: Getting Started - เริ่มต้นกับ Rust](./ch01-getting-started/README.md)

  - [การติดตั้ง Rust](./ch01-getting-started/01-installation.md)
  - [Hello World](./ch01-getting-started/02-hello-world.md)
  - [Cargo เบื้องต้น](./ch01-getting-started/03-cargo-basics.md)

- [บทที่ 2: Variables & Data Types - ตัวแปรและชนิดข้อมูล](./ch02-variables/README.md)

  - [Mutability](./ch02-variables/01-mutability.md)
  - [ชนิดข้อมูล](./ch02-variables/02-data-types.md)
  - [Constants & Shadowing](./ch02-variables/03-constants.md)

- [บทที่ 3: Functions - ฟังก์ชัน](./ch03-functions/README.md)

  - [การสร้างฟังก์ชัน](./ch03-functions/01-defining-functions.md)
  - [Parameters](./ch03-functions/02-parameters.md)
  - [Return Values](./ch03-functions/03-return-values.md)

- [บทที่ 4: Control Flow - การควบคุมโปรแกรม](./ch04-control-flow/README.md)
  - [if/else](./ch04-control-flow/01-if-else.md)
  - [Loops](./ch04-control-flow/02-loops.md)
  - [Match เบื้องต้น](./ch04-control-flow/03-match-basics.md)

---

# Part 2: Core Concepts

- [บทที่ 5: Ownership - ระบบ Ownership](./ch05-ownership/README.md)

  - [Ownership คืออะไร](./ch05-ownership/01-what-is-ownership.md)
  - [Move & Clone](./ch05-ownership/02-move-clone.md)
  - [References & Borrowing](./ch05-ownership/03-references.md)
  - [Slices](./ch05-ownership/04-slices.md)

- [บทที่ 6: Structs - โครงสร้างข้อมูล](./ch06-structs/README.md)

  - [การสร้าง Struct](./ch06-structs/01-defining-structs.md)
  - [Methods](./ch06-structs/02-methods.md)
  - [Associated Functions](./ch06-structs/03-associated-functions.md)

- [บทที่ 7: Enums & Pattern Matching](./ch07-enums/README.md)

  - [การสร้าง Enum](./ch07-enums/01-defining-enums.md)
  - [Option<T>](./ch07-enums/02-option.md)
  - [Match Expression](./ch07-enums/03-match.md)
  - [if let & while let](./ch07-enums/04-if-let.md)

- [บทที่ 8: Collections - คอลเลกชัน](./ch08-collections/README.md)

  - [Vec<T>](./ch08-collections/01-vectors.md)
  - [String](./ch08-collections/02-strings.md)
  - [HashMap](./ch08-collections/03-hashmaps.md)

- [บทที่ 9: Error Handling - การจัดการ Error](./ch09-error-handling/README.md)
  - [panic!](./ch09-error-handling/01-panic.md)
  - [Result<T, E>](./ch09-error-handling/02-result.md)
  - [การส่งต่อ Error](./ch09-error-handling/03-propagating.md)
  - [Custom Error Types](./ch09-error-handling/04-custom-errors.md)

---

# Part 3: Advanced Concepts

- [บทที่ 10: Generics, Traits & Lifetimes](./ch10-generics-traits/README.md)

  - [Generics](./ch10-generics-traits/01-generics.md)
  - [Traits](./ch10-generics-traits/02-traits.md)
  - [Trait Bounds](./ch10-generics-traits/03-trait-bounds.md)
  - [Lifetimes](./ch10-generics-traits/04-lifetimes.md)

- [บทที่ 11: Modules & Packages](./ch11-modules/README.md)

  - [Packages & Crates](./ch11-modules/01-packages-crates.md)
  - [Modules](./ch11-modules/02-modules.md)
  - [Paths](./ch11-modules/03-paths.md)
  - [แยกไฟล์](./ch11-modules/04-separating-files.md)

- [บทที่ 12: Testing](./ch12-testing/README.md)

  - [Unit Tests](./ch12-testing/01-unit-tests.md)
  - [Integration Tests](./ch12-testing/02-integration-tests.md)
  - [จัดระเบียบ Tests](./ch12-testing/03-test-organization.md)

- [บทที่ 13: Iterators & Closures](./ch13-iterators-closures/README.md)

  - [Closures](./ch13-iterators-closures/01-closures.md)
  - [Iterators](./ch13-iterators-closures/02-iterators.md)
  - [Iterator Methods](./ch13-iterators-closures/03-iterator-methods.md)
  - [Custom Iterators](./ch13-iterators-closures/04-custom-iterators.md)

- [บทที่ 14: Smart Pointers](./ch14-smart-pointers/README.md)
  - [Box<T>](./ch14-smart-pointers/01-box.md)
  - [Rc<T>](./ch14-smart-pointers/02-rc.md)
  - [RefCell<T>](./ch14-smart-pointers/03-refcell.md)
  - [Weak<T>](./ch14-smart-pointers/04-weak.md)

---

# Part 4: Concurrency & Advanced

- [บทที่ 15: Concurrency - การทำงานพร้อมกัน](./ch15-concurrency/README.md)

  - [Threads](./ch15-concurrency/01-threads.md)
  - [Message Passing](./ch15-concurrency/02-message-passing.md)
  - [Shared State](./ch15-concurrency/03-shared-state.md)
  - [Sync & Send](./ch15-concurrency/04-sync-send.md)

- [บทที่ 16: Async/Await](./ch16-async/README.md)

  - [Async พื้นฐาน](./ch16-async/01-async-basics.md)
  - [Futures](./ch16-async/02-futures.md)
  - [Tokio Runtime](./ch16-async/03-tokio.md)
  - [Async Patterns](./ch16-async/04-async-patterns.md)

- [บทที่ 17: Unsafe Rust](./ch17-unsafe/README.md)

  - [Unsafe คืออะไร](./ch17-unsafe/01-unsafe-intro.md)
  - [Raw Pointers](./ch17-unsafe/02-raw-pointers.md)
  - [Unsafe Functions](./ch17-unsafe/03-unsafe-functions.md)
  - [Safe Abstractions](./ch17-unsafe/04-safe-abstractions.md)

- [บทที่ 18: Macros](./ch18-macros/README.md)
  - [Declarative Macros](./ch18-macros/01-declarative.md)
  - [Procedural Macros](./ch18-macros/02-procedural.md)
  - [Macros ที่ใช้บ่อย](./ch18-macros/03-common-macros.md)

---

# Part 5: Real World

- [บทที่ 19: Web Development](./ch19-web-development/README.md)

  - [Web ด้วย Rust](./ch19-web-development/01-web-intro.md)
  - [Axum พื้นฐาน](./ch19-web-development/02-axum-basics.md)
  - [สร้าง REST API](./ch19-web-development/03-api-example.md)
  - [เชื่อมต่อ Database](./ch19-web-development/04-database.md)

- [บทที่ 20: Final Project - โปรเจกต์จบ](./ch20-final-project/README.md)
  - [Project Overview](./ch20-final-project/01-project-overview.md)
  - [การออกแบบ](./ch20-final-project/02-design.md)
  - [Implementation](./ch20-final-project/03-implementation.md)
  - [สรุป](./ch20-final-project/04-review.md)

---

# ภาคผนวก

- [Appendix](./appendix/README.md)
  - [Rust Cheatsheet](./appendix/cheatsheet.md)
  - [Cargo Commands](./appendix/cargo-commands.md)
  - [Resources](./appendix/resources.md)
  - [Example Code](./appendix/examples.md)
  - [แบบฝึกหัด](./appendix/exercises/README.md)
    - [บทที่ 1](./appendix/exercises/ch01-exercises.md)
    - [บทที่ 2](./appendix/exercises/ch02-exercises.md)
    - [บทที่ 3](./appendix/exercises/ch03-exercises.md)
    - [บทที่ 4](./appendix/exercises/ch04-exercises.md)
    - [บทที่ 5](./appendix/exercises/ch05-exercises.md)
    - [บทที่ 6](./appendix/exercises/ch06-exercises.md)
    - [บทที่ 7](./appendix/exercises/ch07-exercises.md)
    - [บทที่ 8](./appendix/exercises/ch08-exercises.md)
    - [บทที่ 9](./appendix/exercises/ch09-exercises.md)
    - [บทที่ 10](./appendix/exercises/ch10-exercises.md)
    - [บทที่ 11](./appendix/exercises/ch11-exercises.md)
    - [บทที่ 12](./appendix/exercises/ch12-exercises.md)
    - [บทที่ 13](./appendix/exercises/ch13-exercises.md)
    - [บทที่ 14](./appendix/exercises/ch14-exercises.md)
    - [บทที่ 15](./appendix/exercises/ch15-exercises.md)
    - [บทที่ 16](./appendix/exercises/ch16-exercises.md)
    - [บทที่ 17](./appendix/exercises/ch17-exercises.md)
    - [บทที่ 18](./appendix/exercises/ch18-exercises.md)
    - [บทที่ 19](./appendix/exercises/ch19-exercises.md)
    - [บทที่ 20](./appendix/exercises/ch20-exercises.md)
  - [Quiz](./appendix/quiz/README.md)
    - [บทที่ 1](./appendix/quiz/ch01-quiz.md)
    - [บทที่ 2](./appendix/quiz/ch02-quiz.md)
    - [บทที่ 3](./appendix/quiz/ch03-quiz.md)
    - [บทที่ 4](./appendix/quiz/ch04-quiz.md)
    - [บทที่ 5](./appendix/quiz/ch05-quiz.md)
    - [บทที่ 6](./appendix/quiz/ch06-quiz.md)
    - [บทที่ 7](./appendix/quiz/ch07-quiz.md)
    - [บทที่ 8](./appendix/quiz/ch08-quiz.md)
    - [บทที่ 9](./appendix/quiz/ch09-quiz.md)
    - [บทที่ 10](./appendix/quiz/ch10-quiz.md)
    - [บทที่ 11](./appendix/quiz/ch11-quiz.md)
    - [บทที่ 12](./appendix/quiz/ch12-quiz.md)
    - [บทที่ 13](./appendix/quiz/ch13-quiz.md)
    - [บทที่ 14](./appendix/quiz/ch14-quiz.md)
    - [บทที่ 15](./appendix/quiz/ch15-quiz.md)
    - [บทที่ 16](./appendix/quiz/ch16-quiz.md)
    - [บทที่ 17](./appendix/quiz/ch17-quiz.md)
    - [บทที่ 18](./appendix/quiz/ch18-quiz.md)
    - [บทที่ 19](./appendix/quiz/ch19-quiz.md)
    - [บทที่ 20](./appendix/quiz/ch20-quiz.md)
