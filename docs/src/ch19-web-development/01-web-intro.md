# Web ด้วย Rust

## Frameworks ยอดนิยม

| Framework | คำอธิบาย            |
| --------- | ------------------- |
| Axum      | Modern, tower-based |
| Actix-web | High performance    |
| Rocket    | Easy to use         |
| Warp      | Composable filters  |

### 🏗️ Web Application Architecture

```text
+-------------------------------------------------------------------+
|                    Rust Web Application Stack                     |
+-------------------------------------------------------------------+
|                                                                   |
|   Client (Browser/Mobile)                                         |
|         |                                                         |
|         v                                                         |
|   +---------------------------------------------------------------+
|   |                      Load Balancer                            |
|   |                    (nginx/traefik)                            |
|   +---------------------------------------------------------------+
|         |                                                         |
|         v                                                         |
|   +---------------------------------------------------------------+
|   |              Rust Web Server (Axum/Actix)                     |
|   |  +----------+  +----------+  +------------+                   |
|   |  | Routes   |  | Handlers |  | Middleware |                   |
|   |  +----------+  +----------+  +------------+                   |
|   +---------------------------------------------------------------+
|         |                                                         |
|         v                                                         |
|   +---------------------------------------------------------------+
|   |            Database (PostgreSQL/SQLite/Redis)                 |
|   +---------------------------------------------------------------+
|                                                                   |
+-------------------------------------------------------------------+
```

---

## ทำไมใช้ Rust สำหรับ Web?

| คุณสมบัติ | Rust | Node.js | Python |
|-----------|------|---------|--------|
| Performance | ⭐⭐⭐ | ⭐⭐ | ⭐ |
| Memory | Low | Medium | High |
| Concurrency | ⭐⭐⭐ | ⭐⭐ | ⭐ |
| Type Safety | ⭐⭐⭐ | ⭐ | ⭐ |

### 🚀 Deployment Options

| Option | ตัวอย่าง | เหมาะกับ |
|--------|----------|----------|
| **Docker** | `docker build -t myapp .` | Production |
| **Cloud** | AWS ECS, Google Cloud Run | Scalable |
| **Serverless** | AWS Lambda (+ cargo-lambda) | Low traffic |
| **VPS** | DigitalOcean, Linode | Simple deploy |

---

## เราจะใช้ Axum

**Axum** เป็น framework จาก Tokio team:

- ใช้ง่าย
- Async native
- Type-safe
- Extensible

👉 ต่อไป: [Axum พื้นฐาน](./02-axum-basics.md)
